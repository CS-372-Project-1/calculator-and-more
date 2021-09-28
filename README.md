# Calculator and Auto_Array
Welcome to our smaller demo programs. Calculator demonstrates the different
varieties of integers available in Rust, and Auto_Array fills an array with integers from
0 to 100 and prints it out using special debug print statements.

## How to Run
To run, simply install Rust into your terminal using the 
instructions here: https://www.rust-lang.org/tools/install. Next, navigate to 
the calculator folder in the calculator-and-more directory in the terminal and run the 
command: cargo run. Simply observe the output of the array program and follow
the prompts to interact with the calculator program.

The program will most likely run with warnings. This is due to Rust's compile-time usage checker, 
and we think it might be due to code coverage as well but we are not sure. Rest assured, this program
has been tested thoroughly and works perfectly as intended. If you want, you can run only auto_array by 
commenting lines 12 and 19 or only calculator by commenting lines 13 and 20 in main.rs. Disabling calculator
will run auto_array with no terminal warnings for ease of viewing.

## Language Coverage
Facets of the language demonstrated by this program are:
- Arrays
- Commenting
- Conditionals
- Errors
- For loops and iterators
- Functions
- Getting User Input via Stdin
- Integers (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128)
- Printing to Stdout
- Match statements (Rust try/catch)
- Integer Overflow
- Mutable vs. Immutable Values
- Splitting a program into multiple files
- Strings
- Tuples

For more information, please see the comments in individual files.
