/*
    File: calculator.rs
    Authors: Elijah Acuna and Amy Paul
    Class: CSc 372, Fall 2021
    Purpose: This file demonstrates different integer types in Rust through a calculator program.

    Note: Rust uses the snake_case naming convention, which is followed
    throughout the program code.
*/


use std::io;

/*
    run: Takes in user input on data types and demonstrates
*/
pub fn run() {
    // Use tuples to hold the possible data types.
    // Group all the tuples into an immutable single tuple of tuples
    let num_types_1 = (("u8", u8::MAX), ("u16", u16::MAX), ("u32", u32::MAX), ("u64", u64::MAX), ("u128", u128::MAX), 
        ("ui8", i8::MAX), ("i16", i16::MAX), ("i32", i32::MAX), ("i64", i64::MAX), ("i128", i128::MAX));
    let plus = "+";
    let minus = "-";
    let div = "/";
    let mult = "*";

    // Intro prompt
    println!("Welcome to Bitmath!\n");

    /* I'm so sorry, i know this is hideous but Rust doesn't support indexing tuples in a loop.
    you can try it if you want, it would be:

    for i in 0..10 {
        println!("\t{} with max value of {}", num_types_1.i.0, num_types_1.i.2);
    }

    */

    // List datatypes for num1
    println!("Supported data types:");
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

    println!("Please enter the type of your numbers (Select signed if you want to use negatives): ");
    let mut datatype_1 = String::new();
    io::stdin().read_line(&mut datatype_1).expect("Error: Please input a numeric datatype");
    datatype_1 = datatype_1.trim_end().to_string();

    println!("Please enter your first number: ");
    let mut value_1 = String::new();
    io::stdin().read_line(&mut value_1).expect("Error: Please input a numeric value");
    value_1 = value_1.trim_end().to_string();

    println!("What operation would you like to do? (* / + -)");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Error: Please input a math operator");    
    operator = operator.trim_end().to_string();

    println!("Please enter your second number: ");

    let mut value_2 = String::new();
    io::stdin().read_line(&mut value_2).expect("Error: Please input a numeric value");
    value_2 = value_2.trim_end().to_string();

    println!("hello {}", datatype_1);

    // Unfortunately there isn't a better way to do explicit conversions than this. This is a real limitation of the Rust
    // language when it comes to integers.
    if datatype_1.eq("u8") {
        println!("hi");
        let mut num_1: u8 = 0;
        let mut num_2: u8 = 0;
        
        // Rust doesn't have a built-in try-catch block, this is the equivalent.
        // Verifies that the user actually entered a numeric value.
        match value_1.parse::<u8>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        // Rust returns errors as part of a Result structure. We can use this
        // to catch and respond to errors before they crash our program.
        match value_2.parse::<u8>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        // Create the full equation and print the results
        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("u16") {
        let mut num_1: u16 = 0;
        let mut num_2: u16 = 0;
        match value_1.parse::<u16>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<u16>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }
        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("u32") {
        let mut num_1: u32 = 0;
        let mut num_2: u32 = 0;
        match value_1.parse::<u32>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<u32>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("u64") {
        let mut num_1: u64 = 0;
        let mut num_2: u64 = 0;
        match value_1.parse::<u64>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<u64>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("u128") {
        let mut num_1: u128 = 0;
        let mut num_2: u128 = 0;
        match value_1.parse::<u128>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<u128>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("i8") {
        let mut num_1: i8 = 0;
        let mut num_2: i8 = 0;
        match value_1.parse::<i8>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<i8>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("i16") {
        let mut num_1: i16 = 0;
        let mut num_2: i16 = 0;
        match value_1.parse::<i16>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<i16>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("i32") {
        let mut num_1: i32 = 0;
        let mut num_2: i32 = 0;
        match value_1.parse::<i32>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<i32>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("i64") {
        let mut num_1: i64 = 0;
        let mut num_2: i64 = 0;
        match value_1.parse::<i64>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<i64>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else if datatype_1.eq("i128") {
        let mut num_1: i128 = 0;
        let mut num_2: i128 = 0;
        match value_1.parse::<i128>() {
            Ok(i) => num_1 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_1);
                return;
            },
        }

        match value_2.parse::<i128>() {
            Ok(i) => num_2 = i,
            Err(..) => {
                println!("this was not an integer: {}", value_2);
                return;
            },
        }

        if operator.eq(plus) {
            let result = num_1 + num_2;
            println!("{} + {} = {}", num_1, num_2, result);
        } else if operator.eq(minus) {
            let result = num_1 - num_2;
            println!("{} - {} = {}", num_1, num_2, result);
        } else if operator.eq(div) {
            let result = num_1 / num_2;
            let remainder = num_1 % num_2;
            println!("{} / {} = {} with remainder {}", num_1, num_2, result, remainder);
        } else if operator.eq(mult) {
            let result = num_1 * num_2;
            println!("{} * {} = {}", num_1, num_2, result);
        } else {
            println!("Invalid operator.");
        }
    } else {
        println!("Invalid data type.");
    }
}