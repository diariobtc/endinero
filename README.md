# EnDinero

```rust
// convenience shorthand for formatting f32 in spanish, it will truncate the decimals we don't want
// outputs 2 decimals for amounts > 0, and 7 decimals for amounts < 1.
// This is how we need it for our crypto market pages when dealing with some crapcoins that trade in very small amounts
use endinero::dinero_f32;
assert_eq!(endinero::dinero_f32(0.1234567), "0,123 456 7");
```

```rust
// convenience shorthand for formatting f64 in spanish
use endinero::dinero_f64;
assert_eq!(endinero::dinero_f64(0.22233344455566), "0,222 333 444 555 66");
```

Convert a float value to a user-friendly number, most often used to represent money amounts in Spanish (perhaps German and some other European locales work the same as in spanish)

```rust
assert_eq!(endinero::dinero_f32(10.111), "10,11");
assert_eq!(endinero::dinero_f32(-10.111), "-10,11");
assert_eq!(endinero::dinero_f32(0.1234567), "0,123 456 7");
assert_eq!(endinero::dinero_f64(0.2223334445556677), "0,222 333 444 555 666 77");
```

If you need to specify thousand separators, decimal separators, radix character, how many decimals to show for numbers > 0 or for numbers < 1
use the `endinero::endinero` function that receives all the parameters.

```rust
use endinero::endinero;
assert_eq!(
    endinero_f64(-1234567.456789, // amount 
                 4, // decimals for numbers > 0
                 4, // decimals for numbers < 0
                 '.', // thousands separator
                 ',', // radix separator
                 ' '), // decimals separator
    "-1.234.567,456 7"
);
```
If you also need to format US dollars for American english we've included `endinero::money_f32` and `endinero::money_f64` formatters.

```rust
// it will show only two decimals for numbers > 0
assert_eq!(money_f32(12345678.123456), "12,345,678.12");
assert_eq!(money_f32(1.123456),"1.12");

// it will show up to 7 decimals correctly for numbers < 1 if you pass an `f32` value
assert_eq!(money_f32(0.123456789),"0.123 456 7");

// we recommend you work with f64 if you're dealing with very small amounts
assert_eq!(money_f64(0.123456789),"0.123 456 789");

// up to 15 decimals correctly with f64
assert_eq!(money_f64(0.123456789012345),"0.123 456 789 012 345");
```

## Limits

Integer part works on up to `10^14` Hundreds of trillions (Cientos de billones) when used with `f64` inputs 

```rust
assert_eq!(dinero_f64(123456789012345.1234) ,"123.456.789.012.345,12");
```

<hr>

Decimal part works on up to 7 decimals when used with `f32` inputs

```rust
assert_eq!(endinero::dinero_f32(0.2223334445556) ,"0,222 333 4");
```

<hr>

For f64 if the number is less than 1, it can format up to 17 decimal places
```rust
assert_eq!(dinero_f64(0.12345678912345678) ,"0,123 456 789 123 456 78");
```

Note: As integer portions get larger, there might be rounding issues with the decimals

## Installation

Add to your Cargo.toml
```toml
endinero = "0.1.4"
```

## License
Copyright [2022] DiarioBitcoin.com, Angel Leon

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
