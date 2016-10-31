// Arrays and Slices

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("fist element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed size array uses type signature `[T; size]`
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize all elements to a value
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as Slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing yields a panic
    // println!("{}", xs[5]);
}
