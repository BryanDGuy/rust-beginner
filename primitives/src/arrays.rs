// Arrays are created using brackets `[]` and their size is known at compile time.
// Slices are similar to arrays but their size is not known at compile time.
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice is: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn arrays_and_slices() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements are initialized to the same value
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index .. ending_index]
    println!("borrowing a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
}