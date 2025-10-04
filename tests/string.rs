use tyson::MemoryBlock as Block;
use tyson::StringBuilder;

#[test]
fn test_string_builder() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();

    let mut builder = StringBuilder::new(&arena, 512);
    builder.push_str("Hello, ");
    builder.push_str("world!");
    builder.push_str(" This is a test of the StringBuilder.");
    let s = builder.build();

    assert_eq!(s, "Hello, world! This is a test of the StringBuilder.");
}

#[test]
fn test_empty_string() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();

    let builder = StringBuilder::new(&arena, 16);
    let s = builder.build();

    assert_eq!(s, "");
}

#[test]
fn test_large_string() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024 * 10).unwrap();

    let mut builder = StringBuilder::new(&arena, 512);
    for _ in 0..100 {
        builder.push_str("This is a long string segment. ");
    }
    let s = builder.build();

    assert_eq!(s, "This is a long string segment. ".repeat(100).as_str());
}

#[test]
fn test_string_builder_with_capacity() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();

    let mut builder = StringBuilder::new(&arena, 256);
    builder.push_str("Capacity test: ");
    builder.push_str("This string should fit within the initial capacity.");
    let s = builder.build();

    assert_eq!(
        s,
        "Capacity test: This string should fit within the initial capacity."
    );
}

#[test]
fn test_string_builder_utf8() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();

    let mut builder = StringBuilder::new(&arena, 128);
    builder.push_str("Hello, 世界! ");
    builder.push_str("This is a test with UTF-8 characters.");
    let s = builder.build();

    assert_eq!(s, "Hello, 世界! This is a test with UTF-8 characters.");
}

#[test]
fn test_string_builder_empty_page() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();

    let mut builder = StringBuilder::new(&arena, 64);
    builder.push_str("");
    let s = builder.build();

    assert_eq!(s, "");
}

#[test]
fn test_string_display_and_eq() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();

    let mut builder = StringBuilder::new(&arena, 64);
    builder.push_str("abc");
    builder.push_str("def");
    let s = builder.build();

    // Display uses as_str internally
    let rendered = format!("{}", s);
    assert_eq!(rendered, "abcdef");

    // Equality between two Strings
    let mut builder2 = StringBuilder::new(&arena, 64);
    builder2.push_str("abcdef");
    let s2 = builder2.build();
    assert!(s == s2);
}

#[test]
fn test_string_partial_eq_str_and_deref() {
    let block = Block::with_capacity(1024 * 1024);
    let arena = block.arena(1024).unwrap();

    let mut builder = StringBuilder::new(&arena, 16);
    builder.push_str("hello");
    let s = builder.build();

    // Deref<Target=str>
    // Compare via &str view
    assert_eq!(s, "hello");
}
