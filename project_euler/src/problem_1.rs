pub fn run() {
    println!("Running Problem 1...");

    let mut sum : i32 = 0;
    for i in 1..1000 {
        if i % 3 == 0 ||  i % 5 == 0 {
            sum += i;
        }
    }

    println!("Sum is {}", sum);
}