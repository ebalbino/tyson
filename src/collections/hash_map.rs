use super::array::Array;
use crate::{Arena, make};
use core::fmt::Debug;
use core::hash::Hash;
use core::ptr::NonNull;
use fxhash::hash64;

#[derive(Debug, Clone, Copy)]
pub struct Bucket<K, V> {
    key: K,
    value: V,
    next: Option<NonNull<Bucket<K, V>>>,
}

#[derive(Debug)]
pub struct HashMap<'a, K, V> {
    arena: &'a Arena<'a>,
    buckets: Array<Option<Bucket<K, V>>>,
    size: usize,
}

impl<K, V> Bucket<K, V> {
    fn new(key: K, value: V) -> Self {
        Bucket {
            key,
            value,
            next: None,
        }
    }

    fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        let mut current = Some(self);

        core::iter::from_fn(move || {
            if let Some(bucket) = current {
                current = unsafe { bucket.next.map(|n| n.as_ref()) };
                return Some((&bucket.key, &bucket.value));
            }

            None
        })
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)>
    where
        K: Copy,
        V: Copy,
    {
        let mut current = NonNull::new(self);

        core::iter::from_fn(move || {
            if let Some(mut bucket) = current {
                let bucket = unsafe { bucket.as_mut() };
                current = bucket.next;
                return Some((&bucket.key, &mut bucket.value));
            }

            None
        })
    }
}

impl<'a, K, V> HashMap<'a, K, V> {
    pub fn new(arena: &'a Arena, capacity: usize) -> Self
    where
        K: Copy,
        V: Copy,
    {
        let buckets = make!(arena, Option<Bucket<K, V>>, capacity)
            .map(|b| {
                b.fill(None);
                b
            })
            .expect("Failed to allocate buckets");

        Self {
            arena,
            size: 0,
            buckets: Array::new(buckets),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    pub fn insert(&mut self, key: &K, value: &V) -> Option<&V>
    where
        K: Hash + PartialEq + Copy + Debug,
        V: Copy,
    {
        let hash = hash64(key) as usize;
        let index = hash % self.buckets.capacity();
        let bucket = &mut self.buckets[index];

        match bucket {
            None => {
                *bucket = Some(Bucket::new(*key, *value));
                self.size += 1;
                return bucket.as_mut().map(|b| &b.value);
            }
            Some(b) => {
                let mut current = Some(b);
                while let Some(bucket) = current {
                    if bucket.key == *key {
                        bucket.value = *value;
                        return Some(&bucket.value);
                    }

                    current = match bucket.next {
                        Some(mut next) => Some(unsafe { next.as_mut() }),
                        None => {
                            let arena = self.arena;
                            let new_bucket = make!(arena, Bucket<K, V>)
                                .map(|b| {
                                    *b = Bucket {
                                        key: *key,
                                        value: *value,
                                        next: None,
                                    };
                                    b
                                })
                                .expect("Failed to allocate new bucket");

                            bucket.next = NonNull::new(new_bucket);
                            self.size += 1;
                            return Some(&new_bucket.value);
                        }
                    }
                }
            }
        }

        unreachable!() // Should never reach here
    }

    pub fn find(&self, key: &K) -> Option<&V>
    where
        K: Hash + PartialEq,
    {
        let hash = hash64(key) as usize;
        let index = hash % self.buckets.capacity();
        let mut current = self.buckets[index].as_ref();

        while let Some(bucket) = current {
            if bucket.key == *key {
                return Some(&bucket.value);
            }
            current = unsafe { bucket.next.map(|n| n.as_ref()) };
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Hash + PartialEq + Copy + Debug,
        V: Copy + Debug,
    {
        let hash = hash64(key) as usize;
        let index = hash % self.buckets.capacity();
        let mut current = &self.buckets[index].as_ref().map(|b| NonNull::from(b));
        let mut prev: Option<NonNull<Bucket<K, V>>> = None;

        while let &Some(mut bucket) = current {
            let bucket = unsafe { bucket.as_mut() };
            if bucket.key == *key {
                let value = bucket.value;
                if let Some(mut prev_bucket) = prev {
                    unsafe { prev_bucket.as_mut().next = bucket.next };
                } else {
                    unsafe {
                        self.buckets[index] = bucket.next.map(|n| *n.as_ref());
                    }
                }
                self.size -= 1;
                return Some(value);
            }
            prev = NonNull::new(bucket);
            current = &bucket.next
        }

        None
    }

    pub fn contains(&self, key: &K) -> bool
    where
        K: Hash + PartialEq,
    {
        self.find(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)>
    where
        K: Hash + PartialEq + Copy,
        V: Copy,
    {
        self.buckets[..]
            .iter()
            .filter_map(|bucket| bucket.as_ref().map(|b| b.iter()))
            .flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)>
    where
        K: Hash + PartialEq + Copy,
        V: Copy,
    {
        self.buckets[..]
            .iter_mut()
            .filter_map(|bucket| bucket.as_mut().map(|b| b.iter_mut()))
            .flatten()
    }

    pub fn clear(&mut self)
    where
        K: Hash + PartialEq + Copy,
        V: Copy,
    {
        self.buckets[..].fill(None);
        self.size = 0;
    }

    pub fn buckets(&self) -> &[Option<Bucket<K, V>>] {
        &self.buckets[..]
    }
}
