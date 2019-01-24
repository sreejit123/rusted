#[macro_use]
extern crate text_io;

fn main() {

    println!("Enter number of lines of the christmas tree::");

    let lines:i32 = read!();

    for i in 0..lines {
        for _j in 0..lines-i {
            print!(" ");
        }

        let stars = 2*i+1;

        for _k in 0..stars {
            print!("*");
        }

        println!("");
    }
}
