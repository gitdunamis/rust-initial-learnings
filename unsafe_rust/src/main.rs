use std::slice;

static mut COUNTER: usize = 0;

fn main() {
    let a = 2;
    let n: *const i32 = &a as *const i32;
    
    unsafe {
        println!("Unsafe n : {}", *n);
    }
    
    let v:&mut [i32] = &mut [1,2,4,5];
    let (first, second) = split_at(v,  2);
    
    println!("first: {first:?}\n second: {second:?}");
    
    let c_abs = unsafe { abs(-24) };
    
    println!("c abs: {c_abs:?}");
    
    unsafe {
        COUNTER += 1;
        println!("New COUNTER: {COUNTER}")
    };
}


fn split_at(arr: &mut [i32], index: usize) -> (&mut [i32], &mut [i32]) {
    assert!(index <= arr.len());
    
    let arr_pointer = arr.as_mut_ptr();
    
    unsafe {
        (slice::from_raw_parts_mut(arr_pointer, index),
        slice::from_raw_parts_mut(arr_pointer.add(index), arr.len() - index))
    }
}

extern "C" {
   fn abs(i: i32) -> i32; 
}

#[no_mangle]
pub extern fn from_rust() -> *const u8 {
    "this is from rust".as_ptr()
}

unsafe trait Unsafe { }

unsafe impl Unsafe for i32 {}