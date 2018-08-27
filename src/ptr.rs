use core::ptr;

pub unsafe fn copy<T>(p: *mut T, src: usize, dst: usize, count: usize) {
    ptr::copy(p.offset(src as isize), p.offset(dst as isize), count);
}

pub unsafe fn read<T>(p: *mut T, index: usize) -> T {
    ptr::read(p.offset(index as isize))
}

pub unsafe fn write<T>(p: *mut T, index: usize, item: T) {
    ptr::write(p.offset(index as isize), item)
}

pub unsafe fn get<'a, T>(p: *mut T, index: usize) -> &'a T {
    &*p.offset(index as isize)
}

pub unsafe fn get_mut<'a, T>(p: *mut T, index: usize) -> &'a mut T {
    &mut *p.offset(index as isize)
}
