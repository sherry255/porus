use crate::io;
use crate::libc;
use core::iter::Iterator;
use core::ptr;

pub fn read(fd: i32, buf: *mut u8, count: usize) -> Result<usize, libc::Error> {
    let mut length = 0;
    let mut ptr = buf;

    while length < count {
        let size = unsafe { libc::read(fd, ptr, count - length) };

        if size < 0 {
            return libc::get_error();
        }
        if size == 0 {
            break;
        }

        #[allow(clippy::cast_sign_loss)]
        {
            length += size as usize;
        }

        unsafe {
            ptr = ptr.offset(size);
        }
    }

    Ok(length)
}

pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<(), libc::Error> {
    let mut written = 0;
    let mut ptr = buf;
    while written < count {
        let size = unsafe { libc::write(fd, ptr, count - written) };

        if size < 0 {
            return libc::get_error();
        }

        #[allow(clippy::cast_sign_loss)]
        {
            written += size as usize;
        }

        unsafe {
            ptr = ptr.offset(size);
        }
    }

    Ok(())
}

#[cfg(debug_assertions)]
fn unwrap<T, E: core::fmt::Debug>(result: Result<T,E>) -> T {
    result.unwrap()
}

#[cfg(not(debug_assertions))]
fn unwrap<T, E: core::fmt::Debug>(result: Result<T,E>) -> T {
    match result {
        Ok(v) => v,
        Err(_) => panic!(),
    }
}

pub struct Source {
    fd: i32,
    size: usize,
    offset: usize,
    capacity: usize,
    buffer: *mut u8,
}

impl Source {
    pub fn new(fd: i32, buffer: &mut [u8]) -> Self {
        let size = buffer.len();
        Self {
            fd,
            size,
            offset: size,
            capacity: size,
            buffer: buffer.as_ptr() as *mut _,
        }
    }
}

impl Iterator for Source {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if (self.offset == self.size) && (self.size == self.capacity) {
            self.offset = 0;
            self.size = unwrap(read(self.fd, self.buffer, self.capacity));
        }

        if self.offset < self.size {
            let c = unsafe { ptr::read(self.buffer.add(self.offset)) };
            self.offset += 1;
            Some(c)
        } else {
            None
        }
    }
}

pub struct Sink {
    fd: i32,
    offset: usize,
    capacity: usize,
    buffer: *mut u8,
}

impl Sink {
    pub fn new(fd: i32, buffer: &mut [u8]) -> Self {
        Self {
            fd,
            offset: 0,
            capacity: buffer.len(),
            buffer: buffer.as_ptr() as *mut _,
        }
    }
}

impl io::Sink for Sink {
    fn write(&mut self, c: u8) {
        if self.offset == self.capacity {
            unwrap(write(self.fd, self.buffer, self.capacity));
            self.offset = 0;
        }

        unsafe { ptr::write(self.buffer.add(self.offset), c) };
        self.offset += 1;
    }
}

impl Drop for Sink {
    fn drop(&mut self) {
        unwrap(write(self.fd, self.buffer, self.offset));
    }
}
