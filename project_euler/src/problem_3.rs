pub fn run() {
    println!("Running Problem 3...");

    const number : i64 = 600851475143;

    let mut n = number;

    let max_prime =   loop {
        let x = smallest_factor(n);
        if x < n {
            n /= x;
        } else {
            break n;
        }


    };

    println!("Largest prime factor of {} is {}", number, max_prime);
}

fn smallest_factor( num : i64) -> i64 {
    for i in 2..num/2 {
        if num % i == 0 {
            return i;
        }
    }

    return num;
}