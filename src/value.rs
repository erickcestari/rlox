use crate::memory::{grow_array, grow_capacity};
use std::ptr;

type Value = f64;

struct ValueArray {
    capacity: usize,
    count: usize,
    values: *mut Value,
}

fn init_value_array(array: &mut ValueArray) {
    array.values = ptr::null_mut();
    array.capacity = 0;
    array.count = 0;
}

fn write_value_array(array: &mut ValueArray, value: Value) {
    if array.capacity < array.count + 1 {
        let old_capacity = array.capacity;
        array.capacity = grow_capacity(old_capacity);
        unsafe {
            array.values = grow_array(array.values, old_capacity, array.capacity);
        }
    }
    unsafe {
        *array.values.add(array.count) = value;
    }
    array.count += 1;
}
