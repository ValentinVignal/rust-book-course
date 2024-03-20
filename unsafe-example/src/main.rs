use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = Vec::with_capacity(4);
    for i in 0..3 {
        v.push(i);
    }
    let n = &v[0] as *const i32;
    v.push(4);
    println!("{}", unsafe { *n });
    println!("{:?}", v);
}
