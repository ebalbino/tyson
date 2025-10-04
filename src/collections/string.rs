use crate::{Arena, Array, List, make};

pub struct StringBuilder<'a> {
    arena: &'a Arena<'a>,
    data: List<'a, Array<u8>>,
    page_size: usize,
}

impl<'a> StringBuilder<'a> {
    pub fn new(arena: &'a Arena, capacity: usize) -> Self {
        StringBuilder {
            arena,
            page_size: capacity,
            data: List::new(arena),
        }
    }

    fn make_page(&mut self, s: &str) {
        let arena = self.arena;
        let mut page = make!(arena, u8, self.page_size)
            .map(Array::new)
            .expect("Failed to allocate memory for new page");

        page.concat(s.as_bytes());
        self.data.push_back(&page);
    }

    pub fn push_str(&mut self, s: &str) {
        match self.data.tail() {
            Some(mut tail) => {
                // SAFETY: The data is guaranteed to be valid UTF-8
                let mut tail = Some(unsafe { tail.as_mut() });

                while let Some(t) = tail {
                    if t.value.len() + s.len() <= t.value.capacity() {
                        t.value.concat(s.as_bytes());
                        return;
                    }

                    tail = t.next.map(|mut n| unsafe { n.as_mut() });
                }

                self.make_page(s);
            }
            None => {
                self.make_page(s);
            }
        }
    }

    pub fn build(self) -> &'a str {
        let arena = self.arena;
        let total_size: usize = self.data.iter().map(|page| page.len()).sum();
        let result = make!(arena, u8, total_size).expect("Failed to allocate memory for String");
        let mut result = Array::new(result);

        for page in self.data.iter() {
            result.concat(&page[..page.len()]);
        }

        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                result.as_ptr(),
                result.len(),
            ))
        }
    }
}
