use rand::seq::IteratorRandom;
use rand::Rng;

pub fn remove_special_chars_with_capacity(input: &str) -> String {
    let mut result = String::with_capacity((input.len() as f64 * 1.1) as usize);

    for c in input.chars() {
        match c {
            '\u{201C}' => result.push('\"'),  // LEFT DOUBLE QUOTATION MARK
            '\u{201D}' => result.push('\"'),  // RIGHT DOUBLE QUOTATION MARK
            '\u{2018}' => result.push('\''),   // LEFT SINGLE QUOTATION MARK
            '\u{2019}' => result.push('\''),   // RIGHT SINGLE QUOTATION MARK
            '\u{2013}' => result.push('-'),   // EN DASH
            '\u{2014}' => result.push_str("--"),  // EM DASH
            '\u{2022}' => result.push('*'),   // Bullet
            '\u{2026}' => result.push_str("..."), // HORIZONTAL ELLIPSIS
            '\u{2039}' => result.push('<'),   // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
            '\u{203A}' => result.push('>'),   // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
            _ => result.push(c),
        }
    }

    result.shrink_to_fit();
    result
}

pub fn remove_special_chars_without_capacity(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        match c {
            '\u{201C}' => result.push('\"'),  // LEFT DOUBLE QUOTATION MARK
            '\u{201D}' => result.push('\"'),  // RIGHT DOUBLE QUOTATION MARK
            '\u{2018}' => result.push('\''),   // LEFT SINGLE QUOTATION MARK
            '\u{2019}' => result.push('\''),   // RIGHT SINGLE QUOTATION MARK
            '\u{2013}' => result.push('-'),   // EN DASH
            '\u{2014}' => result.push_str("--"),  // EM DASH
            '\u{2022}' => result.push('*'),   // Bullet
            '\u{2026}' => result.push_str("..."), // HORIZONTAL ELLIPSIS
            '\u{2039}' => result.push('<'),   // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
            '\u{203A}' => result.push('>'),   // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
            _ => result.push(c),
        }
    }

    result.shrink_to_fit();
    result
}

pub fn remove_special_chars_replace(input: &str) -> String {
    input
        .replace('\u{2014}', "--") // EM DASH
        .replace('\u{2026}', "...") // HORIZONTAL ELLIPSIS
        .chars()
        .map(|x| match x {
            '\u{201C}' => '"',  // LEFT DOUBLE QUOTATION MARK
            '\u{201D}' => '"',  // RIGHT DOUBLE QUOTATION MARK
            '\u{2018}' => '\'', // LEFT SINGLE QUOTATION MARK
            '\u{2019}' => '\'', // RIGHT SINGLE QUOTATION MARK
            '\u{2013}' => '-',  // EN DASH
            '\u{2022}' => '*',  // Bullet
            '\u{2039}' => '<',  // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
            '\u{203A}' => '>',  // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
            _ => x,
        })
        .collect()
}

const CHAR_TABLE: &str = " \t\nabcdefghijklmnopqrstuvwxzyABCDEFGHIJKLMNOPQRSTUVWXZY\u{201C}\u{201D}\u{2018}\u{2019}\u{2013}\u{2014}\u{2022}\u{2026}\u{2039}\u{203A}";

pub fn generate_random_special_char_string<R: Rng>(rng: &mut R, len: usize) -> String {
    let mut result = String::with_capacity(len);

    for _ in 0..len {
        result.push(CHAR_TABLE.chars().choose(rng).unwrap());
    }

    result
}