#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::io;
use std::str::FromStr;
//use clap::{App};

// Create and implement an enumeration Denomination, with 4 possible values.  custom_derive to use as type &str

enum Denomination {
    sats,
    bits,
    mBTC,
    BTC
}

impl FromStr for Denomination {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sats" => Self::sats,
            "bits" => Self::bits,
            "mBTC" => Self::mBTC,
            "BTC" => Self::BTC
        }
    }
}

fn main() {
    // Welcome user
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");

    loop {
        println!("Enter your current denomination (sats, bits, mBTC, or BTC): ");

<<<<<<< HEAD:src/main.rs
        // Create mutable variable user_denom and bind to a new, empty instance of a String
        let mut user_denom = String::new();
        
        // Call read_line method to get user input, pass the &mut user_denom argument to read_line
        // read_line takes standard input and places it into a String
        io::stdin().read_line(&mut user_denom)
            .expect("Failed to read your input");
        
        // bind user_denom to expression std::env::args().next().unwrap().parse().unwrap()
        let user_denom: Denomination = std::env::args().next().unwrap().parse().unwrap();
=======
    // create mutable variable named btc and bind to a new, empty instance of a String
        let mut btc = String::new();

    // call stdin function from the io module
        io::stdin().read_line(&mut btc).expect("Failed to read your input");

    // calls read_line method on the standard input handle to get input from user
    // also passes one argument to read_line - &mut btc
    // read_line takes standard input and places it into a string

    // bind btc to expression btc.trim.parse - trim will remove whitespace
    // u32 can only contain numerical characters
        let btc: u32 = match btc.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
>>>>>>> ec86f9f383f200cd081b507698287d9e1343874c:bitverter-cli/src/main.rs
    }
}
