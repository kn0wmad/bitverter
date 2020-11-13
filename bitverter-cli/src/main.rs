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
        // println!("Debugging... {:?}", s);
        match s.trim() {
            "sats" => Ok(Self::sats),
            "bits" => Ok(Self::bits),
            "mBTC" => Ok(Self::mBTC),
            "BTC" => Ok(Self::BTC),
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
        

    // Call read_line method to get user input, pass the &mut starting_denom argument to read_line
    // read_line takes standard input and places it into a String
        println!("Enter your current denomination (sats, bits, mBTC, or BTC): ");
        io::stdin().read_line(&mut starting_denom).expect("Failed to read your input");
        
    // Bind starting_denom to correct Denomination enum variant    
        let starting_denom = Denomination::from_str(starting_denom.as_str());

        println!("You have selected: {:?}", starting_denom);

    // Request current value in the user's starting denomination
        let mut starting_value = String::new();

        println!("Input your current value in {:?}", starting_denom);
        io::stdin().read_line(&mut starting_value).expect("Failed to read your input");

    // Repeat process to get user's desired denomination and bind to correct Denomination enum variant
        let mut desired_denom = String::new();

        println!("Input your desired denomination: ");
        io::stdin().read_line(&mut desired_denom).expect("Failed to read your input");

        let desired_denom = Denomination::from_str(desired_denom.as_str());
        println!("You have selected {:?}", desired_denom);

    // Calculate conversion and return converted result to user
    // Possibilites are BTC, mBTC, bits, or sats -> BTC, mBTC, bits, or sats
        let starting_value: f64 = starting_value.trim().parse().unwrap();
        
        if starting_denom == Self::BTC && desired_denom == Self::BTC {
            let converted_value:f64 = starting_value;
            println!("{} BTC = {} BTC", starting_value, converted_value);
        }   
        
        else if starting_denom == Self::BTC && desired_denom == Self::mBTC {
            let converted_value:f64 = starting_value / 1000.0;
            println!("{} BTC = {} mBTC", starting_value, converted_value);
        }   
        
        else if starting_denom == Self::BTC && desired_denom == Self::bits {
            let converted_value:f64 = starting_value / 1000000.0;
            println!("{} BTC = {} bits", starting_value, converted_value);
        }   
        
        else if starting_denom == Self::BTC && desired_denom == Self::sats {
            let converted_value:f64 = starting_value * 100000000.0;
            println!("{} BTC = {} sats", starting_value, converted_value);
        }   
        
        else if starting_denom == Self::mBTC && desired_denom == Self::BTC {
            let converted_value:f64 = starting_value / 1000.0;
            println!("{} mBTC = {} BTC", starting_value, converted_value);
        }

        else if starting_denom == Self::mBTC && desired_denom == Self::mBTC {
            let converted_value:f64 = starting_value;
            println!("{} mBTC = {} mBTC", starting_value, converted_value);
        }  

        else if starting_denom == Self::mBTC && desired_denom == Self::bits {
            let converted_value:f64 = starting_value * 1000.0;
            println!("{} mBTC = {} bits", starting_value, converted_value);
        }  

        else if starting_denom == Self::mBTC && desired_denom == Self::sats {
            let converted_value:f64 = starting_value * 100000.0;
            println!("{} mBTC = {} sats", starting_value, converted_value);
        }
        
        else if starting_denom == Self::bits && desired_denom == Self::BTC {
            let converted_value:f64 = starting_value / 1000000.0;
            println!("{} bits = {} BTC", starting_value, converted_value);
        } 

        else if starting_denom == Self::bits && desired_denom == Self::mBTC {
            let converted_value:f64 = starting_value / 1000.0;
            println!("{} bits = {} mBTC", starting_value, converted_value);
        }

        else if starting_denom == Self::bits && desired_denom == Self::bits {
            let converted_value:f64 = starting_value;
            println!("{} bits = {} bits", starting_value, converted_value);
        }

        else if starting_denom == Self::bits && desired_denom == Self::sats {
            let converted_value:f64 = starting_value * 100.0;
            println!("{} bits = {} sats", starting_value, converted_value);
        }
        
        else if starting_denom == Self::sats && desired_denom == Self::BTC {
            let converted_value:f64 = starting_value / 100000000.0;
            println!("{} sats = {} BTC", starting_value, converted_value);
        }

        else if starting_denom == Self::sats && desired_denom == Self::mBTC {
            let converted_value:f64 = starting_value / 100000.0;
            println!("{} sats = {} mBTC", starting_value, converted_value);
        }   

        else if starting_denom == Self::sats && desired_denom == Self::bits {
            let converted_value:f64 = starting_value / 100.0;
            println!("{} sats = {} bits", starting_value, converted_value);
        }   

        else if starting_denom == Self::sats && desired_denom == Self::sats {
            let converted_value:f64 = starting_value;
            println!("{} sats = {} sats", starting_value, converted_value);
        }   
        
        else    {
            return
        }

        break
    }
}