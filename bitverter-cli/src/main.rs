// bitverter

use std::io;
//use clap::{App};


fn main() {
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");

    loop{
        println!("Choose denomination to convert from (Please enter: sats, bits, mbtc, btc):");

        println!("Choose denomination to convert to (Please enter: sats, bits, mbtc, btc):");

        println!("Enter amount:");

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
    }
}
