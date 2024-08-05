use std::collections::HashMap;

use super::digit_type::DigitType;

/// Gets HashMap containing all the digits and point in ASCII art
pub fn get_digits(
    font: DigitType,
) -> (HashMap<char, Vec<&'static str>>, usize) {
    match font {
        DigitType::Italic => get_italic_digits(),
        DigitType::Train => get_train_digits(),
        DigitType::Card => get_card_digits(),
        DigitType::Regular => get_regular_digits(),
    }
}

fn get_italic_digits() -> (HashMap<char, Vec<&'static str>>, usize) {
    (
        HashMap::from([
            (
                '0',
                vec![
                    r"  ____   ",
                    r" /\   \  ",
                    r" \ \ \ \ ",
                    r"  \ \___\",
                    r"   \/___/",
                ],
            ),
            (
                '1',
                vec![
                    r"  ___    ",
                    r" /\  \   ",
                    r" \ \  \  ",
                    r"  \ \__\ ",
                    r"   \/__/ ",
                ],
            ),
            (
                '2',
                vec![
                    r" _____   ",
                    r"/\__  \  ",
                    r"\/\  __\ ",
                    r" \ \____\",
                    r"  \/____/",
                ],
            ),
            (
                '3',
                vec![
                    r" _____   ",
                    r"/\__  \  ",
                    r"\/\__  \ ",
                    r" \/\____\",
                    r"  \/____/",
                ],
            ),
            (
                '4',
                vec![
                    r" ___     ",
                    r"/\  \__  ",
                    r"\ \__  \ ",
                    r" \/_/\__\",
                    r"    \/__/",
                ],
            ),
            (
                '5',
                vec![
                    r" _____   ",
                    r"/\  __\  ",
                    r"\ \__  \ ",
                    r" \/\____\",
                    r"  \/____/",
                ],
            ),
            (
                '6',
                vec![
                    r" _____   ",
                    r"/\  __\  ",
                    r"\ \  _ \ ",
                    r" \ \____\",
                    r"  \/____/",
                ],
            ),
            (
                '7',
                vec![
                    r" _____   ",
                    r"/\__  \  ",
                    r"\/_/\  \ ",
                    r"   \ \__\",
                    r"    \/__/",
                ],
            ),
            (
                '8',
                vec![
                    r" _____   ",
                    r"/\  _ \  ",
                    r"\ \  _ \ ",
                    r" \ \____\",
                    r"  \/____/",
                ],
            ),
            (
                '9',
                vec![
                    r" _____   ",
                    r"/\  _ \  ",
                    r"\ \__  \ ",
                    r" \/\____\",
                    r"  \/____/",
                ],
            ),
            ('.', vec![r"     ", r"     ", r"  __ ", r" /\_\", r" \/_/"]),
        ]),
        5,
    )
}

fn get_train_digits() -> (HashMap<char, Vec<&'static str>>, usize) {
    (
        HashMap::from([
            (
                '0',
                vec![
                    r"    __  ",
                    r"   /  \ ",
                    r"  | () |",
                    r"  _\__/ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '1',
                vec![
                    r"    _   ",
                    r"   / |  ",
                    r"   | |  ",
                    r"  _|_|_ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '2',
                vec![
                    r"   ___  ",
                    r"  |_  ) ",
                    r"   / /  ",
                    r"  /___| ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '3',
                vec![
                    r"   ____ ",
                    r"  |__ / ",
                    r"   |_ \ ",
                    r"  |___/ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '4',
                vec![
                    r"  _ _   ",
                    r" | | |  ",
                    r" |_  _| ",
                    r"  _|_|_ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '5',
                vec![
                    r"   ___  ",
                    r"  | __| ",
                    r"  |__ \ ",
                    r"  |___/ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '6',
                vec![
                    r"    __  ",
                    r"   / /  ",
                    r"  / _ \ ",
                    r"  \___/ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '7',
                vec![
                    r"   ____ ",
                    r"  |__  |",
                    r"    / / ",
                    r"  _/_/_ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '8',
                vec![
                    r"   ___  ",
                    r"  ( _ ) ",
                    r"  / _ \ ",
                    r"  \___/ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '9',
                vec![
                    r"   ___  ",
                    r"  / _ \ ",
                    r"  \_, / ",
                    r"  _/_/_ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
            (
                '.',
                vec![
                    r"        ",
                    r"        ",
                    r"    _   ",
                    r"  _(_)_ ",
                    "_|\"\"\"\"\"|",
                    "\"`-0-0-'",
                ],
            ),
        ]),
        6,
    )
}

fn get_card_digits() -> (HashMap<char, Vec<&'static str>>, usize) {
    (
        HashMap::from([
            (
                '0',
                vec![
                    r".------.",
                    r"|0.--. |",
                    r"| :/\: |",
                    r"| :\/: |",
                    r"| '--'0|",
                    r"`------'",
                ],
            ),
            (
                '1',
                vec![
                    r".------.",
                    r"|1.--. |",
                    r"| :/\: |",
                    r"| (__) |",
                    r"| '--'1|",
                    r"`------'",
                ],
            ),
            (
                '2',
                vec![
                    r".------.",
                    r"|2.--. |",
                    r"| (\/) |",
                    r"| :\/: |",
                    r"| '--'2|",
                    r"`------'",
                ],
            ),
            (
                '3',
                vec![
                    r".------.",
                    r"|3.--. |",
                    r"| :(): |",
                    r"| ()() |",
                    r"| '--'3|",
                    r"`------'",
                ],
            ),
            (
                '4',
                vec![
                    r".------.",
                    r"|4.--. |",
                    r"| :/\: |",
                    r"| :\/: |",
                    r"| '--'4|",
                    r"`------'",
                ],
            ),
            (
                '5',
                vec![
                    r".------.",
                    r"|5.--. |",
                    r"| :/\: |",
                    r"| (__) |",
                    r"| '--'5|",
                    r"`------'",
                ],
            ),
            (
                '6',
                vec![
                    r".------.",
                    r"|6.--. |",
                    r"| (\/) |",
                    r"| :\/: |",
                    r"| '--'6|",
                    r"`------'",
                ],
            ),
            (
                '7',
                vec![
                    r".------.",
                    r"|7.--. |",
                    r"| :(): |",
                    r"| ()() |",
                    r"| '--'7|",
                    r"`------'",
                ],
            ),
            (
                '8',
                vec![
                    r".------.",
                    r"|8.--. |",
                    r"| :/\: |",
                    r"| :\/: |",
                    r"| '--'8|",
                    r"`------'",
                ],
            ),
            (
                '9',
                vec![
                    r".------.",
                    r"|9.--. |",
                    r"| :/\: |",
                    r"| (__) |",
                    r"| '--'9|",
                    r"`------'",
                ],
            ),
            (
                '.',
                vec![
                    r".------.",
                    r"|..--. |",
                    r"| :(): |",
                    r"| ()() |",
                    r"| '--'.|",
                    r"`------'",
                ],
            ),
        ]),
        6,
    )
}

fn get_regular_digits() -> (HashMap<char, Vec<&'static str>>, usize) {
    (
        HashMap::from([
            (
                '0',
                vec![
                    r"  ██████ ",
                    r" ██  ████",
                    r" ██ ██ ██",
                    r" ████  ██",
                    r"  ██████ ",
                ],
            ),
            ('1', vec![r"  ██", r" ███", r"  ██", r"  ██", r"  ██"]),
            (
                '2',
                vec![
                    r" ██████ ",
                    r"      ██",
                    r"  █████ ",
                    r" ██     ",
                    r" ███████",
                ],
            ),
            (
                '3',
                vec![
                    r" ██████ ",
                    r"      ██",
                    r"  █████ ",
                    r"      ██",
                    r" ██████ ",
                ],
            ),
            (
                '4',
                vec![
                    r" ██   ██",
                    r" ██   ██",
                    r" ███████",
                    r"      ██",
                    r"      ██",
                ],
            ),
            (
                '5',
                vec![
                    r" ███████",
                    r" ██     ",
                    r" ███████",
                    r"      ██",
                    r" ███████",
                ],
            ),
            (
                '6',
                vec![
                    r"  ██████ ",
                    r" ██      ",
                    r" ███████ ",
                    r" ██    ██",
                    r"  ██████ ",
                ],
            ),
            (
                '7',
                vec![
                    r" ███████",
                    r"      ██",
                    r"     ██ ",
                    r"    ██  ",
                    r"    ██  ",
                ],
            ),
            (
                '8',
                vec![
                    r"  █████ ",
                    r" ██   ██",
                    r"  █████ ",
                    r" ██   ██",
                    r"  █████ ",
                ],
            ),
            (
                '9',
                vec![
                    r"  █████ ",
                    r" ██   ██",
                    r"  ██████",
                    r"      ██",
                    r"  █████ ",
                ],
            ),
            ('.', vec![r"   ", r"   ", r"   ", r"   ", r" ██"]),
        ]),
        5,
    )
}

//  __   ___    ____    _  _     _____     __    ______    ___     ___
// /_ | |__ \  |___ \  | || |   | ____|   / /   |____  |  / _ \   / _ \
//  | |    ) |   __) | | || |_  | |__    / /_       / /  | (_) | | (_) |
//  | |   / /   |__ <  |__   _| |___ \  | '_ \     / /    > _ <   \__, |
//  | |  / /_   ___) |    | |    ___) | | (_) |   / /    | (_) |    / /
//  |_| |____| |____/     |_|   |____/   \___/   /_/      \___/    /_/

//  ██ ██████  ██████  ██   ██ ███████  ██████  ███████  █████   █████
// ███      ██      ██ ██   ██ ██      ██            ██ ██   ██ ██   ██
//  ██  █████   █████  ███████ ███████ ███████      ██   █████   ██████
//  ██ ██           ██      ██      ██ ██    ██    ██   ██   ██      ██
//  ██ ███████ ██████       ██ ███████  ██████     ██    █████   █████

//  ██╗██████╗ ██████╗ ██╗  ██╗███████╗ ██████╗███████╗ █████╗  █████╗
// ███║╚════██╗╚════██╗██║  ██║██╔════╝██╔════╝╚════██║██╔══██╗██╔══██╗
// ╚██║ █████╔╝ █████╔╝███████║███████╗███████╗    ██╔╝╚█████╔╝╚██████║
//  ██║██╔═══╝  ╚═══██╗╚════██║╚════██║██╔═══██╗  ██╔╝ ██╔══██╗ ╚═══██║
//  ██║███████╗██████╔╝     ██║███████║╚██████╔╝  ██║  ╚█████╔╝ █████╔╝
//  ╚═╝╚══════╝╚═════╝      ╚═╝╚══════╝ ╚═════╝   ╚═╝   ╚════╝  ╚════╝
