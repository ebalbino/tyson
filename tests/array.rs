use core::ops::{Deref, DerefMut};
use tyson::Array;

#[test]
fn test_array() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    assert!(array.is_empty());
    assert_eq!(array.len(), 0);
    assert_eq!(array.capacity(), 10);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    assert!(!array.is_empty());
    assert_eq!(array.len(), 5);

    for (i, el) in array.iter().enumerate() {
        assert_eq!(el, &i);
    }
}

#[test]
fn test_array_push_pop() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    assert_eq!(array.len(), 5);

    for i in (0..5).rev() {
        assert_eq!(array.pop(), Some(&i));
    }

    assert!(array.is_empty());
}

#[test]
fn test_array_concat() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    let other = [5, 6, 7, 8, 9];
    assert_eq!(
        array.concat(&other),
        Some(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..])
    );
    assert_eq!(array.len(), 10);
}

#[test]
fn test_array_insert_remove() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    assert_eq!(array.insert(2, &99), Some(&99));
    assert_eq!(array.len(), 6);
    assert_eq!(array[2], 99);

    assert_eq!(array.remove(2), Some(99));
    assert_eq!(array.len(), 5);
}

#[test]
fn test_array_clear() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    assert!(!array.is_empty());
    array.clear();
    assert!(array.is_empty());
    assert_eq!(array.len(), 0);
}

#[test]
fn test_array_contains() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    assert!(array.contains(&3));
    assert!(!array.contains(&10));
}

#[test]
fn test_array_iter() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    let mut iter = array.iter();
    for i in 0..5 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_array_iter_mut() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    for (i, el) in array.iter_mut().enumerate() {
        *el += 1; // Increment each element
        assert_eq!(*el, i + 1);
    }

    assert_eq!(array.len(), 5);
}

#[test]
fn test_array_deref() {
    let mut buffer = [0; 10];
    let array = Array::new(&mut buffer);

    assert_eq!(array.deref(), &buffer[..]);
}

#[test]
fn test_array_deref_mut() {
    let mut buffer = [0; 10];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    let slice: &mut [u8] = array.deref_mut();
    assert_eq!(slice.len(), 10);
    assert_eq!(slice[0..5], [0, 1, 2, 3, 4]);
}

#[test]
fn test_array_as_ref() {
    let mut buffer = [0; 10];
    let array = Array::new(&mut buffer);

    assert_eq!(array.as_ref(), &buffer[..]);
}

#[test]
fn test_array_push_over_capacity() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        assert!(array.push(&i).is_some());
    }

    assert!(array.push(&5).is_none()); // Should fail as capacity is reached
}

#[test]
fn test_array_pop_empty() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    assert!(array.pop().is_none()); // Should return None when empty
}

#[test]
fn test_array_concat_over_capacity() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    let other = [5, 6]; // This will exceed the capacity
    assert!(array.concat(&other).is_none()); // Should return None as it exceeds capacity
}

#[test]
fn test_array_insert_out_of_bounds() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    assert!(array.insert(6, &99).is_none()); // Should return None as index is out of bounds
}

#[test]
fn test_array_remove_out_of_bounds() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    for i in 0..5 {
        array.push(&i).unwrap();
    }

    assert!(array.remove(5).is_none()); // Should return None as index is out of bounds
}

#[test]
fn test_array_clear_on_empty() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    assert!(array.is_empty());
    array.clear(); // Should not panic or change anything
    assert!(array.is_empty());
    assert_eq!(array.len(), 0);
}

#[test]
fn test_array_contains_on_empty() {
    let mut buffer = [0; 5];
    let array = Array::new(&mut buffer);

    assert!(!array.contains(&3)); // Should return false as array is empty
}

#[test]
fn test_array_iter_on_empty() {
    let mut buffer = [0; 5];
    let array = Array::new(&mut buffer);

    let mut iter = array.iter();
    assert_eq!(iter.next(), None); // Should return None as array is empty
}

#[test]
fn test_array_iter_mut_on_empty() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    let mut iter = array.iter_mut();
    assert_eq!(iter.next(), None); // Should return None as array is empty
}

#[test]
fn test_array_deref_on_empty() {
    let mut buffer = [0; 5];
    let array = Array::new(&mut buffer);

    assert_eq!(array.deref(), &buffer[..]); // Should return the underlying buffer
}

#[test]
fn test_array_deref_mut_on_empty() {
    let mut buffer = [0; 5];
    let mut array = Array::new(&mut buffer);

    let slice: &mut [u8] = array.deref_mut();
    assert_eq!(slice.len(), 5); // Should return the underlying buffer
}

#[test]
fn test_array_as_ref_on_empty() {
    let mut buffer = [0; 5];
    let array = Array::new(&mut buffer);

    assert_eq!(array.as_ref(), &buffer[..]); // Should return the underlying buffer
}
