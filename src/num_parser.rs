use self::digits::get_digits;

#[path = "digits.rs"]
mod digits;

pub fn get_time(num: f64, decimals: usize) -> Vec<String> {
    let digit_lines = get_digits();
    let mut numbers: Vec<String> = Vec::new();
    numbers.resize_with(5, String::new);

    let number = format!("{:.1$}", num, decimals);

    for digit in number.chars() {
        for i in 0..5 as usize {
            numbers[i].push_str(digit_lines[&digit][i]);
        }
    }
    numbers
}

fn get_time_length(time: &Vec<String>) -> usize {
    if time.len() <= 0 {
        return 0;
    }
    return time[0].len();
}

pub fn print_time(time: Vec<String>) {
    let (w, h) = termion::terminal_size().unwrap();
    let px = (w as usize - get_time_length(&time)) / 2;
    let mut py = (h as usize - 5) / 2;

    let mut r = 0;
    let mut g = 220;
    for line in time {
        println!("\x1b[{py};{px}H\x1b[38;2;{r};{g};255m{line}");
        r += 40;
        g -= 30;
        py += 1;
    }
}
