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
    format!("{}{}{}", int_part, radix_character, dec_part)
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
    // INTEGER PART
    let int_part = integer_part_f32(amount, thousands_separator);
    let total_decimals = if amount.abs() < 1.0 {
        zero_comma_decimal_places
    } else {
        max_decimal_places
    };
    // DECIMAL PART
    let dec_part = decimal_part_f32(amount, total_decimals, decimals_separator);
    // RESULT
    format!("{}{}{}", int_part, radix_character, dec_part)
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

/// Format `f32` input into a US money format, max 2 decimal places if amount > 0, up to 17 decimal places for amount < 1. Thousands separator: `,`, Radix character: `.`, Decimals separator: ` `
pub fn money_f32(amount: f64) -> String {
    endinero_f64(amount, 2, 7, ',', '.', ' ')
}

fn integer_part_f64(amount: f64, thousands_separator: char) -> String {
    let int_part = amount.abs().trunc() as i64;
    let unsigned_int_digits_str = format!("{}", int_part);
    let mut formatted_int_digits: Vec<char> = Vec::new();
    for (digits_added, c) in unsigned_int_digits_str.chars().rev().enumerate() {
        if digits_added > 0 && digits_added % 3 == 0 {
            formatted_int_digits.push(thousands_separator);
        }
        formatted_int_digits.push(c);
    }
    if amount < 0f64 {
        formatted_int_digits.push('-');
    }
    formatted_int_digits.reverse();
    let result = formatted_int_digits.iter().collect::<String>();
    result
}

fn integer_part_f32(amount: f32, thousands_separator: char) -> String {
    let int_part = amount.abs().trunc() as i32;
    let unsigned_int_digits_str = int_part.to_string(); //format!("{}", int_part);
    let mut formatted_int_digits: Vec<char> = Vec::new();
    for (digits_added, c) in unsigned_int_digits_str.chars().rev().enumerate() {
        if digits_added > 0 && digits_added % 3 == 0 {
            formatted_int_digits.push(thousands_separator);
        }
        formatted_int_digits.push(c);
    }
    if amount < 0f32 {
        formatted_int_digits.push('-');
    }
    formatted_int_digits.reverse();
    let result = formatted_int_digits.iter().collect::<String>();
    result
}

fn decimal_part_f64(amount: f64, total_decimals: u16, decimals_separator: char) -> String {
    let dec_digits_str = format!("{:.17}", amount)
        .split(".")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .to_string();
    let mut formatted_dec_digits: Vec<char> = Vec::new();
    let mut digits_added = 0;
    for c in dec_digits_str.chars() {
        if digits_added > 0 && digits_added % 3 == 0 {
            formatted_dec_digits.push(decimals_separator);
        }
        formatted_dec_digits.push(c);
        digits_added += 1;

        if digits_added == total_decimals {
            break;
        }
    }

    // remove any zeroes or blank spaces left over at the end
    while formatted_dec_digits.len() > 1
        && formatted_dec_digits.last().is_some()
        && (formatted_dec_digits.last().unwrap() == &'0'
            || formatted_dec_digits.last().unwrap() == &' ')
    {
        formatted_dec_digits.pop();
    }

    let result = formatted_dec_digits.iter().collect::<String>();
    result
}

fn decimal_part_f32(amount: f32, total_decimals: u16, decimals_separator: char) -> String {
    let dec_digits_str = format!("{:.7}", amount)
        .split(".")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .to_string();
    let mut formatted_dec_digits: Vec<char> = Vec::new();
    let mut digits_added = 0;
    for c in dec_digits_str.chars() {
        if digits_added > 0 && digits_added % 3 == 0 {
            formatted_dec_digits.push(decimals_separator);
        }
        formatted_dec_digits.push(c);
        digits_added += 1;

        if digits_added == total_decimals {
            break;
        }
    }
    // remove any zeroes or blank spaces left over at the end
    while formatted_dec_digits.len() > 1
        && formatted_dec_digits.last().is_some()
        && (formatted_dec_digits.last().unwrap() == &'0'
            || formatted_dec_digits.last().unwrap() == &' ')
    {
        formatted_dec_digits.pop();
    }

    let result = formatted_dec_digits.iter().collect::<String>();
    result
}

#[test]
fn integer_part_tests() {
    // Zeroes
    assert_eq!(integer_part_f64(0.0, '.'), "0");
    assert_eq!(integer_part_f64(-0.0, '.'), "0");
    assert_eq!(integer_part_f64(-0.0001, '.'), "-0");

    // Units [1-9]
    assert_eq!(integer_part_f64(1.0, '.'), "1");
    assert_eq!(integer_part_f64(-1.0, '.'), "-1");

    // Tens
    assert_eq!(integer_part_f64(12.0, '.'), "12");
    assert_eq!(integer_part_f64(-12.0, '.'), "-12");

    // Hundreds
    assert_eq!(integer_part_f64(123.0, '.'), "123");
    assert_eq!(integer_part_f64(-123.0, '.'), "-123");

    // Thousands
    assert_eq!(integer_part_f64(1234.0, '.'), "1.234");
    assert_eq!(integer_part_f64(-1234.0, '.'), "-1.234");

    // Tens of thousands
    assert_eq!(integer_part_f64(12345.0, '.'), "12.345");
    assert_eq!(integer_part_f64(-12345.0, '.'), "-12.345");

    // Hundreds of thousands
    assert_eq!(integer_part_f64(123456.0, '.'), "123.456");
    assert_eq!(integer_part_f64(-123456.0, '.'), "-123.456");

    // Millions
    assert_eq!(integer_part_f64(1234567.0, '.'), "1.234.567");
    assert_eq!(integer_part_f64(-1234567.0, '.'), "-1.234.567");

    // Tens of millions
    assert_eq!(integer_part_f64(12345678.0, '.'), "12.345.678");
    assert_eq!(integer_part_f64(-12345678.0, '.'), "-12.345.678");

    // Hundreds of millions
    assert_eq!(integer_part_f64(123456789.0, '.'), "123.456.789");
    assert_eq!(integer_part_f64(-123456789.0, '.'), "-123.456.789");

    // Billions / Millardos
    assert_eq!(integer_part_f64(1234567890.0, '.'), "1.234.567.890");
    assert_eq!(integer_part_f64(-1234567890.0, '.'), "-1.234.567.890");

    // Tens of billions / Decenas de millardos
    assert_eq!(integer_part_f64(12345678901.0, '.'), "12.345.678.901");
    assert_eq!(integer_part_f64(-12345678901.0, '.'), "-12.345.678.901");

    // Hundreds of billions / Cientos de millardos
    assert_eq!(integer_part_f64(123456789012.0, '.'), "123.456.789.012");
    assert_eq!(integer_part_f64(-123456789012.0, '.'), "-123.456.789.012");

    // Trillions (10^12) / Billones
    assert_eq!(integer_part_f64(1234567890123.0, '.'), "1.234.567.890.123");
    assert_eq!(
        integer_part_f64(-1234567890123.0, '.'),
        "-1.234.567.890.123"
    );

    // Tens of trillions (10^13) / Decenas de billones
    assert_eq!(
        integer_part_f64(12345678901234.0, '.'),
        "12.345.678.901.234"
    );
    assert_eq!(
        integer_part_f64(-12345678901234.0, '.'),
        "-12.345.678.901.234"
    );

    // Hundreds of trillions (10^14) / Cientos de billones
    assert_eq!(
        integer_part_f64(123456789012345.0, '.'),
        "123.456.789.012.345"
    );
    assert_eq!(
        integer_part_f64(-123456789012345.0, '.'),
        "-123.456.789.012.345"
    );

    // Quadrillion (10^15) / Billardo
    assert_eq!(
        integer_part_f64(1234567890123456.0, '.'),
        "1.234.567.890.123.456"
    );
    assert_eq!(
        integer_part_f64(-1234567890123456.0, '.'),
        "-1.234.567.890.123.456"
    );
}

#[test]
fn decimal_part_tests() {
    assert_eq!(decimal_part_f64(0.1, 1, ' '), "1");
    assert_eq!(decimal_part_f64(0.12, 1, ' '), "1");

    assert_eq!(decimal_part_f64(0.12, 2, ' '), "12");
    assert_eq!(decimal_part_f64(0.123, 2, ' '), "12");

    assert_eq!(decimal_part_f64(0.123, 3, ' '), "123");
    assert_eq!(decimal_part_f64(0.1234, 3, ' '), "123");

    assert_eq!(decimal_part_f64(0.1234, 4, ' '), "123 4");
    assert_eq!(decimal_part_f64(0.1234, 4, '.'), "123.4");

    assert_eq!(decimal_part_f64(0.222333444555, 12, '.'), "222.333.444.555");
    assert_eq!(
        decimal_part_f64(0.2223334445556, 12, '.'),
        "222.333.444.555"
    );
    assert_eq!(
        decimal_part_f64(0.2223334445556, 13, '.'),
        "222.333.444.555.6"
    );

    assert_eq!(decimal_part_f32(0.2223334445556, 7, '.'), "222.333.4");
    // breaks after 7 digits, starts doing weird rounding
    assert_ne!(decimal_part_f32(0.22233344455566, 8, '.'), "222.333.44");
}

#[test]
fn tests() {
    assert_eq!(dinero_f32(3.1415926), "3,14");
    assert_eq!(dinero_f32(1000000.44), "1.000.000,43");
    assert_eq!(dinero_f64(10000000.44), "10.000.000,43");

    assert_eq!(
        endinero_f64(1234567.456, 2, 4, '.', ',', ' '),
        "1.234.567,45"
    );

    assert_eq!(
        endinero_f64(-1234567.456789, 2, 4, '.', ',', ' '),
        "-1.234.567,45"
    );

    assert_eq!(
        endinero_f64(
            -1234567.456789, // amount
            4,               // decimals for numbers > 0
            4,               // decimals for numbers < 0
            '.',             // thousands separator
            ',',             // radix separator
            ' '
        ), // decimals separator
        "-1.234.567,456 7"
    );

    assert_eq!(endinero_f64(0.456, 2, 4, '.', ',', ' '), "0,456");
    assert_eq!(endinero_f64(0.456789, 2, 4, '.', ',', '.'), "0,456.7");
    assert_eq!(endinero_f64(0.456789, 2, 5, '.', ',', '.'), "0,456.78");
    assert_eq!(endinero_f64(0.456789, 2, 5, '.', '🔻', '.'), "0🔻456.78");
    assert_eq!(endinero_f64(0.456789, 2, 6, '.', '🔻', '.'), "0🔻456.789");

    assert_eq!(dinero_f64(0.1234567), "0,123 456 7");
    assert_eq!(dinero_f64(10000000.1232456), "10.000.000,12");

    // info!("Hundreds of trillions (10^14) / Cientos de billones");
    let x: f64 = 123456789012345.1234;
    assert_eq!(dinero_f64(x), "123.456.789.012.345,12"); // rounding issue

    // should break here
    assert_ne!(dinero_f64(x), "1.234.567.890.123.456,12");
    assert_eq!(dinero_f64(0.12345678912345678), "0,123 456 789 123 456 78");

    assert_eq!(dinero_f32(10.111), "10,11");
    assert_eq!(dinero_f32(-10.111), "-10,11");

    assert_eq!(dinero_f32(0.2223334445556), "0,222 333 4");

    // rounding issue here
    assert_eq!(dinero_f32(11111111.1234), "11.111.111,0");
    assert_eq!(dinero_f32(0.1234567), "0,123 456 7");

    assert_eq!(money_f32(12345678.123456), "12,345,678.12");
    assert_eq!(money_f32(1.123456), "1.12");
    assert_eq!(money_f32(0.123456789), "0.123 456 7");
    assert_eq!(money_f64(0.123456789), "0.123 456 789");
    assert_eq!(money_f64(0.123456789012345), "0.123 456 789 012 345");
}
