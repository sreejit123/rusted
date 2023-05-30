pub fn run() {
    println!("Running Problem 2...");

    let (mut val, mut first, mut second, mut sum) = (0, 0, 1, 0);


    while val <= 4000000 {
        val = first + second;
        if val %2 == 0 {
            sum += val;
        }

        first = second;
        second = val;
    }

    println!("Sum of even fibo numbers under 4000000 is {}", sum);
}