use std::mem;

#[allow(dead_code)]
#[inline]
pub(crate) unsafe fn allocate(size: usize) -> *mut u8 {
    ptr_from_vec(Vec::with_capacity(size))
}
#[inline]
fn ptr_from_vec(mut buf: Vec<u8>) -> *mut u8 {
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr
}

#[allow(dead_code)]
#[inline]
pub(crate) unsafe fn deallocate(ptr: *mut u8, old_size: usize) {
    Vec::from_raw_parts(ptr, 0, old_size);
}
#[allow(dead_code)]
pub(crate) fn empty() -> *mut u8 {
    1 as *mut u8
}
#[allow(dead_code)]
pub(crate) unsafe fn reallocate(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    if old_size > new_size {
        let mut buf = Vec::from_raw_parts(ptr, new_size, old_size);
        buf.shrink_to_fit();

        ptr_from_vec(buf)
    } else if new_size > old_size {
        let additional = new_size - old_size;

        let mut buf = Vec::from_raw_parts(ptr, 0, old_size);
        buf.reserve_exact(additional);

        ptr_from_vec(buf)
    } else {
        ptr
    }
}
