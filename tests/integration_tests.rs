use endinero::*;

/// Tests for endinero_f64 - the main public API
#[test]
fn endinero_f64_basic_formatting() {
    assert_eq!(
        endinero_f64(1234567.456, 2, 4, '.', ',', ' '),
        "1.234.567,45"
    );
}

#[test]
fn endinero_f64_negative_numbers() {
    assert_eq!(
        endinero_f64(-1234567.456, 2, 4, '.', ',', ' '),
        "-1.234.567,45"
    );
}

#[test]
fn endinero_f64_sub_unity_numbers() {
    assert_eq!(endinero_f64(0.456, 2, 4, '.', ',', ' '), "0,456");
    assert_eq!(
        endinero_f64(-1234567.456789, 4, 4, '.', ',', ' '),
        "-1.234.567,456 7"
    );
}

#[test]
fn endinero_f64_decimal_separator_variations() {
    assert_eq!(endinero_f64(0.456789, 2, 4, '.', ',', '.'), "0,456.7");
    assert_eq!(endinero_f64(0.456789, 2, 5, '.', ',', '.'), "0,456.78");
}

#[test]
fn endinero_f64_unicode_separators() {
    assert_eq!(
        endinero_f64(0.456789, 2, 5, '.', '🔻', '.'),
        "0🔻456.78"
    );
    assert_eq!(
        endinero_f64(0.456789, 2, 6, '.', '🔻', '.'),
        "0🔻456.789"
    );
}

#[test]
fn endinero_f64_zero_values() {
    assert_eq!(endinero_f64(0.0, 2, 4, '.', ',', ' '), "0,0");
    assert_eq!(endinero_f64(-0.0, 2, 4, '.', ',', ' '), "0,0");
}

#[test]
fn endinero_f64_very_large_numbers() {
    let x: f64 = 123456789012345.1234;
    assert_eq!(endinero_f64(x, 2, 4, '.', ',', ' '), "123.456.789.012.345,12");
}

#[test]
fn endinero_f64_high_decimal_precision() {
    assert_eq!(
        endinero_f64(0.12345678912345678, 2, 17, '.', ',', ' '),
        "0,123 456 789 123 456 78"
    );
}

/// Tests for endinero_f32 - f32 variant
#[test]
fn endinero_f32_basic_formatting() {
    let result = endinero_f32(1234.125f32, 2, 4, ',', '.', ' ');
    assert_eq!(result, "1,234.12");
}

#[test]
fn endinero_f32_zero_values() {
    assert_eq!(endinero_f32(0.0f32, 2, 4, ',', '.', ' '), "0.0");
}

#[test]
fn endinero_f32_negative_numbers() {
    let result = endinero_f32(-1234.125f32, 2, 4, ',', '.', ' ');
    assert_eq!(result, "-1,234.12");
}

/// Tests for dinero_f64 - Spanish formatting
#[test]
fn dinero_f64_basic() {
    assert_eq!(dinero_f64(0.1234567), "0,123 456 7");
}

#[test]
fn dinero_f64_large_amounts() {
    assert_eq!(dinero_f64(10000000.1232456), "10.000.000,12");
}

#[test]
fn dinero_f64_negative() {
    assert_eq!(dinero_f64(-1234.5), "-1.234,5");
}

#[test]
fn dinero_f64_zero() {
    assert_eq!(dinero_f64(0.0), "0,0");
}

#[test]
fn dinero_f64_very_large() {
    let x: f64 = 123456789012345.1234;
    assert_eq!(dinero_f64(x), "123.456.789.012.345,12");
}

#[test]
fn dinero_f64_high_precision() {
    assert_eq!(dinero_f64(0.12345678912345678), "0,123 456 789 123 456 78");
}

#[test]
fn dinero_f64_rounding() {
    // Should not equal a wrong formatting
    let x: f64 = 123456789012345.1234;
    assert_ne!(dinero_f64(x), "1.234.567.890.123.456,12");
}

/// Tests for dinero_f32 - Spanish formatting with f32
#[test]
fn dinero_f32_basic() {
    assert_eq!(dinero_f32(10.111f32), "10,11");
}

#[test]
fn dinero_f32_negative() {
    assert_eq!(dinero_f32(-10.111f32), "-10,11");
}

#[test]
fn dinero_f32_zero() {
    assert_eq!(dinero_f32(0.0f32), "0,0");
}

#[test]
fn dinero_f32_sub_unity() {
    assert_eq!(dinero_f32(0.2223334445556f32), "0,222 333 4");
    assert_eq!(dinero_f32(0.1234567f32), "0,123 456 7");
}

#[test]
fn dinero_f32_large_integer_precision_loss() {
    // f32 loses precision with large integers
    assert_eq!(dinero_f32(11111111.1234f32), "11.111.111,0");
}

/// Tests for money_f64 - US/English formatting
#[test]
fn money_f64_basic() {
    let result = money_f64(1234.5);
    assert!(result.contains("1,234"));
}

#[test]
fn money_f64_large_amounts() {
    let result = money_f64(1234567.5);
    assert!(result.contains("1,234,567"));
}

#[test]
fn money_f64_zero() {
    assert_eq!(money_f64(0.0), "0.0");
}

#[test]
fn money_f64_negative() {
    let result = money_f64(-1234.5);
    assert!(result.starts_with('-'));
    assert!(result.contains("1,234"));
}

#[test]
fn money_f64_decimal_variations() {
    assert_eq!(money_f64(0.123456789), "0.123 456 789");
    assert_eq!(money_f64(0.123456789012345), "0.123 456 789 012 345");
}

/// Tests for money_f32 - US/English formatting with f32
#[test]
fn money_f32_basic() {
    assert_eq!(money_f32(1234.125f32), "1,234.12");
}

#[test]
fn money_f32_zero() {
    assert_eq!(money_f32(0.0f32), "0.0");
}

#[test]
fn money_f32_negative() {
    assert_eq!(money_f32(-1234.125f32), "-1,234.12");
}

#[test]
fn money_f32_sub_unity() {
    assert_eq!(money_f32(0.123456f32), "0.123 456");
}

/// Integration tests combining multiple locales
#[test]
fn test_different_locales() {
    let amount = 1234567.5;

    // Spanish
    let spanish = dinero_f64(amount);
    assert!(spanish.contains("1.234.567"));
    assert!(spanish.contains(','));

    // US/English
    let english = money_f64(amount);
    assert!(english.contains("1,234,567"));
    assert!(english.contains('.'));

    // German (using . for thousands and , for decimal)
    let german = endinero_f64(amount, 2, 17, '.', ',', ' ');
    assert!(german.contains("1.234.567"));
}

/// Test consistency between f32 and f64 variants
#[test]
fn test_f32_f64_consistency() {
    let amount_64: f64 = 1234.5;
    let amount_32: f32 = 1234.5;

    let result_64 = dinero_f64(amount_64);
    let result_32 = dinero_f32(amount_32);

    // Results should be identical for representable values
    assert_eq!(result_64, result_32);
}

/// Test edge case: amount is exactly 1
#[test]
fn edge_case_amount_one() {
    assert_eq!(dinero_f64(1.0), "1,0");
    assert_eq!(money_f64(1.0), "1.0");
}

/// Test edge case: amount between 0 and 1
#[test]
fn edge_case_sub_unity() {
    let result_dinero = dinero_f64(0.5);
    assert!(result_dinero.contains("0,"));
    let result_money = money_f64(0.5);
    assert!(result_money.contains("0."));
}

/// Test edge case: very small amounts
#[test]
fn edge_case_very_small() {
    let result1 = dinero_f64(0.01);
    assert!(result1.contains("01"));
    let result2 = dinero_f64(0.001);
    assert!(result2.contains("001"));
}
