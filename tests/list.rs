use std::vec::Vec;
use tyson::List;
use tyson::MemoryBlock as Block; // bring trait for .arena()

#[test]
fn test_list_operations() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    assert!(list.is_empty());
    assert_eq!(list.len(), 0);

    list.push_front(&1);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_head(), Some(&1));
    assert_eq!(list.peek_tail(), Some(&1));

    list.push_back(&2);
    assert_eq!(list.len(), 2);
    assert_eq!(list.peek_tail(), Some(&2));

    assert_eq!(list.pop_front(), Some(&1));
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_head(), Some(&2));

    assert_eq!(list.pop_back(), Some(&2));
    assert!(list.is_empty());

    assert_eq!(list.peek_head(), None);
    assert_eq!(list.peek_tail(), None);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.len(), 0);

    assert!(list.push_back(&3).is_some());
    assert_eq!(list.head().is_some(), true);
    assert_eq!(list.len(), 1);
}

#[test]
fn test_list_iter() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    list.push_front(&1);
    list.push_back(&2);
    list.push_front(&3);

    let values: Vec<_> = list.iter().collect();
    assert_eq!(values, vec![&3, &1, &2]);

    let mut iter_mut = list.iter_mut();
    if let Some(first) = iter_mut.next() {
        *first += 10;
    }

    let values_mut: Vec<_> = iter_mut.collect();
    assert_eq!(values_mut, vec![&1, &2]);
}

#[test]
fn test_list_iter_mut() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    list.push_front(&1);
    list.push_back(&2);
    list.push_front(&3);

    let values: Vec<_> = list.iter().collect();
    assert_eq!(values, vec![&3, &1, &2]);

    let mut iter_mut = list.iter_mut();
    if let Some(first) = iter_mut.next() {
        *first += 10;
    }

    let values_mut: Vec<_> = iter_mut.collect();
    assert_eq!(values_mut, vec![&1, &2]);
}

#[test]
fn test_list_push_front() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    assert!(list.push_front(&10).is_some());
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_head(), Some(&10));
    assert_eq!(list.peek_tail(), Some(&10));

    assert!(list.push_front(&20).is_some());
    assert_eq!(list.len(), 2);
    assert_eq!(list.peek_head(), Some(&20));
    assert_eq!(list.peek_tail(), Some(&10));
}

#[test]
fn test_list_push_back() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    assert!(list.push_back(&10).is_some());
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_head(), Some(&10));
    assert_eq!(list.peek_tail(), Some(&10));

    assert!(list.push_back(&20).is_some());
    assert_eq!(list.len(), 2);
    assert_eq!(list.peek_head(), Some(&10));
    assert_eq!(list.peek_tail(), Some(&20));
}

#[test]
fn test_list_pop_front() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    list.push_front(&10);
    list.push_front(&20);
    assert_eq!(list.len(), 2);

    assert_eq!(list.pop_front(), Some(&20));
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_head(), Some(&10));

    assert_eq!(list.pop_front(), Some(&10));
    assert!(list.is_empty());
    assert_eq!(list.peek_head(), None);
}

#[test]
fn test_list_pop_back() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    list.push_back(&10);
    list.push_back(&20);
    assert_eq!(list.len(), 2);

    assert_eq!(list.pop_back(), Some(&20));
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_tail(), Some(&10));

    assert_eq!(list.pop_back(), Some(&10));
    assert!(list.is_empty());
    assert_eq!(list.peek_tail(), None);
}

#[test]
fn test_list_head() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    assert!(list.head().is_none());

    list.push_front(&10);
    assert!(list.head().is_some());
    assert_eq!(list.peek_head(), Some(&10));

    list.push_back(&20);
    assert!(list.tail().is_some());
    assert_eq!(list.peek_tail(), Some(&20));
}

#[test]
fn test_list_tail() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();
    let mut list: List<i32> = List::new(&arena);

    assert!(list.tail().is_none());

    list.push_front(&10);
    assert!(list.tail().is_some());
    assert_eq!(list.peek_tail(), Some(&10));

    list.push_back(&20);
    assert!(list.tail().is_some());
    assert_eq!(list.peek_tail(), Some(&20));
}
