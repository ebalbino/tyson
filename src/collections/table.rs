use crate::{Arena, Array, make};
use core::hash::Hash;
use fxhash::hash64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key<K> {
    pub hash: u64,
    pub value: K,
}

#[derive(Debug, Clone, Copy)]
pub struct Table<K, V> {
    keys: Array<Key<K>>,
    values: Array<V>,
}

impl<K, V> Table<K, V> {
    pub fn new(arena: &Arena, capacity: usize) -> Self {
        let keys = make!(arena, Key<K>, capacity).expect("Failed to allocate memory for keys");
        let values = make!(arena, V, capacity).expect("Failed to allocate memory for values");

        Table {
            keys: Array::new(keys),
            values: Array::new(values),
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.keys.capacity()
    }

    pub fn insert(&mut self, key: &K, value: &V) -> Option<&V>
    where
        K: Copy + Hash + PartialEq<K>,
        V: Copy,
    {
        let hash = hash64(key);

        for i in 0..self.keys.len() {
            if self.keys[i].hash == hash && self.keys[i].value == *key {
                self.values[i] = *value;
                return Some(&self.values[i]);
            }
        }

        if self.keys.len() < self.keys.capacity() {
            self.keys.push(&Key { hash, value: *key });
            self.values.push(value);

            Some(&self.values[self.values.len() - 1])
        } else {
            None
        }
    }

    pub fn index(&self, key: &K) -> Option<usize>
    where
        K: Hash + PartialEq<K>,
    {
        let hash = hash64(key);
        (0..self.keys.len()).find(|&i| self.keys[i].hash == hash && self.keys[i].value == *key)
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Hash + PartialEq<K>,
    {
        self.index(key).and_then(|i| self.values.get(i))
    }

    pub fn contains(&self, key: &K) -> bool
    where
        K: Hash + PartialEq<K>,
    {
        self.index(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys
            .iter()
            .zip(self.values.iter())
            .map(|(k, v)| (&k.value, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.keys
            .iter()
            .zip(self.values.iter_mut())
            .map(|(k, v)| (&k.value, v))
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter().map(|k| &k.value)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }
}
