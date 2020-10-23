#![allow(dead_code)]
#![allow(non_camel_case_types)]

/*#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;*/

use std::io;
//use clap::{App};

// Create and implement an enumeration Denomination, with 4 possible values.  custom_derive to use as type &str

enum Denomination {
    sats(String),
    bits(String),
    mBTC(String),
    BTC(String)
}

impl Denomination {
         fn which_denom(&self) -> &String {
            match *self {
                Denomination::sats(ref msg) |
                Denomination::bits(ref msg) |
                Denomination::mBTC(ref msg) |
                Denomination::BTC(ref msg) => msg
                }
            }
        }

fn main() {
    // Welcome user
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");

    // Create mutable variable named user_denom and bind to a new, empty instance of a String
    let mut user_denom = Denomination::sats(String::from("sats"));
    //println!("{}", user_denom.which_denom());

    //user_denom = Denomination::bits(String::from("bits"));
    //println!("{}", user_denom.which_denom());

    //user_denom = Denomination::mBTC(String::from("mBTC"));
    //println!("{}", user_denom.which_denom());

    //user_denom = Denomination::BTC(String::from("BTC"));
    //println!("{}", user_denom.which_denom());

    // Check if user input matches any value in enum Denomination
    /*match io::stdin().read_line(&mut user_denom) {
        if {
            Ok(String) "sats" => println!("Did you provide a valid denomination? {}", user_denom.which_denom())
        }
    }*/
    //println!("Please enter your current denomination - sats, bits, mBTC, or BTC:");
}

    //println!("Is user_denom a Denomination? {}", user_denom.which_denom());
    
    
/*
    assert_eq!("sats", user_denom.as_str());

    match io::stdin().read_line(&mut user_denom){
        Ok(_) => (
            println!("Successs! You have selected: {}", user_denom.to_lowercase())
        ),
        Err(e) => println!("Error. Please enter sats, bits, mBTC, or BTC: {}", e)
    }

        let mut desired_denom: String = String::new();
        let mut user_amount: String = String::new();

        use crate::Denomination::*;

    /* call stdin function from the io module & call read_line method on the standard input handle to get input from user
    also passes one argument to read_line - &mut VARIABLE
    read_line takes standard input and places it into a string */

        
        io::stdin().read_line(&mut user_denom).expect("Please enter sats, bits, mBTC, or BTC");

        match user_denom.as_str() {
            "sats" => println!("Enter amount in sats:"),
            "bits" => println!("Enter amount in bits:"),
            "mBTC" => println!("Enter amount in mBTC:"),
            "BTC" => println!("Enter amount in BTC:"),
            _ => panic!("Unexpected entry!"),
        }
*/
        /*
        if {

            println!("Desired Denomination:");
            io::stdin().read_line(&mut desired_denom).expect("Please enter sats, bits, mBTC, or BTC");

            if desired_denom.contains("sats") || desired_denom.contains("bits") || desired_denom.contains("mBTC") || desired_denom.contains("BTC") {
                
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
