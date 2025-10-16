#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod alloc;
mod collections;
pub mod env;
pub mod parser;
pub mod print;
mod tokenizer;

pub use alloc::*;
pub use collections::*;

#[macro_export]
macro_rules! make {
    ($arena:ident, $type:ty) => {{
        let arena = &$arena;
        arena
            .allocate::<$type>(1)
            .map(|mut ptr| unsafe { ptr.as_mut() })
    }};
    ($arena:ident, $type:ty, $len:expr) => {{
        let arena = &$arena;
        let len = $len;
        arena
            .allocate::<$type>(len)
            .map(|ptr| unsafe { core::slice::from_raw_parts_mut(ptr.as_ptr(), len) })
    }};
}

#[macro_export]
macro_rules! strmake {
    ($arena:ident, $($arg:tt)*) => {{
        let arena = &$arena;
        let mut sink = $crate::StrSink::new(unsafe { arena.remaining() });
        let result = core::fmt::write(&mut sink, format_args!($($arg)*));

        match result {
            Ok(_) => {
                arena.seek(arena.len() + sink.used() + 1);
                sink.as_str()
            },
            Err(_) => None
        }
    }};
}
