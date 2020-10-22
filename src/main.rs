use std::io;
//use clap::{App};

enum Denomination {
    sats,
    bits,
    mbtc,
    btc,
}

fn main() {
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");
    
    // create mutable variable named user_denom and bind to a new, empty instance of a String

        let mut user_denom = String::new();
        let mut desired_denom: String = String::new();
        let mut user_amount: String = String::new();

        use crate::Denomination::*;

        impl Denomination {
            fn as_str(&self) -> &'static str {
                match *self {
                    Denomination::sats => "sats",
                    Denomination::bits => "bits",
                    Denomination::mbtc => "mbtc",
                    Denomination::btc => "btc"
                }
            }
        }
    /* call stdin function from the io module & call read_line method on the standard input handle to get input from user
    also passes one argument to read_line - &mut VARIABLE
    read_line takes standard input and places it into a string */

        println!("Your denomination:");
        io::stdin().read_line(&mut user_denom).expect("Please enter sats, bits, mbtc, or btc");

        match user_denom.as_str() {
            "sats" => println!("Enter amount in Sats:"),
            "bits" => println!("Enter amount in Bits:"),
            "mbtc" => println!("Enter amount in mBTC:"),
            "btc" => println!("Enter amount in BTC:"),
            _ => panic!("Unexpected entry!"),
        }

        /*
        if {

            println!("Desired Denomination:");
            io::stdin().read_line(&mut desired_denom).expect("Please enter sats, bits, mbtc, or btc");

            if desired_denom.contains("sats") || desired_denom.contains("bits") || desired_denom.contains("mbtc") || desired_denom.contains("btc") {
                
                println!("Enter amount:");
                io::stdin().read_line(&mut user_amount).expect("Please enter the amount, for example: 0.042");

                let user_amount: f64 = match user_amount.parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                };

                let output = user_amount / 10.0;
                println!("Value: {:}", output);
                
            } else {
                println!("Invalid input");
            }
            
        } else {
            println!("Invalid input");
        } */
}