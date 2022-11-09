# EnDinero

```rust
use endinero::endinero;

...

let as_formatted_string_in_spanish = endinero::endinero(some_float_value, max_decimals);
```

Convert a float value to a user friendly number, most often used to represent money amounts in Spanish (perhaps German and some other European locales work the same as in spanish)

```
10.1             -> "10,1"
10.111           -> "10,111"
10000000.21      -> "10.000.000,21"
0.22233344455566 -> "0.222.333.444.555.66"
```

## Installation

Add to your Cargo.toml
```toml
endinero = "0.1.1"
```
