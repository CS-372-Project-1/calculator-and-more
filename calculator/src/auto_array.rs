/*
    File: auto_array.rs
    Authors: Elijah Acuna and Amy Paul
    Class: CSc 372, Fall 2021
    Purpose: This file demonstrates while loops and arrays in Rust.

    Note: Rust uses the snake_case naming convention, which is followed
    throughout the program code.
*/


/*
    run: Creates a fixed-size array of 100 elements and uses a while loop
    to assign a new value to each index. Then prints out the array.
*/
pub fn run() {
    let mut i = 0;

    // Arrays in Rust are fixed-size and must be fully initialized before
    // their first use.
    let mut numbers: [usize; 100] = [0; 100];
    while i < 100 {
        numbers[i] = i;
        i += 1;
    }
    
    // The final feature of the Rust programming language that we wanted 
    // to show off is a nifty thing called debug print statements.
    // It is denoted by using {:?} in your string to print, and then
    // passing an array, vector, hashmap, or some other iterable structure.

    // Important to note: it is the only way to print an iterable other than
    // looping by index
    println!("{:?}", numbers);
}