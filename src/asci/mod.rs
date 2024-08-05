use digit_type::DigitType;
use termint::{
    enums::Wrap,
    geometry::Constraint,
    widgets::{Grad, Layout},
};

use self::digits::get_digits;

pub mod digit_type;
pub mod digits;

/// Gets time as string created using asci numbers
///
/// **Parameters:**
/// * `num` - number to be converted
/// * `decimals` - number of decimals
///
/// **Returns:**
/// * Number converted to asci
pub fn get_time(
    num: f64,
    decimals: usize,
    font: &DigitType,
) -> (String, usize) {
    let (digits, height) = get_digits(font);
    let number = format!("{:.1$}", num, decimals);
    let mut res = String::new();

    for i in 0..height {
        for digit in number.chars() {
            res.push_str(digits[&digit][i]);
        }
    }
    (res, height)
}

/// Creates layout containing centered asci time (needs to be 5 height)
///
/// **Parameters:**
/// * `num` - number to be added to the layout
/// * `decimals` - number of decimals places to show
///
/// **Returns:**
/// - Time [`Layout`] with centered time
pub fn time_layout(
    num: f64,
    decimals: usize,
    font: &DigitType,
) -> (Layout, usize) {
    let (time, height) = get_time(num, decimals, font);
    let grad =
        Grad::new(time, (0, 220, 255), (160, 100, 255)).wrap(Wrap::Letter);
    let mut layout = Layout::horizontal().center();
    layout.add_child(grad, Constraint::Min(0));
    (layout, height)
}
