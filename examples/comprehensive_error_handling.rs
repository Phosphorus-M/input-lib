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