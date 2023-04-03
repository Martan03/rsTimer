use self::digits::{get_digits};

#[path = "digits.rs"]
mod digits;

pub fn print_number(num: f64) {
    let digit_lines = get_digits();
    let mut numbers: Vec<String> = Vec::new();
    numbers.resize_with(5,  String::new);

    let number = format!("{:.3}", num);

    for digit in number.chars() {
        for i in 0..5 as usize {
            numbers[i].push_str(digit_lines[&digit][i]);
        }
    }

    for line in numbers {
        println!("{line}");
    }
}
