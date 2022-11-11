///
/// Copyright [2022] DiarioBitcoin.com, Angel Leon
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
/// https://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
///
use log::info;

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
    let result = format!("{}{}{}", int_part, radix_character, dec_part);
    info!("endinero_f64(amount={}, max_decimal_places={}, zero_comma_decimal_places={}, thousands_separator='{}', radix_character='{}', decimals_separator='{}') -> {}",
    amount,
    max_decimal_places,
    zero_comma_decimal_places,
    thousands_separator,
    radix_character,
    decimals_separator,
    result);

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
    let result = format!("{}{}{}", int_part, radix_character, dec_part);
    info!("endinero_f32(amount={}, max_decimal_places={}, zero_comma_decimal_places={}, thousands_separator='{}', radix_character='{}', decimals_separator='{}') -> {}",
    amount,
    max_decimal_places,
    zero_comma_decimal_places,
    thousands_separator,
    radix_character,
    decimals_separator,
    result);

    result
}

/// Format `f64` input into a spanish money format, max 2 decimal places if amount > 0, up to 17 decimal places for amount < 1. Thousands separator: `.`, Radix character: `,`, Decimals separator: ` `
pub fn dinero_f64(amount: f64) -> String {
    endinero_f64(amount, 2, 17, '.', ',', ' ')
}

/// Format `f32` input into a spanish money format, max 2 decimal places if amount > 0, up to 17 decimal places for amount < 1. Thousands separator: `.`, Radix character: `,`, Decimals separator: ` `
pub fn dinero_f32(amount: f64) -> String {
    endinero_f64(amount, 2, 7, '.', ',', ' ')
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
    let int_part = amount.abs() as i64;
    let unsigned_int_digits_str = format!("{}", int_part);
    let mut formatted_int_digits: Vec<char> = Vec::new();
    let mut digits_added = 0;
    for c in unsigned_int_digits_str.chars().rev() {
        if digits_added > 0 && digits_added % 3 == 0 {
            formatted_int_digits.push(thousands_separator.clone());
        }
        formatted_int_digits.push(c);
        digits_added += 1;
    }
    if amount < 0f64 {
        formatted_int_digits.push('-');
    }
    formatted_int_digits.reverse();
    let result = formatted_int_digits.iter().collect::<String>();
    info!(
        "endinero::integer_part_f64({},'{}') -> {}",
        amount, thousands_separator, result
    );
    result
}

fn integer_part_f32(amount: f32, thousands_separator: char) -> String {
    let int_part = amount.abs() as i32;
    let unsigned_int_digits_str = format!("{}", int_part);
    let mut formatted_int_digits: Vec<char> = Vec::new();
    let mut digits_added = 0;
    for c in unsigned_int_digits_str.chars().rev() {
        if digits_added > 0 && digits_added % 3 == 0 {
            formatted_int_digits.push(thousands_separator.clone());
        }
        formatted_int_digits.push(c);
        digits_added += 1;
    }
    if amount < 0f32 {
        formatted_int_digits.push('-');
    }
    formatted_int_digits.reverse();
    let result = formatted_int_digits.iter().collect::<String>();
    info!(
        "endinero::integer_part_f32({},'{}') -> {}",
        amount, thousands_separator, result
    );
    result
}

fn decimal_part_f64(amount: f64, total_decimals: u16, decimals_separator: char) -> String {
    let dec_digits_str = format!("{}", amount.abs() % 1.0f64).replace("0.", "");
    info!(
        "decimal_part_f64(amount={}): dec_digits_str = {}",
        amount, dec_digits_str
    );

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
    let result = formatted_dec_digits.iter().collect::<String>();
    info!(
        "endinero::decimal_part_f64(amount={}, total_decimals={}, decimals_separator='{}') -> {}",
        amount, total_decimals, decimals_separator, result
    );
    result
}

fn decimal_part_f32(amount: f32, total_decimals: u16, decimals_separator: char) -> String {
    let dec_digits_str = format!("{}", amount.abs() % 1.0).replace("0.", "");
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
    let result = formatted_dec_digits.iter().collect::<String>();
    info!(
        "endinero::decimal_part_f32(amount={}, total_decimals={}, decimals_separator='{}') -> {}",
        amount, total_decimals, decimals_separator, result
    );
    result
}

#[test]
fn integer_part_tests() {
    env_logger::try_init().unwrap();

    info!("Zeroes");
    assert_eq!(integer_part_f64(0.0, '.'), "0");
    assert_eq!(integer_part_f64(-0.0, '.'), "0");
    assert_eq!(integer_part_f64(-0.0001, '.'), "-0");

    info!("Units [1-9]");
    assert_eq!(integer_part_f64(1.0, '.'), "1");
    assert_eq!(integer_part_f64(-1.0, '.'), "-1");

    info!("Tens");
    assert_eq!(integer_part_f64(12.0, '.'), "12");
    assert_eq!(integer_part_f64(-12.0, '.'), "-12");

    info!("Hundreds");
    assert_eq!(integer_part_f64(123.0, '.'), "123");
    assert_eq!(integer_part_f64(-123.0, '.'), "-123");

    info!("Thousands");
    assert_eq!(integer_part_f64(1234.0, '.'), "1.234");
    assert_eq!(integer_part_f64(-1234.0, '.'), "-1.234");

    info!("Tens of thousands");
    assert_eq!(integer_part_f64(12345.0, '.'), "12.345");
    assert_eq!(integer_part_f64(-12345.0, '.'), "-12.345");

    info!("Hundreds of thousands");
    assert_eq!(integer_part_f64(123456.0, '.'), "123.456");
    assert_eq!(integer_part_f64(-123456.0, '.'), "-123.456");

    info!("Millions");
    assert_eq!(integer_part_f64(1234567.0, '.'), "1.234.567");
    assert_eq!(integer_part_f64(-1234567.0, '.'), "-1.234.567");

    info!("Tens of millions");
    assert_eq!(integer_part_f64(12345678.0, '.'), "12.345.678");
    assert_eq!(integer_part_f64(-12345678.0, '.'), "-12.345.678");

    info!("Hundreds of millions");
    assert_eq!(integer_part_f64(123456789.0, '.'), "123.456.789");
    assert_eq!(integer_part_f64(-123456789.0, '.'), "-123.456.789");

    info!("Billions / Millardos");
    assert_eq!(integer_part_f64(1234567890.0, '.'), "1.234.567.890");
    assert_eq!(integer_part_f64(-1234567890.0, '.'), "-1.234.567.890");

    info!("Tens of billions / Decenas de millardos");
    assert_eq!(integer_part_f64(12345678901.0, '.'), "12.345.678.901");
    assert_eq!(integer_part_f64(-12345678901.0, '.'), "-12.345.678.901");

    info!("Hundreds of billions / Cientos de millardos");
    assert_eq!(integer_part_f64(123456789012.0, '.'), "123.456.789.012");
    assert_eq!(integer_part_f64(-123456789012.0, '.'), "-123.456.789.012");

    info!("Trillions (10^12) / Billones");
    assert_eq!(integer_part_f64(1234567890123.0, '.'), "1.234.567.890.123");
    assert_eq!(
        integer_part_f64(-1234567890123.0, '.'),
        "-1.234.567.890.123"
    );

    info!("Tens of trillions (10^13) / Decenas de billones");
    assert_eq!(
        integer_part_f64(12345678901234.0, '.'),
        "12.345.678.901.234"
    );
    assert_eq!(
        integer_part_f64(-12345678901234.0, '.'),
        "-12.345.678.901.234"
    );

    info!("Hundreds of trillions (10^14) / Cientos de billones");
    assert_eq!(
        integer_part_f64(123456789012345.0, '.'),
        "123.456.789.012.345"
    );
    assert_eq!(
        integer_part_f64(-123456789012345.0, '.'),
        "-123.456.789.012.345"
    );

    info!("Quadrillion (10^15) / Billardo");
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
    env_logger::try_init().unwrap();
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
    env_logger::try_init().unwrap();
    assert_eq!(
        endinero_f64(1234567.456, 2, 4, '.', ',', ' '),
        "1.234.567,45"
    );
    assert_eq!(
        endinero_f64(-1234567.456789, 2, 4, '.', ',', ' '),
        "-1.234.567,45"
    );

    assert_eq!(
        endinero_f64(-1234567.456789, // amount
                     4, // decimals for numbers > 0
                     4, // decimals for numbers < 0
                     '.', // thousands separator
                     ',', // radix separator
                     ' '), // decimals separator
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
    assert_eq!(dinero_f64(x), "123.456.789.012.345,12");

    // should break here
    assert_ne!(dinero_f64(x), "1.234.567.890.123.456,12");
    assert_eq!(dinero_f64(0.12345678912345678), "0,123 456 789 123 456 78");

    assert_eq!(dinero_f32(10.111), "10,11");
    assert_eq!(dinero_f32(-10.111), "-10,11");
    assert_eq!(dinero_f32(10000000.1232456), "10.000.000,12");
    assert_eq!(dinero_f32(0.2223334445556), "0,222 333 4");

    assert_eq!(dinero_f32(10000000.12324567), "10.000.000,12");
    assert_eq!(dinero_f32(0.1234567), "0,123 456 7");
}
