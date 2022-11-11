# EnDinero

```rust
use endinero::endinero;
let as_formatted_string_in_spanish = endinero::endinero(some_float_value, max_decimals, '.', ',', ' ');
```

```rust
// convenience shorthand for formatting f32 in spanish
use endinero::dinero_f32;
let as_formatted_string_in_spanish = endinero::dinero_f32(some_f32);
```

```rust
// convenience shorthand for formatting f64 in spanish
use endinero::dinero_f64;
let as_formatted_string_in_spanish = endinero::dinero_f64(some_f64);
```

Convert a float value to a user-friendly number, most often used to represent money amounts in Spanish (perhaps German and some other European locales work the same as in spanish)

```
10.1             -> "10,1"
10.111           -> "10,111"
10000000.21      -> "10.000.000,21"
0.22233344455566 -> "0,222.333.444.555.66"
```

## Limits

Integer part works on up to `10^15` (Quadrillions/_Billardos_) when used with `f64` inputs 

`1234567890123456.0 -> 1.234.567.890.123.456.0`

## Installation

Add to your Cargo.toml
```toml
endinero = "0.1.1"
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
