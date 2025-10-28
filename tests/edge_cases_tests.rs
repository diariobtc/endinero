use endinero::*;

/// Edge case tests for boundary conditions and special values

/// Test zero in various forms
#[test]
fn test_zero_variations() {
    assert_eq!(dinero_f64(0.0), "0,0");
    assert_eq!(dinero_f64(-0.0), "0,0");
    assert_eq!(money_f64(0.0), "0.0");
    assert_eq!(money_f64(-0.0), "0.0");
}

/// Test numbers very close to zero
#[test]
fn test_near_zero_positive() {
    // Sub-unity numbers use higher decimal precision
    let result = dinero_f64(0.001);
    assert!(result.contains("001"));
}

/// Test numbers exactly at powers of 10
#[test]
fn test_powers_of_ten() {
    assert_eq!(dinero_f64(1.0), "1,0");
    assert_eq!(dinero_f64(10.0), "10,0");
    assert_eq!(dinero_f64(100.0), "100,0");
    assert_eq!(dinero_f64(1000.0), "1.000,0");
    assert_eq!(dinero_f64(10000.0), "10.000,0");
}

/// Test numbers at thousands boundaries
#[test]
fn test_thousand_boundaries() {
    assert_eq!(dinero_f64(1000.0), "1.000,0");
    assert_eq!(dinero_f64(10000.0), "10.000,0");
    assert_eq!(dinero_f64(100000.0), "100.000,0");
}

/// Test small decimal values using endinero with limited precision
#[test]
fn test_small_decimal_values() {
    // dinero_f64 uses 17 decimals for < 1, so use endinero_f64 with limited decimals
    assert_eq!(endinero_f64(0.1, 2, 1, '.', ',', ' '), "0,1");
    assert_eq!(endinero_f64(0.2, 2, 1, '.', ',', ' '), "0,2");
    assert_eq!(endinero_f64(0.5, 2, 1, '.', ',', ' '), "0,5");
}

/// Test separator characters work correctly
#[test]
fn test_custom_separator_characters() {
    let amount = 1234567.5;

    let result = endinero_f64(amount, 1, 0, ',', '.', ' ');
    assert_eq!(result, "1,234,567.5");
}

/// Test with different decimal place requirements
#[test]
fn test_variable_decimal_places() {
    let amount = 1.23456789;

    // For amounts >= 1, uses max_decimal_places
    assert_eq!(endinero_f64(amount, 1, 4, ',', '.', ' '), "1.2");
    assert_eq!(endinero_f64(amount, 2, 4, ',', '.', ' '), "1.23");
    assert_eq!(endinero_f64(amount, 3, 4, ',', '.', ' '), "1.234");
}

/// Test different behavior for amounts > 1 vs < 1
#[test]
fn test_different_decimal_places_by_magnitude() {
    // For amounts >= 1: use max_decimal_places (2)
    assert_eq!(
        endinero_f64(123.123456, 2, 6, '.', ',', ' '),
        "123,12"
    );

    // For amounts < 1: use zero_comma_decimal_places (6)
    let result = endinero_f64(0.123456, 2, 6, '.', ',', ' ');
    assert!(result.contains("123 456"));
}

/// Test f32 precision limits
#[test]
fn test_f32_precision_limits() {
    let precise: f32 = 0.12345678f32;
    let result = dinero_f32(precise);
    // Verify it handles f32 precision gracefully without panic
    assert!(!result.is_empty());
}

/// Test negative amounts with high decimal values
#[test]
fn test_negative_with_decimals() {
    let result = dinero_f64(-0.123456);
    assert!(result.starts_with('-'));
}

/// Test that thousands separators are placed correctly
#[test]
fn test_thousands_separator_placement() {
    let result = dinero_f64(1234567890.0);
    assert!(result.contains("1.234.567.890"));
}

/// Test consistency of formatting across repeated calls
#[test]
fn test_formatting_consistency() {
    let amount = 1234567.89;
    let result1 = dinero_f64(amount);
    let result2 = dinero_f64(amount);
    let result3 = dinero_f64(amount);

    assert_eq!(result1, result2);
    assert_eq!(result2, result3);
}

/// Test that negative sign is at the beginning
#[test]
fn test_negative_sign_placement() {
    let negative = dinero_f64(-1234567.89);
    assert!(negative.starts_with('-'));

    let negative_decimal = dinero_f64(-0.89);
    assert!(negative_decimal.starts_with('-'));
}

/// Test complex currency scenarios
#[test]
fn test_real_world_amounts() {
    assert_eq!(dinero_f64(10.0), "10,0");
    assert_eq!(dinero_f64(100.0), "100,0");
    assert_eq!(dinero_f64(1000.0), "1.000,0");
    assert_eq!(dinero_f64(10000.0), "10.000,0");
}

/// Test that different separator characters work
#[test]
fn test_custom_separators() {
    let amount = 1234567.5;

    // Comma for thousands, dot for decimal
    let result1 = endinero_f64(amount, 1, 0, ',', '.', ' ');
    assert_eq!(result1, "1,234,567.5");

    // Space for thousands, comma for decimal
    let result2 = endinero_f64(amount, 1, 0, ' ', ',', '.');
    assert_eq!(result2, "1 234 567,5");
}

/// Test amount = 1 with various formats
#[test]
fn test_one_with_different_formats() {
    assert_eq!(dinero_f64(1.0), "1,0");
    assert_eq!(money_f64(1.0), "1.0");
}
