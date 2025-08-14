# Input Lib

A basic library for reading input with prompts and parsing.

This library provides a simple way to read input from the user, optionally 
displaying a prompt, and parsing the input into various types.

This library is [part of a RFC process](https://github.com/rust-lang/rfcs/pull/3799).

## Features
- **Prompting**: Optionally print a prompt before reading input.
- **Parsing**: Automatically parse input into types that implement `FromStr`.
- **Error Handling**: Unified error type for I/O errors, parse errors, and EOF.

## Usage

### Simple Input

```rust
use std::error::Error;
use input_lib::input;

fn main() -> Result<(), Box<dyn Error>> {
    let name: String = input!("Enter your name: ")?;
    println!("Hello, {}!", name);

    Ok(())
}
```

### Custom Parsing

```rust
#![allow(dead_code)]
use std::str::FromStr;
use input_lib::{input};

#[derive(Debug)]
struct Price {
    currency: String,
    amount: f64,
}

impl FromStr for Price {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("String must have two parts".to_string());
        }
        let currency = parts[0].to_string();
        let amount = parts[1].parse().unwrap();
        Ok(Price { currency, amount })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let price: Price = input!("Please enter a price: ")?; 
            // This could fail for example if the input is reading from a pipe and 
            // we delete the file whose descriptor is being read while the
            // program is running
    
    println!("{price:#?}!");

    Ok(())
}
```

### Comprehensive Error Handling

```rust
use input_lib::{input, InputError};


fn main() {
    let number: i32 = match input!("Please enter a number: ") {
        Ok(value) => value,
        Err(e) => match e {
            InputError::Eof => {
                println!("End of input reached.");
                return;
            },
            InputError::Parse(_) => {
                println!("Failed to parse the input.");
                return;
            },
            InputError::Io(_) => {
                println!("An I/O error occurred.");
                return;
            },
        },
    };

    println!("You entered: {}", number);
}
```


