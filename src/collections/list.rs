use crate::{Arena, make};
use core::ptr::NonNull;

#[derive(Debug, Clone, Copy)]
pub struct Node<T> {
    pub next: Option<NonNull<Node<T>>>,
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct List<'a, T> {
    arena: &'a Arena<'a>,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    count: usize,
}

impl<T> Node<T> {
    pub fn from_list(list: List<'_, T>) -> Option<Self>
    where
        T: Copy,
    {
        let head = list.head()?;
        Some(unsafe { *head.as_ref() })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut current = Some(self);

        core::iter::from_fn(move || match current {
            None => None,
            Some(node) => {
                current = Some(unsafe { node.next?.as_ref() });
                Some(&node.value)
            }
        })
    }
}

impl<'a, T> List<'a, T> {
    pub fn new(arena: &'a Arena) -> Self {
        List {
            arena,
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn head(&self) -> Option<NonNull<Node<T>>> {
        self.head
    }

    pub fn tail(&self) -> Option<NonNull<Node<T>>> {
        self.tail
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut current = self.head;

        core::iter::from_fn(move || match current {
            None => None,
            Some(ptr) => {
                let node = unsafe { ptr.as_ref() };
                current = node.next;
                Some(&node.value)
            }
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let mut current = self.head;

        core::iter::from_fn(move || match current {
            None => None,
            Some(mut ptr) => {
                let node = unsafe { ptr.as_mut() };
                current = node.next;
                Some(&mut node.value)
            }
        })
    }

    pub fn push_front(&mut self, value: &T) -> Option<&T>
    where
        T: Copy,
    {
        let arena = self.arena;
        make!(arena, Node<T>).map(|node| {
            node.value = *value;
            node.next = self.head;

            self.head = NonNull::new(node);
            if self.tail.is_none() {
                self.tail = self.head;
            }

            self.count += 1;
            &node.value
        })
    }

    pub fn pop_front(&mut self) -> Option<&T> {
        match self.head {
            None => None,
            Some(head_ptr) => {
                let node = unsafe { head_ptr.as_ref() };
                let value = &node.value;
                let next = node.next;

                self.head = next;

                if self.head.is_none() {
                    self.tail = None;
                }

                self.count -= 1;
                Some(value)
            }
        }
    }

    pub fn push_back(&mut self, value: &T) -> Option<&T>
    where
        T: Copy,
    {
        let arena = self.arena;
        make!(arena, Node<T>).map(|node| {
            node.value = *value;
            node.next = None;

            match self.head {
                None => {
                    self.head = NonNull::new(node);
                }
                Some(_) => {
                    if let Some(mut ptr) = self.tail {
                        unsafe {
                            let n = ptr.as_mut();
                            n.next = NonNull::new(node);
                        }
                    }
                }
            }

            self.tail = NonNull::new(node);
            self.count += 1;

            &node.value
        })
    }

    pub fn pop_back(&mut self) -> Option<&T> {
        match self.head {
            None => None,
            Some(head) => {
                let mut prev = None;
                let mut current = Some(head);

                while let Some(ptr) = current {
                    let node = unsafe { ptr.as_ref() };
                    if node.next.is_none() {
                        break;
                    }

                    prev = current;
                    current = node.next;
                }

                match prev {
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                    Some(mut ptr) => unsafe {
                        let n = ptr.as_mut();
                        n.next = None;
                        self.tail = NonNull::new(n);
                    },
                }

                self.count -= 1;
                Some(unsafe { &current.unwrap().as_ref().value })
            }
        }
    }

    pub fn peek_head(&self) -> Option<&T> {
        self.head
            .map(|ptr| unsafe { ptr.as_ref() })
            .map(|node| &node.value)
    }

    pub fn peek_tail(&self) -> Option<&T> {
        self.tail
            .map(|ptr| unsafe { ptr.as_ref() })
            .map(|node| &node.value)
    }

    pub fn to_node(self) -> Option<Node<T>>
    where
        T: Copy,
    {
        Node::from_list(self)
    }
}
