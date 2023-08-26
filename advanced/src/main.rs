#![allow(dead_code)]

use std::slice;

fn main() {
    
}

#[test]
fn const_pointer() {
    let num = 5;
    let pointer = &num as *const i32;

    unsafe {
        assert_eq!(*pointer, 5);
    }
}


#[test]
fn mut_pointer() {
    let mut num = 0;
    let pointer = &mut num as *mut i32;

    unsafe {
        *pointer += 1;
        assert_eq!(*pointer, 1);
        assert_eq!(num, 1);
    }
}

fn custom_split(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        return (
            slice::from_raw_parts_mut(ptr, mid), 
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        );
    }
}

#[test]
fn custom_split_test() {
    let mut vals = [1, 2, 3, 4, 5];
    let (a, b) = custom_split(&mut vals, 2);

    assert_eq!(a, [1, 2]);
    assert_eq!(b, [3, 4, 5]);
}