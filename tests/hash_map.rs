use tyson::HashMap;
use tyson::MemoryBlock as Block;

#[test]
fn test_hash_map() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new(&arena, 10);

    assert_eq!(map.len(), 0);
    assert!(map.insert(&1, &10).is_some());
    assert_eq!(map.len(), 1);
    assert_eq!(map.find(&1), Some(&10));
    assert_eq!(map.insert(&1, &20), Some(&20));
    assert_eq!(map.find(&1), Some(&20));
    assert!(map.contains(&1));
    assert!(!map.contains(&2));
}

#[test]
fn test_hash_map_collision() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024 * 10).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new(&arena, 10);

    for i in 0..100 {
        assert!(map.insert(&i, &(i * 10)).is_some());
    }

    assert_eq!(map.find(&1), Some(&10));
    assert_eq!(map.find(&11), Some(&110));
    assert_eq!(map.len(), 100);

    // All buckets should be filled
    assert!(map.buckets().iter().all(|bucket| bucket.is_some()));

    assert_eq!(map.remove(&1), Some(10));
    assert_eq!(map.remove(&11), Some(110));
    assert_eq!(map.len(), 98);

    for i in 0..50 {
        map.remove(&i);
    }

    for i in 0..50 {
        assert!(!map.contains(&i));
    }

    for i in 50..100 {
        assert!(map.contains(&i));
    }

    assert_eq!(map.len(), 50);

    for i in 50..100 {
        assert_eq!(map.remove(&i), Some(i * 10));
    }

    assert_eq!(map.len(), 0);
}

#[test]
fn test_hash_map_remove() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new(&arena, 10);

    assert!(map.insert(&1, &10).is_some());
    assert!(map.insert(&2, &20).is_some());
    assert_eq!(map.len(), 2);

    assert_eq!(map.remove(&1), Some(10));
    assert!(!map.contains(&1));
    assert_eq!(map.len(), 1);

    assert_eq!(map.remove(&2), Some(20));
    assert!(!map.contains(&2));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_hash_map_capacity() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new(&arena, 10);

    assert_eq!(map.len(), 0);
    for i in 0..10 {
        assert!(map.insert(&i, &(i * 10)).is_some());
    }
    assert_eq!(map.len(), 10);

    // Check that we can still find all inserted values
    for i in 0..10 {
        assert_eq!(map.find(&i), Some(&(i * 10)));
    }
}

#[test]
fn test_hash_map_empty() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new(&arena, 10);

    assert_eq!(map.len(), 0);
    assert!(!map.contains(&1));
    assert!(map.find(&1).is_none());
    assert!(map.remove(&1).is_none());
}

#[test]
fn test_hash_map_iter_and_clear() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new(&arena, 8);

    for i in 0..8 {
        let _ = map.insert(&i, &(i * 2));
    }

    // Collect via iter() and check presence of all pairs
    let mut items: Vec<(i32, i32)> = map.iter().map(|(k, v)| (*k, *v)).collect();
    items.sort_by_key(|(k, _)| *k);
    assert_eq!(items, (0..8).map(|i| (i, i * 2)).collect::<Vec<_>>());

    // Clear and validate state
    map.clear();
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
    assert!(map.iter().next().is_none());
}

#[test]
fn test_hash_map_iter_mut_updates_values() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new(&arena, 8);

    for i in 0..8 {
        let _ = map.insert(&i, &i);
    }

    // Double all values using iter_mut
    for (_k, v) in map.iter_mut() {
        *v *= 2;
    }

    for i in 0..8 {
        assert_eq!(map.find(&i), Some(&(i * 2)));
    }
}
