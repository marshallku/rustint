
# Rustint

Rustint is a Rust library for working with RGB colors. It provides a simple and efficient way to create, manipulate, and convert colors in Rust applications.

## Features

- Create RGB colors
- Interpolate between two colors
- Convert hexadecimal color strings to RGB
- Convert RGB colors to hexadecimal strings
- Error handling for invalid color formats

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustint = "0.1.0"
```

## Usage

### Creating a Color

```rust
use rustint::Color;

let red = Color::new(255, 0, 0);
let blue = Color::new(0, 0, 255);

// or

let red = Color::try_from("#FF0000").unwrap();
let blue = Color::try_from("#0000FF").unwrap();
```

### Interpolating Between Colors

```rust
let red = Color::new(255, 0, 0);
let blue = Color::new(0, 0, 255);
let purple = red.interpolate(&blue, 50.0);
println!("Purple: {}", purple); // Outputs: Purple: #7F007F
```

### Converting to Hex String

```rust
let color = Color::new(128, 64, 32);
println!("Hex: {}", color); // Outputs: Hex: #804020
```

## Error Handling

The library provides custom error types for handling invalid color formats:

```rust
use rustint::{Color, ColorError};

let result = Color::try_from("#INVALID");
match result {
    Ok(color) => println!("Valid color: {}", color),
    Err(ColorError::InvalidFormat) => println!("Invalid format"),
    Err(ColorError::InvalidHexValue) => println!("Invalid hex value"),
}
```

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
