#[allow(unused_imports)]
use crate::fmt::f;
use crate::fmt::fwrite;
use crate::io::{PeekableSource, Sink, Source};
use crate::scan::{fread, Consumer, Whitespace};
use core::cell::Cell;
use core::iter::Iterator;
use core::ptr::NonNull;

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

#[allow(clippy::option_option)]
struct PeekableSourceInput {
    source: Input,
    _peeked: Option<Option<u8>>,
}

pub fn initialize(stdin: *mut dyn Source<Item = u8>, stdout: *mut dyn Sink) {
    unsafe {
        (*(&STDIN as *const _ as *const PeekableSourceInput))
            .source
            .0
            .set(NonNull::new(stdin));
        STDOUT.0.set(NonNull::new(stdout));
    }
}

pub fn read<C: Consumer>(c: C) -> bool {
    unsafe { fread(&mut STDIN, c) }
}

pub fn read_skip_ws<C: Consumer>(c: C) -> bool {
    read(Whitespace);
    read(c)
}

pub macro read_opt() {{
    let mut x = Default::default();
    if read_skip_ws(&mut x) {
        Some(x)
    } else {
        None
    }
}}

pub macro read {
    () => {
        {
            read_opt!().unwrap()
        }
    },
    ( $($expr:expr),* ) => {
        $(
            read_skip_ws($expr);
        )*
    }
}

pub fn write<F: FnMut(&mut Output)>(f: F) {
    unsafe {
        fwrite(&mut STDOUT, f);
    }
}

pub fn writeln<F: FnMut(&mut Output)>(f: F) {
    write(f);
    unsafe {
        Sink::write(&mut STDOUT, b'\n');
    }
}

/// Macro for printing to the standard output, i.e. alternative to
/// `print!` in `std`.
///
/// # Examples
///
/// ```
/// # use porus::prelude::*;
/// # fn main() {
/// # let mut stdout: StringBuffer = default();
/// # stdio::initialize(&mut io::Bytes::new(b""), &mut stdout);
/// writef!("Hello, world!\n");
/// assert_eq!(b"Hello, world!\n", stdout.as_ref());
/// # }
/// ```
///
/// ```
/// # use porus::prelude::*;
/// # fn main() {
/// # let mut stdout: StringBuffer = default();
/// # stdio::initialize(&mut io::Bytes::new(b""), &mut stdout);
/// writef!("{:d}\n", 123);
/// assert_eq!(b"123\n", stdout.as_ref());
/// # }
/// ```
pub macro writef($($arg:tt)*) {
    write(f!($($arg)*))
}

/// Macro for writing to the standard output, with a newline,
/// i.e. alternative to `println!` in `std`.
///
/// # Examples
///
/// ```
/// # use porus::prelude::*;
/// # let mut stdout: StringBuffer = default();
/// # stdio::initialize(&mut io::Bytes::new(b""), &mut stdout);
/// writelnf!("Hello, world!");
/// assert_eq!(b"Hello, world!\n", stdout.as_ref());
/// ```
///
/// ```
/// # use porus::prelude::*;
/// # let mut stdout: StringBuffer = default();
/// # stdio::initialize(&mut io::Bytes::new(b""), &mut stdout);
/// writelnf!("{:d}", 123);
/// assert_eq!(b"123\n", stdout.as_ref());
/// ```
pub macro writelnf($($arg:tt)*) {
    writeln(f!($($arg)*))
}
