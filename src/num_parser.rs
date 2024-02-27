use termint::{
    enums::wrap::Wrap,
    geometry::constrain::Constrain,
    widgets::{layout::Layout, span::StrSpanExtension},
};

use crate::digits::get_digits;

pub fn get_time(num: f64, decimals: usize) -> Vec<String> {
    let digit_lines = get_digits();
    let mut numbers: Vec<String> = Vec::new();
    numbers.resize_with(5, String::new);

    let number = format!("{:.1$}", num, decimals);

    for digit in number.chars() {
        for (i, item) in numbers.iter_mut().enumerate().take(5_usize) {
            item.push_str(digit_lines[&digit][i]);
        }
    }
    numbers
}

pub fn get_time_length(time: &[String]) -> usize {
    if time.is_empty() {
        return 0;
    }
    time[0].len()
}

pub fn time_layout(time: &[String]) -> Layout {
    let time_len = get_time_length(time);
    let time_str = time.join("");

    let mut layout = Layout::horizontal();
    layout.add_child("".to_span(), Constrain::Fill);
    layout.add_child(
        // Grad::new(time_str, (0, 220, 255), (160, 100, 255))
        //     .direction(Direction::Vertical)
        //     .wrap(Wrap::Letter)
        //     .ellipsis(""),
        time_str.to_span().wrap(Wrap::Letter).ellipsis(""),
        Constrain::Length(time_len),
    );
    layout.add_child("".to_span(), Constrain::Fill);
    layout
}
