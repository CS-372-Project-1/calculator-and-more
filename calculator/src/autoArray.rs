pub fn run() {
    let i = 0;
    let mut numbers: [usize; 100];
    while i < 100 {
        numbers[i] = i;
    }
    println!("{:?}", numbers);
}