use self::digits::{get_digits, get_decimal_point};

#[path = "digits.rs"] mod digits;

pub fn print_number(num: f64) {
    let digit_lines = get_digits();
    let decimal_point = get_decimal_point();

    let ipart = num.floor();
    let dpart = ((num % 1.0) * 1000.0).round();
    
    for line in 0..5 {
        for digit in ipart.to_string().chars() {
            let dig = digit.to_digit(10).unwrap();
            print!("{}", digit_lines[dig as usize][line]);
        }

        print!("{}", decimal_point[line]);

        for digit in dpart.to_string().chars() {
            let dig = digit.to_digit(10).unwrap();
            print!("{}", digit_lines[dig as usize][line]);
        }

        println!();
    }
}
