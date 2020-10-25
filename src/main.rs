#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::io;
use std::str::FromStr;
//use clap::{App};

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
            "BTC" => Self::BTC,
        }
    }
}

fn main() {
    // Welcome user
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");

    loop {
        println!("Enter your current denomination (sats, bits, mBTC, or BTC): ");

        // Create mutable variable starting_denom and bind to a new, empty instance of a String
        let mut starting_denom = String::new();
        
        // Call read_line method to get user input, pass the &mut starting_denom argument to read_line
        // read_line takes standard input and places it into a String
        io::stdin().read_line(&mut starting_denom)
            .expect("Failed to read your input");
        
        // bind starting_denom to expression std::env::args().next().unwrap().parse().unwrap()
        let starting_denom: Denomination = std::env::args().next().unwrap().parse().unwrap();
    }
}