/* the final feature of the Rust programming language that we wanted 
   to show off is a nifty thing called debug print statements.
   It is denoted by using {:?} in your string to print, and then
   passing an array, vector, hashmap, or some other iterable structure.

   Important to note: it is the only way to print an iterable other than
   looping by index
*/

pub fn run() {
    let i = 0;
    let mut numbers: [usize; 100];
    while i < 100 {
        numbers[i] = i;
    }
    println!("{:?}", numbers);
}