use std::io;

pub fn run() {
        // group all the tuples into an immutable single tuple of tuples
        let mut num_types_1 = (("u8", u8::MAX), ("u16", u16::MAX), ("u32", u32::MAX), ("u64", u64::MAX), ("u128", u128::MAX), 
        ("ui8", i8::MAX), ("i16", i16::MAX), ("i32", i32::MAX), ("i64", i64::MAX), ("i128", i128::MAX));
let mut num_types_2 = (("u8", u8::MAX), ("u16", u16::MAX), ("u32", u32::MAX), ("u64", u64::MAX), ("u128", u128::MAX), 
        ("ui8", i8::MAX), ("i16", i16::MAX), ("i32", i32::MAX), ("i64", i64::MAX), ("i128", i128::MAX));
let plus = "+";
let minus = "-";
let div = "/";
let mult = "*";

// we need to intiialize as u8 values because these fit into any other numeric types
// however this is not always true the other way around
let mut num_1: u8 = 0;
let mut num_2: u8 = 0;

// intro prompt
println!("Welcome to Bitmath!\n");
println!("Please choose your first number datatype:");

/* im so sorry, i know this is hideous but Rust doesn't support indexing tuples in a loop.
you can try it if you want, it would be:

for i in 0..10 {
println!("\t{} with max value of {}", num_types_1.i.0, num_types_1.i.2);
}

*/

// list datatypes for num1
println!("\t{} with max value of {}", num_types_1.0.0, num_types_1.0.1);
println!("\t{} with max value of {}", num_types_1.1.0, num_types_1.1.1);
println!("\t{} with max value of {}", num_types_1.2.0, num_types_1.2.1);
println!("\t{} with max value of {}", num_types_1.3.0, num_types_1.3.1);
println!("\t{} with max value of {}", num_types_1.4.0, num_types_1.4.1);
println!("\t{} with max value of {}", num_types_1.5.0, num_types_1.5.1);
println!("\t{} with max value of {}", num_types_1.6.0, num_types_1.6.1);
println!("\t{} with max value of {}", num_types_1.7.0, num_types_1.7.1);
println!("\t{} with max value of {}", num_types_1.8.0, num_types_1.8.1);
println!("\t{} with max value of {}", num_types_1.9.0, num_types_1.9.1);
print!("Please enter the numeric type of your first number\nSelect signed if you want it to be negative: ");

let mut datatype_1 = String::new();
io::stdin().read_line(&mut datatype_1).expect("Error: Please input a numeric datatype");

// TODO: if for all types
let value_1 = String::new();
io::stdin().read_line(&mut datatype_1).expect("Error: Please input a numeric value");

if datatype_1 == "u8" {
    num_1 = value_1.parse().unwrap();
} else if datatype_1 == "u16" {
    let num_1 = value_1.parse().unwrap() as u16;
} else if datatype_1 == "u32" {
    let num_1 = value_1.parse().unwrap() as u32;
} else if datatype_1 == "u64" {
    let num_1 = value_1.parse().unwrap() as u64;
} else if datatype_1 == "u128" {
    let num_1 = value_1.parse().unwrap() as u128;
} else if datatype_1 == "i8" {
    let num_1 = value_1.parse().unwrap() as i8;
} else if datatype_1 == "i16" {
    let num_1 = value_1.parse().unwrap() as i16;
} else if datatype_1 == "i32" {
    let num_1 = value_1.parse().unwrap() as i32;
} else if datatype_1 == "i64" {
    let num_1 = value_1.parse().unwrap() as i64;
} else if datatype_1 == "i128" {
    let num_1 = value_1.parse().unwrap() as i128;
}

let mut operator = String::new();
io::stdin().read_line(&mut operator).expect("Error: Please input a math operator");    

// list datatypes for num2
println!("\t{} with max value of {}", num_types_2.0.0, num_types_2.0.1);
println!("\t{} with max value of {}", num_types_2.1.0, num_types_2.1.1);
println!("\t{} with max value of {}", num_types_2.2.0, num_types_2.2.1);
println!("\t{} with max value of {}", num_types_2.3.0, num_types_2.3.1);
println!("\t{} with max value of {}", num_types_2.4.0, num_types_2.4.1);
println!("\t{} with max value of {}", num_types_2.5.0, num_types_2.5.1);
println!("\t{} with max value of {}", num_types_2.6.0, num_types_2.6.1);
println!("\t{} with max value of {}", num_types_2.7.0, num_types_2.7.1);
println!("\t{} with max value of {}", num_types_2.8.0, num_types_2.8.1);
println!("\t{} with max value of {}", num_types_2.9.0, num_types_2.9.1);

print!("Please enter the numeric type of your first number: ");

let mut datatype_2 = String::new();
io::stdin().read_line(&mut datatype_2).expect("Error: Please input a numeric datatype");

// TODO: if for all types
let mut value_2 = String::new();
io::stdin().read_line(&mut value_2).expect("Error: Please input a numeric value");

if datatype_2 == "u8" {
    num_2 = value_1.parse().unwrap();
} else if datatype_2 == "u16" {
    let num_2 = value_1.parse().unwrap() as u16;
} else if datatype_2 == "u32" {
    let num_2 = value_1.parse().unwrap() as u32;
} else if datatype_2 == "u64" {
    let num_2 = value_1.parse().unwrap() as u64;
} else if datatype_2 == "u128" {
    let num_2 = value_1.parse().unwrap() as u128;
} else if datatype_2 == "i8" {
    let num_2 = value_1.parse().unwrap() as i8;
} else if datatype_2 == "i16" {
    let num_2 = value_1.parse().unwrap() as i16;
} else if datatype_2 == "i32" {
    let num_2 = value_1.parse().unwrap() as i32;
} else if datatype_2 == "i64" {
    let num_2 = value_1.parse().unwrap() as i64;
} else if datatype_2 == "i128" {
    let num_2 = value_1.parse().unwrap() as i128;
}

// create the full equation and print the results and allocated memory
if operator == plus {
    let result = num_1 + num_2;
    println!("{} + {} = {}", num_1, num_2, result);
} else if operator == minus {
    let result = num_1 - num_2;
    println!("{} - {} = {}", num_1, num_2, result);
} else if operator == div {
    let result = num_1 / num_2;
    let remainder = num_1 % num_2;
    println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
} else if operator == "*" {
    let result = num_1 * num_2;
    println!("{} * {} = {}", num_1, num_2, result);
    }
}