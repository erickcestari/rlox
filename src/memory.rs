use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

pub fn grow_capacity(capacity: usize) -> usize {
    if capacity < 8 {
        8
    } else {
        capacity * 2
    }
}

pub unsafe fn grow_array<T>(pointer: *mut T, old_count: usize, new_count: usize) -> *mut T {
    let old_size = mem::size_of::<T>() * old_count;
    let new_size = mem::size_of::<T>() * new_count;

    if new_size == 0 {
        dealloc(
            pointer as *mut u8,
            Layout::from_size_align_unchecked(old_size, mem::align_of::<T>()),
        );
        return ptr::null_mut();
    }

    let result = alloc(Layout::from_size_align_unchecked(
        new_size,
        mem::align_of::<T>(),
    )) as *mut T;

    if result.is_null() {
        std::process::exit(1);
    }

    ptr::copy_nonoverlapping(pointer, result, old_count.min(new_count));

    if old_size > 0 {
        dealloc(
            pointer as *mut u8,
            Layout::from_size_align_unchecked(old_size, mem::align_of::<T>()),
        );
    }

    result
}

pub unsafe fn free_array<T>(pointer: *mut T, old_count: usize) {
    let old_size = mem::size_of::<T>() * old_count;
    dealloc(
        pointer as *mut u8,
        Layout::from_size_align_unchecked(old_size, mem::align_of::<T>()),
    );
}
