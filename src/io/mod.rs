use core::cell::Cell;
use core::iter::Iterator;
use core::ptr::NonNull;
#[allow(unused_imports)]
use porus_macros::format;

pub trait Source: Iterator<Item = u8> {}

impl<T: Iterator<Item = u8>> Source for T {}

#[cfg_attr(feature = "cargo-clippy", allow(option_option))]
pub struct PeekableSource<S: Source> {
    source: S,
    peeked: Option<Option<S::Item>>,
}

impl<S: Source> PeekableSource<S> {
    pub const fn new(s: S) -> Self {
        PeekableSource {
            source: s,
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&S::Item> {
        if self.peeked.is_none() {
            self.consume();
        }

        if let Some(ref x) = self.peeked {
            return x.as_ref();
        }

        unreachable!();
    }

    pub fn consume(&mut self) {
        self.peeked = Some(Iterator::next(&mut self.source));
    }

    pub fn eof(&mut self) -> bool {
        match self.peek() {
            None => true,
            _ => false,
        }
    }
}

pub trait Sink {
    fn write(&mut self, c: u8);
}

pub mod read;
pub mod slice;
pub use self::read::{fread, read, read_skip_ws};
pub mod write;
pub use self::write::{fwrite, write, writeln};

pub struct Input(Cell<Option<NonNull<dyn Source<Item = u8>>>>);
pub struct Output(Cell<Option<NonNull<dyn Sink>>>);

impl Iterator for Input {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        Iterator::next(unsafe { self.0.get().unwrap().as_mut() })
    }
}

impl Sink for Output {
    fn write(&mut self, c: u8) {
        Sink::write(unsafe { self.0.get().unwrap().as_mut() }, c)
    }
}

static mut STDIN: PeekableSource<Input> = PeekableSource::new(Input(Cell::new(None)));
static mut STDOUT: Output = Output(Cell::new(None));

pub fn initialize(stdin: *mut dyn Source<Item = u8>, stdout: *mut dyn Sink) {
    unsafe {
        STDIN.source.0.set(NonNull::new(stdin));
        STDOUT.0.set(NonNull::new(stdout));
    }
}

#[macro_export]
macro_rules! read_opt {
    () => {{
        let mut x = Default::default();
        if $crate::io::read_skip_ws(&mut x) {
            Some(x)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! read {
    () => (
        {
            read_opt!().unwrap()
        }
    );
    ( $($expr:expr),* ) => (
        $(
            $crate::io::read_skip_ws($expr);
        )*
    )
}

pub macro f($($arg:tt)*) {
    format!($($arg)*)
}

pub macro writef($($arg:tt)*) {
    write(&mut format!($($arg)*))
}

pub macro writelnf($($arg:tt)*) {
    writeln(&mut format!($($arg)*))
}
