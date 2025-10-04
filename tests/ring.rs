use tyson::RingBuffer;

#[test]
fn test_ring_buffer() {
    let mut buffer = [0; 5];
    let mut ring = RingBuffer::new(&mut buffer);

    assert!(ring.is_empty());
    assert!(!ring.is_full());
    assert_eq!(ring.len(), 0);

    for i in 1..=5 {
        ring.push(i);
        assert_eq!(ring.len(), i);
        assert!(!ring.is_empty());
        if i == 5 {
            assert!(ring.is_full());
        }
    }

    for i in 1..=5 {
        assert_eq!(ring.pop(), Some(i));
        assert_eq!(ring.len(), 5 - i);
    }

    assert!(ring.is_empty());
    assert!(!ring.is_full());
}

#[test]
fn test_ring_buffer_overflow() {
    let mut buffer = [0; 3];
    let mut ring = RingBuffer::new(&mut buffer);

    for i in 1..=5 {
        ring.push(i);
        if i > 3 {
            assert!(ring.is_full());
        }
    }

    assert_eq!(ring.len(), 3);
    assert_eq!(ring.pop(), Some(3));
    assert_eq!(ring.len(), 2);
    assert_eq!(ring.pop(), Some(4));
    assert_eq!(ring.len(), 1);
    assert_eq!(ring.pop(), Some(5));
    assert!(ring.is_empty());
}

#[test]
fn test_ring_buffer_wrap_around() {
    let mut buffer = [0; 5];
    let mut ring = RingBuffer::new(&mut buffer);

    for i in 1..=5 {
        ring.push(i);
    }

    assert!(ring.is_full());
    assert_eq!(ring.len(), 5);

    for _ in 1..=3 {
        ring.pop();
    }

    assert_eq!(ring.len(), 2);
    ring.push(6);
    assert_eq!(ring.pop(), Some(4));
    assert_eq!(ring.pop(), Some(5));
    assert_eq!(ring.pop(), Some(6));
    assert!(ring.is_empty());
}

#[test]
fn test_ring_buffer_empty_pop() {
    let mut buffer = [0; 5];
    let mut ring = RingBuffer::new(&mut buffer);

    assert!(ring.is_empty());
    assert_eq!(ring.pop(), None);
    assert_eq!(ring.len(), 0);
}

#[test]
fn test_ring_buffer_full_pop() {
    let mut buffer = [0; 3];
    let mut ring = RingBuffer::new(&mut buffer);

    for i in 1..=3 {
        ring.push(i);
    }

    assert!(ring.is_full());
    assert_eq!(ring.len(), 3);

    assert_eq!(ring.pop(), Some(1));
    assert_eq!(ring.len(), 2);
    assert!(!ring.is_full());
}

#[test]
fn test_ring_buffer_capacity() {
    let mut buffer = [0; 10];
    let ring = RingBuffer::new(&mut buffer);

    assert_eq!(ring.capacity(), 10);
    assert!(ring.is_empty());
    assert!(!ring.is_full());
}

#[test]
fn test_ring_buffer_push_pop() {
    let mut buffer = [0; 5];
    let mut ring = RingBuffer::new(&mut buffer);

    ring.push(1);
    assert_eq!(ring.pop(), Some(1));
    assert!(ring.is_empty());

    ring.push(2);
    ring.push(3);
    assert_eq!(ring.pop(), Some(2));
    assert_eq!(ring.pop(), Some(3));
    assert!(ring.is_empty());
}

#[test]
fn test_ring_buffer_overwrite() {
    let mut buffer = [0; 3];
    let mut ring = RingBuffer::new(&mut buffer);

    ring.push(1);
    ring.push(2);
    ring.push(3);
    assert!(ring.is_full());

    // Overwrite the oldest element
    ring.push(4);
    assert_eq!(ring.pop(), Some(2)); // 1 is overwritten
    assert_eq!(ring.pop(), Some(3));
    assert_eq!(ring.pop(), Some(4));
    assert!(ring.is_empty());
}

#[test]
fn test_ring_buffer_read_write_indices() {
    let mut buffer = [0; 5];
    let mut ring = RingBuffer::new(&mut buffer);

    assert_eq!(ring.read_index(), 0);
    assert_eq!(ring.write_index(), 0);

    ring.push(1);
    assert_eq!(ring.read_index(), 0);
    assert_eq!(ring.write_index(), 1);

    ring.push(2);
    assert_eq!(ring.read_index(), 0);
    assert_eq!(ring.write_index(), 2);

    ring.pop();
    assert_eq!(ring.read_index(), 1);
    assert_eq!(ring.write_index(), 2);

    ring.push(3);
    assert_eq!(ring.read_index(), 1);
    assert_eq!(ring.write_index(), 3);
}
