use std::ptr;

pub unsafe fn write_to_immut<T>(immut: &T, value: T) {
    let const_ptr = immut as *const T;
    let mut_ptr = const_ptr as *mut T;
    ptr::write(&mut *mut_ptr, value);
}

