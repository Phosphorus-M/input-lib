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