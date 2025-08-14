use std::error::Error;
use input_lib::input;

fn main() -> Result<(), Box<dyn Error>> {
    let name: String = input!("Enter your name: ")?;
    println!("Hello, {}!", name);

    Ok(())
}