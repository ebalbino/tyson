use tyson::{MemoryBlock, make, strmake};

const TEST_CAPACITY: usize = 1024;

#[test]
fn test_block_with_capacity() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    assert!(block.buffer() != core::ptr::null_mut());
    assert_eq!(block.len(), 0);
    assert_eq!(block.capacity(), 1024);
}

#[test]
fn test_block_deref() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    assert_eq!(block[..].len(), block.capacity());
}

#[test]
fn test_block_deref_mut() {
    let mut block = MemoryBlock::with_capacity(TEST_CAPACITY);
    assert_eq!(block[..].len(), block.capacity());

    let byte = &mut block[0];
    *byte = 128;

    assert_eq!(block[0], 128);
}

#[test]
fn test_arena() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();

    assert_eq!(block.len(), 512);
    assert_eq!(block.capacity(), 1024);
    assert_eq!(arena.len(), 0);
    assert_eq!(arena.capacity(), 512);
}

#[test]
fn test_arena_deref() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();

    assert_eq!(arena[..].len(), 512);
    assert_eq!(arena.capacity(), 512);
}

#[test]
fn test_arena_deref_mut() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let mut arena = block.arena(512).unwrap();

    assert_eq!(arena[..].len(), 512);
    assert_eq!(arena.capacity(), 512);

    let byte = &mut arena[0];
    *byte = 128;

    assert_eq!(arena[0], 128);
}

#[test]
fn test_arena_allocate() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();

    assert_eq!(block.len(), 512);
    assert_eq!(block.capacity(), 1024);
    assert_eq!(arena.len(), 0);
    assert_eq!(arena.capacity(), 512);

    let _ = arena.allocate::<u32>(1);

    assert_eq!(block.len(), 512);
    assert_eq!(block.capacity(), 1024);
    assert_eq!(arena.len(), 4);
    assert_eq!(arena.capacity(), 512);
}

#[test]
fn test_block_arena_clear() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();

    assert_eq!(block.len(), 512);
    assert_eq!(block.capacity(), 1024);
    assert_eq!(arena.len(), 0);
    assert_eq!(arena.capacity(), 512);

    let _ = arena.allocate::<u32>(1);

    assert_eq!(block.len(), 512);
    assert_eq!(block.capacity(), 1024);
    assert_eq!(arena.len(), 4);
    assert_eq!(arena.capacity(), 512);

    let cleared = arena.clear();

    assert_eq!(arena.len(), 0);
    assert_eq!(arena.capacity(), 512);
    assert_eq!(cleared, 4);
}

#[test]
fn test_block_exhaust_memory() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena_one = block.arena(512);
    let arena_two = block.arena(512);
    let arena_three = block.arena(1);

    assert!(arena_one.is_some());
    assert!(arena_two.is_some());
    assert!(arena_three.is_none());
}

#[test]
fn test_arena_exhaust_memory() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let alloc_one = arena.allocate::<u32>(100);
    let alloc_two = arena.allocate::<u32>(28);
    let alloc_three = arena.allocate::<u32>(10);

    assert!(alloc_one.is_some());
    assert!(alloc_two.is_some());
    assert!(alloc_three.is_none());
}

#[test]
fn test_make() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let value = make!(arena, u32).unwrap();

    assert_eq!(*value, 0);
    *value = 42;
    assert_eq!(*value, 42);
}

#[test]
fn test_strmake() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(512).unwrap();
    let result = strmake!(arena, "Hello, {}!", "World");
    assert_eq!(result, Some("Hello, World!"));
}

#[test]
fn test_make_failure() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(1).unwrap(); // Only 1 byte available
    let value = make!(arena, u32);
    assert!(value.is_none()); // Should fail to allocate
}

#[test]
fn test_strmake_failure() {
    let block = MemoryBlock::with_capacity(TEST_CAPACITY);
    let arena = block.arena(1).unwrap(); // Only 1 byte available
    let result = strmake!(arena, "This string is too long for the arena");
    assert!(result.is_none()); // Should fail to write the string
}
