/*
 amount: the original amount in float
 max_decimal_places: number of decimal digits to keep for amounts > 0
 zero_comma_decimal_places: number of decimal digits to keep for amount < 1
 */
pub fn endinero(amount: f32, max_decimal_places: u16, zero_comma_decimal_places: u16) -> String {
    let int_part = amount as i32;
    // we'll need to deal with thousand separators.
    if int_part.abs() > 1000 {

    }

    // let's keep all the decimals up to max_decimal_places
    if int_part.abs() == 0 {

    }

    // TODO: Take care of the sign
    "1.234.567,123.456.78".to_string()
}
