fn main() {

    // the gifts per day
    let gifts = ["Partridge in a Pear Tree", "Turtle Doves", "French Hens", "Calling Birds",
        "Golden Rings", "Geese a Laying", "Swans a Swimming", "Maids a Milking", "Ladies Dancing",
    "Lords a Leaping", "Pipers Piping", "Drummers Drumming"];



    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me, ", (i+1).to_string() + &get_ordinal(i+1).to_string());

        for j in (0..i+1).rev() {
            if j == 0 && i != 0 {
                println!(" and ");
            }

            print!("{} {}", j+1, gifts[j as usize]);

            if j > 1 {
                println!(",");
            }

            if j == 0 {
                println!(".")
            }

        }

        println!();
    }


}


fn get_ordinal(number: i8) -> String {

    let num_str = number.to_string();

    // Numbers ending in 11 to 19 should return 'th'
    if number > 10 {
        let last_two = &num_str[num_str.len() - 2..num_str.len()];

        let last_two_digits = last_two.parse::<i8>().unwrap();

        if last_two_digits > 10 && last_two_digits < 20 {
            return "th".to_string();
        }
    }

    // return according to if number ends with 1, 2, 3 or some other digit
    match num_str.chars().last().unwrap() {
        '1' => return "st".to_string(),
        '2' => return "nd".to_string(),
        '3' => return "rd".to_string(),
        _ => return "th".to_string()
    }

}