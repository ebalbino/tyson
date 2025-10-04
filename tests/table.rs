use core::hash::Hash;
use tyson::{MemoryBlock, Table};

const TEST_CAPACITY: usize = 2048;

#[test]
fn test_table() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 10);

    table.insert(&1, &100);
    table.insert(&2, &200);
    table.insert(&3, &300);

    assert!(!table.is_empty());
    assert_eq!(table.len(), 3);
    assert_eq!(table.get(&1), Some(&100));
    assert_eq!(table.get(&2), Some(&200));
    assert_eq!(table.get(&3), Some(&300));
}

#[test]
fn test_table_insert_existing_key() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    table.insert(&1, &100);
    assert_eq!(table.get(&1), Some(&100));

    // Update existing key
    table.insert(&1, &150);
    assert_eq!(table.get(&1), Some(&150));
}

#[test]
fn test_table_contains() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    table.insert(&1, &100);
    assert!(table.contains(&1));
    assert!(!table.contains(&2));
}

#[test]
fn test_table_iter() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    table.insert(&1, &100);
    table.insert(&2, &200);

    let mut iter = table.iter();
    assert_eq!(iter.next(), Some((&1, &100)));
    assert_eq!(iter.next(), Some((&2, &200)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_table_keys_values() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    table.insert(&1, &100);
    table.insert(&2, &200);

    let keys_iter: Vec<_> = table.keys().collect();
    let values_iter: Vec<_> = table.values().collect();

    assert_eq!(keys_iter, vec![&1, &2]);
    assert_eq!(values_iter, vec![&100, &200]);
}

#[test]
fn test_table_full_capacity() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 2);

    assert!(table.insert(&1, &100).is_some());
    assert!(table.insert(&2, &200).is_some());
    assert!(table.insert(&3, &300).is_none()); // Should fail due to capacity

    assert_eq!(table.len(), 2);
    assert_eq!(table.get(&1), Some(&100));
    assert_eq!(table.get(&2), Some(&200));
}

#[test]
fn test_table_index() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    table.insert(&1, &100);
    table.insert(&2, &200);
    table.insert(&3, &300);

    assert_eq!(table.index(&1), Some(0));
    assert_eq!(table.index(&2), Some(1));
    assert_eq!(table.index(&3), Some(2));
    assert_eq!(table.index(&4), None);
}

#[test]
fn test_table_insert_with_capacity() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 5);

    for i in 0..5 {
        assert!(table.insert(&i, &(i * 10)).is_some());
    }

    assert!(table.insert(&5, &50).is_none()); // Should fail due to capacity
    assert_eq!(table.len(), 5);
}

#[test]
fn test_table_insert_with_str_keys() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    // Insert different types of keys and values
    table.insert(&"key1", &100);
    table.insert(&"key2", &200);

    assert_eq!(table.get(&"key1"), Some(&100));
    assert_eq!(table.get(&"key2"), Some(&200));
}

#[test]
fn test_table_insert_with_custom_key_type() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct CustomKey {
        id: usize,
    }

    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    table.insert(&CustomKey { id: 1 }, &100);
    table.insert(&CustomKey { id: 2 }, &200);

    assert_eq!(table.get(&CustomKey { id: 1 }), Some(&100));
    assert_eq!(table.get(&CustomKey { id: 2 }), Some(&200));
}

#[test]
fn test_table_insert_with_custom_value_type() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct CustomValue {
        data: usize,
    }

    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 10);

    table.insert(&1, &CustomValue { data: 100 });
    table.insert(&2, &CustomValue { data: 200 });

    assert_eq!(table.get(&1), Some(&CustomValue { data: 100 }));
    assert_eq!(table.get(&2), Some(&CustomValue { data: 200 }));
}

#[test]
fn test_table_insert_with_large_capacity() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY * 10);
    let arena = block.arena(TEST_CAPACITY * 10).unwrap();
    let mut table = Table::new(&arena, 100);

    for i in 0..100 {
        assert!(table.insert(&i, &(i * 10)).is_some());
    }

    assert!(table.insert(&101, &1010).is_none()); // Should fail due to capacity
    assert_eq!(table.len(), 100);
}

#[test]
fn test_table_insert_with_empty_keys_and_values() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 0);

    assert!(table.insert(&1, &100).is_none()); // Should fail due to empty capacity
    assert!(table.is_empty());
}

#[test]
fn test_table_insert_with_full_capacity() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let mut table = Table::new(&arena, 5);

    for i in 0..5 {
        assert!(table.insert(&i, &(i * 10)).is_some());
    }

    assert!(table.insert(&5, &50).is_none()); // Should fail due to capacity
    assert_eq!(table.len(), 5);
}
