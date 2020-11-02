#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::io;
//use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Denomination {
    sats,
    bits,
    mBTC,
    BTC
}

impl FromStr for Denomination {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("Debugging... {:?}", s);
        match s {
            "sats\n" => Ok(Self::sats),
            "bits\n" => Ok(Self::bits),
            "mBTC\n" => Ok(Self::mBTC),
            "BTC\n" => Ok(Self::BTC),
            _ => Err(())
        }
    }
}

fn main() {

// Welcome user
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");

    loop {
    // Create mutable variable starting_denom and bind to a new, empty instance of a String
        let mut starting_denom = String::new();
        
        println!("Enter your current denomination (sats, bits, mBTC, or BTC): ");
        
    // Call read_line method to get user input, pass the &mut starting_denom argument to read_line
    // read_line takes standard input and places it into a String
        io::stdin().read_line(&mut starting_denom).expect("Failed to read input");
        
        let starting_denom = Denomination::from_str(starting_denom.as_str());

        println!("You have selected: {:?}", starting_denom);

        // if starting_denom = Denomination::sats {
            
        // }

    // Create mutable variable desired_denom, bind to new, empty instance of a String, 
    // and request input from user
        /*let mut desired_denom = String::new();

        println!("Enter your current denomination (sats, bits, mBTC, or BTC): ");

        io::stdin().read_line(&mut desired_denom).expect("Failed to read your input");

        println!("You have entered {}", desired_denom);*/
    }
}