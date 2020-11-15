#![allow(dead_code)]

use std::io;

// #[derive(Debug)]
// enum Denomination {
//     Sats,
//     Bits,
//     MilliBTC,
//     BTC // -> // BTC(u64)
// }

// impl FromStr for Denomination {
//     type Err = ();
//     fn from_str(s: &str, v: &str) -> Result<u64, u64::Err> {
//         // println!("Debugging... {:?}", s);
//         match s.trim() {
//             "sats" => Ok(),
//             "bits" => Ok(),
//             "mBTC" => Ok(),
//             "BTC" => Ok(),
//             _ => Err(())
//         }
//     }
// }

fn from_denom_to_sats(starting_denom: &str, val: &str) -> Option<u64> {
    // println!("Debugging... {:?}", s);

    let v = val.parse::<u64>().unwrap();

    match starting_denom.trim() {
        "sats" => Some(v),
        "bits" => Some(bits_to_sats(v)),
        "mBTC" => Some(mbtc_to_sats(v)),
        "BTC" => Some(btc_to_sats(v)),
        _ => None
    }
}

fn to_string_from_denom(desired_denom: &str, sats: u64) -> Option<String> {
    // let v = val.parse::<u64>().unwrap();

    // Some(format!("{}{}", s))

    match desired_denom.trim() {
        "sats" => Some(format!("{} {}", sats, desired_denom)),
        "bits" => Some(format!("{} {}", sats_to_bits(sats), desired_denom)),
        "mBTC" => Some(format!("{} {}", sats_to_mbtc(sats), desired_denom)),
        "BTC" => Some(format!("{} {}", sats_to_btc(sats), desired_denom)),
        _ => None
    }
}

fn main() {

// INPUT CODE

// Welcome user
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");

// Create mutable variable starting_denom and bind to a new, empty instance of a String
    let mut starting_denom = String::new();
    
// Call read_line method to get user input, pass the &mut starting_denom argument to read_line
// read_line takes standard input and places it into a String
    println!("Enter your current denomination (sats, bits, mBTC, or BTC): ");
    io::stdin().read_line(&mut starting_denom).expect("Failed to read your input");
    let starting_denom = starting_denom.trim();

// Bind starting_denom to correct Denomination enum variant    
    println!("You have selected: {:?}", starting_denom);

// Request current value in the user's starting denomination
    let mut starting_value = String::new();

    println!("Input your current value in {:?}", starting_denom);
    io::stdin().read_line(&mut starting_value).expect("Failed to read your input");
    
    let starting_value = starting_value.trim();
    let starting_sats = from_denom_to_sats(&starting_denom, &starting_value).unwrap();

// OUTPUT CODE

    // notes
    // keep in mind, sats will only be u64
    // whenever there's other denominations, they should only be strings that are output from a function

    // match for starting input string to convert to sats

// Repeat process to get user's desired denomination and bind to correct Denomination enum variant
    let mut desired_denom = String::new();

    println!("Input your desired denomination: ");
    io::stdin().read_line(&mut desired_denom).expect("Failed to read your input");

    let desired_denom = desired_denom.trim();
    println!("You have selected {:?}", desired_denom);

    println!("{}", to_string_from_denom(&desired_denom, starting_sats).unwrap());
    // println!("{:?} {:?} = {:?} {:?}", starting_value, starting_denom, converted_value, desired_denom)
}

// Calculation functions
fn sats_to_btc(input: u64) -> u64 {
    input / 100000000
}

fn sats_to_mbtc(input: u64) -> u64 {
    input / 100000
}

fn sats_to_bits(input: u64) -> u64 {
    input / 100
}

fn bits_to_sats(input: u64) -> u64 {
    input * 100
}

fn mbtc_to_sats(input: u64) -> u64 {
    input * 100000
}

fn btc_to_sats(input: u64) -> u64 {
    input * 100000000
}