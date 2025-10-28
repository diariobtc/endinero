//! Copyright [2022] DiarioBitcoin.com, Angel Leon
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! https://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

/// ## Customize the separator characters and decimal precision used to format the output of the given `f64` number
///
/// ## Parameters:
/// **amount: f64**: the original `amount` in `f64`
///
/// **max_decimal_places: u16**: number of decimal digits to keep for `amounts > 0`
///
/// **zero_comma_decimal_places: u16**: number of decimal digits to keep for `amount < 1`
///
/// **thousands_separator: char**: character to be used to separate thousands on the integer part of `amount`
///
/// **radix_character: char**: character that separates the integer portion from the decimal portion (e.g. `.` for numbers in US locale but `,` for numbers in Spanish locales)
///
/// **decimal_separator: char**: character that separates decimal portions every thousandth
pub fn endinero_f64(
    amount: f64,
    max_decimal_places: u16,
    zero_comma_decimal_places: u16,
    thousands_separator: char,
    radix_character: char,
    decimals_separator: char,
) -> String {
    let int_part = integer_part_f64(amount, thousands_separator);
    let total_decimals = if amount.abs() < 1.0f64 {
        zero_comma_decimal_places
    } else {
        max_decimal_places
    };
    let dec_part = decimal_part_f64(amount, total_decimals, decimals_separator);

    let mut result = String::with_capacity(int_part.len() + dec_part.len() + 1);
    result.push_str(&int_part);
    result.push(radix_character);
    result.push_str(&dec_part);
    result
}

/// ## Customize the separator characters and decimal precision used to format the output of the given `f32` number
///
/// ## Parameters:
/// **amount: f64**: the original `amount` in `f32`
///
/// **max_decimal_places: u16**: number of decimal digits to keep for `amounts > 0`
///
/// **zero_comma_decimal_places: u16**: number of decimal digits to keep for `amount < 1`
///
/// **thousands_separator: char**: character to be used to separate thousands on the integer part of `amount`
///
/// **radix_character: char**: character that separates the integer portion from the decimal portion (e.g. `.` for numbers in US locale but `,` for numbers in Spanish locales)
///
/// **decimal_separator: char**: character that separates decimal portions every thousandth
pub fn endinero_f32(
    amount: f32,
    max_decimal_places: u16,
    zero_comma_decimal_places: u16,
    thousands_separator: char,
    radix_character: char,
    decimals_separator: char,
) -> String {
    let int_part = integer_part_f32(amount, thousands_separator);
    let total_decimals = if amount.abs() < 1.0 {
        zero_comma_decimal_places
    } else {
        max_decimal_places
    };
    let dec_part = decimal_part_f32(amount, total_decimals, decimals_separator);

    let mut result = String::with_capacity(int_part.len() + dec_part.len() + 1);
    result.push_str(&int_part);
    result.push(radix_character);
    result.push_str(&dec_part);
    result
}

/// Format `f64` input into a spanish money format, max 2 decimal places if amount > 0, up to 17 decimal places for amount < 1. Thousands separator: `.`, Radix character: `,`, Decimals separator: ` `
pub fn dinero_f64(amount: f64) -> String {
    endinero_f64(amount, 2, 17, '.', ',', ' ')
}

/// Format `f32` input into a spanish money format, max 2 decimal places if amount > 0, up to 17 decimal places for amount < 1. Thousands separator: `.`, Radix character: `,`, Decimals separator: ` `
pub fn dinero_f32(amount: f32) -> String {
    endinero_f32(amount, 2, 7, '.', ',', ' ')
}

/// Format `f64` input into a US money format, max 2 decimal places if amount > 0, up to 17 decimal places for amount < 1. Thousands separator: `,`, Radix character: `.`, Decimals separator: ` `
pub fn money_f64(amount: f64) -> String {
    endinero_f64(amount, 2, 17, ',', '.', ' ')
}

/// Format `f32` input into a US money format, max 2 decimal places if amount > 0, up to 7 decimal places for amount < 1. Thousands separator: `,`, Radix character: `.`, Decimals separator: ` `
pub fn money_f32(amount: f32) -> String {
    endinero_f32(amount, 2, 7, ',', '.', ' ')
}

fn integer_part_f64(amount: f64, thousands_separator: char) -> String {
    let int_part = amount.abs().trunc() as i64;
    let unsigned_int_digits_str = format!("{}", int_part);
    let digits_len = unsigned_int_digits_str.len();
    let separator_count = (digits_len.saturating_sub(1)) / 3;
    let mut result = String::with_capacity(digits_len + separator_count);

    for (i, c) in unsigned_int_digits_str.chars().enumerate() {
        if i > 0 && (digits_len - i) % 3 == 0 {
            result.push(thousands_separator);
        }
        result.push(c);
    }

    if amount < 0f64 {
        result.insert(0, '-');
    }
    result
}

fn integer_part_f32(amount: f32, thousands_separator: char) -> String {
    let int_part = amount.abs().trunc() as i32;
    let unsigned_int_digits_str = int_part.to_string();
    let digits_len = unsigned_int_digits_str.len();
    let separator_count = (digits_len.saturating_sub(1)) / 3;
    let mut result = String::with_capacity(digits_len + separator_count);

    for (i, c) in unsigned_int_digits_str.chars().enumerate() {
        if i > 0 && (digits_len - i).is_multiple_of(3) {
            result.push(thousands_separator);
        }
        result.push(c);
    }

    if amount < 0f32 {
        result.insert(0, '-');
    }
    result
}

fn format_decimal_digits(
    dec_digits_str: &str,
    total_decimals: u16,
    decimals_separator: char,
) -> String {
    let mut result = String::with_capacity(total_decimals as usize + 4);
    let mut digits_added = 0;
    for c in dec_digits_str.chars() {
        if digits_added > 0 && digits_added % 3 == 0 {
            result.push(decimals_separator);
        }
        result.push(c);
        digits_added += 1;

        if digits_added == total_decimals {
            break;
        }
    }

    // remove any zeroes or blank spaces left over at the end
    while result.len() > 1 {
        match result.chars().last() {
            Some('0') | Some(' ') => result.pop(),
            _ => break,
        };
    }

    result
}

fn decimal_part_f64(amount: f64, total_decimals: u16, decimals_separator: char) -> String {
    let formatted = format!("{:.17}", amount);
    let dec_digits_str = formatted
        .split_once('.')
        .map(|(_, dec)| dec)
        .unwrap_or("");
    format_decimal_digits(dec_digits_str, total_decimals, decimals_separator)
}

fn decimal_part_f32(amount: f32, total_decimals: u16, decimals_separator: char) -> String {
    let formatted = format!("{:.7}", amount);
    let dec_digits_str = formatted
        .split_once('.')
        .map(|(_, dec)| dec)
        .unwrap_or("");
    format_decimal_digits(dec_digits_str, total_decimals, decimals_separator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_part_f64_basic() {
        assert_eq!(integer_part_f64(0.0, '.'), "0");
        assert_eq!(integer_part_f64(1234.0, '.'), "1.234");
        assert_eq!(integer_part_f64(-1234.0, '.'), "-1.234");
    }

    #[test]
    fn decimal_part_f64_basic() {
        assert_eq!(decimal_part_f64(0.1, 1, ' '), "1");
        assert_eq!(decimal_part_f64(0.12, 2, ' '), "12");
        assert_eq!(decimal_part_f64(0.222333444555, 12, '.'), "222.333.444.555");
    }

    #[test]
    fn format_decimal_digits_basic() {
        assert_eq!(format_decimal_digits("123456789", 9, '.'), "123.456.789");
        assert_eq!(format_decimal_digits("1", 1, ' '), "1");
    }
}
