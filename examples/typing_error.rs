use std::num::ParseIntError;
use input_lib::{input, InputError};

fn main() {
    let age: Result<u8, InputError<ParseIntError>> = input!("Enter your age: ");

    let age = age.expect("The age is required, please enter the number");

    println!("Your age is {age}");
}