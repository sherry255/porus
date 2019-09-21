use crate::io;
use crate::libc;
use core::convert::TryFrom;
use core::iter::Iterator;
use core::ptr;

pub fn read(fd: i32, buf: *mut u8, count: usize) -> Result<*mut u8, libc::Error> {
    if count > 0 {
        if let Ok(size) = TryFrom::try_from(unsafe { libc::read(fd, buf, count) }) {
            if size > 0 {
                if let Some(remain) = usize::checked_sub(count, size) {
                    return read(fd, unsafe { buf.add(size) }, remain);
                }
            }
        } else {
            return libc::get_error();
        }
    }

    Ok(buf)
}

pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<(), libc::Error> {
    if count > 0 {
        if let Ok(size) = TryFrom::try_from(unsafe { libc::write(fd, buf, count) }) {
            if let Some(remain) = usize::checked_sub(count, size) {
                return write(fd, unsafe { buf.add(size) }, remain);
            }
        } else {
            return libc::get_error();
        }
    }

    Ok(())
}

pub struct Source {
    fd: i32,
    end: *mut u8,
    current: *mut u8,
    capacity: usize,
    buffer: *mut u8,
}

impl Source {
    pub fn new(fd: i32, buffer: &mut [u8]) -> Self {
        let p = buffer.as_ptr() as *mut u8;
        let capacity = buffer.len();
        let end = unsafe { p.add(capacity) };
        Self {
            fd,
            end,
            current: end,
            capacity,
            buffer: p,
        }
    }
}

impl Iterator for Source {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if (self.current == self.end) && (self.end == unsafe { self.buffer.add(self.capacity) }) {
            self.current = self.buffer;
            self.end = read(self.fd, self.buffer, self.capacity).expect("read failed");
        }

        if self.current < self.end {
            let c = unsafe { ptr::read(self.current) };
            self.current = unsafe { self.current.add(1) };
            Some(c)
        } else {
            None
        }
    }
}

pub struct Sink {
    fd: i32,
    current: *mut u8,
    capacity: usize,
    buffer: *mut u8,
}

impl Sink {
    pub fn new(fd: i32, buffer: &mut [u8]) -> Self {
        let p = buffer.as_ptr() as *mut _;

        Self {
            fd,
            current: p,
            capacity: buffer.len(),
            buffer: p,
        }
    }
}

impl io::Sink for Sink {
    fn write(&mut self, c: u8) {
        if self.current == unsafe { self.buffer.add(self.capacity) } {
            write(self.fd, self.buffer, self.capacity).expect("write failed");
            self.current = self.buffer;
        }

        unsafe { ptr::write(self.current, c) };
        self.current = unsafe { self.current.add(1) };
    }
}

impl Drop for Sink {
    fn drop(&mut self) {
        if let Ok(size) = TryFrom::try_from(unsafe { self.current.offset_from(self.buffer) }) {
            write(self.fd, self.buffer, size).expect("write failed");
        }
    }
}
