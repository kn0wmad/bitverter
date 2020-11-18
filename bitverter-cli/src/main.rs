use std::{ io, rc::Rc };
use rust_decimal::prelude::*;

// Take a value and apply the correct conversion to sats, based on denomination provided

fn from_denom_to_sats(starting_denom: &str, val: Rc<String>) -> Option<Rc<String>> {
    match starting_denom.trim() {
        "sats" => Some(val),
        "bits" => Some(bits_to_sats(val)),
        "mBTC" => Some(mbtc_to_sats(val)),
        "BTC" => Some(btc_to_sats(val)),
        _ => None
    }
}

// Take sats calculated and denomination provided and return a string based on denomination provided

fn to_string_from_denom(desired_denom: &str, sats: Rc<String>) -> Option<String> {
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
// Call read_line method to get user input, pass the &mut starting_denom argument to read_line
// read_line takes standard input and places it into a String - trim the result and print back to user
    let mut starting_denom = String::new();
    
    println!("Enter your current denomination (sats, bits, mBTC, or BTC): ");
    io::stdin().read_line(&mut starting_denom).expect("Failed to read your input");
  
    let starting_denom = starting_denom.trim();
    println!("You have selected: {:?}", starting_denom);

// Repeat above process to request current value in the user's starting denomination, then perform sats conversion
    let mut starting_value = String::new();

    println!("Input your current value in {:?}", starting_denom);
    io::stdin().read_line(&mut starting_value).expect("Failed to read your input");
    
    let starting_value = Rc::new(starting_value.trim().to_owned());
    let starting_sats = from_denom_to_sats(&starting_denom, Rc::clone(&starting_value)).unwrap();
    // let starting_sats_string = Decimal::from_str(starting_sats).unwrap();

// OUTPUT CODE

// Repeat process from above (lines 46 - 56) to get user's desired denomination
    let mut desired_denom = String::new();

    println!("Input your desired denomination (sats, bits, mBTC, or BTC): ");
    io::stdin().read_line(&mut desired_denom).expect("Failed to read your input");

    let desired_denom = desired_denom.trim();
    println!("You have selected {:?}", desired_denom);

    println!("{}", to_string_from_denom(&desired_denom, Rc::clone(&starting_sats)).unwrap());
    println!("{} {} = {} {}", starting_value, starting_denom, starting_sats, desired_denom);
}

// Calculation functions
fn div(input: Rc<String>, scale: u32) -> Rc<String> {
    let from_string = Decimal::from_str(&input);
    let divisor = Decimal::new(1, scale);

    match from_string {
        Ok(num) => {
            Rc::new(divisor.checked_div(num).unwrap().to_string())
        }
        Err(_) => Rc::new("Error".to_owned())
    }
    // input / 100000000
}

fn mul(input: Rc<String>, scale: u32) -> Rc<String> {
    let from_string = Decimal::from_str(&input);
    let multiplier = Decimal::new(1, scale);

    match from_string {
        Ok(num) => {
            Rc::new(num.checked_mul(multiplier).unwrap().to_string())
        }
        Err(_) => Rc::new("Error".to_owned())
    }
    // input / 100000000
}

fn sats_to_btc(input: Rc<String>) -> Rc<String> {
    div(input, 8)
}

fn sats_to_mbtc(input: Rc<String>) -> Rc<String> {
    div(input, 5)
    // input / 100000
}

fn sats_to_bits(input: Rc<String>) -> Rc<String> {
    div(input, 2)
    // input / 100
}

fn bits_to_sats(input: Rc<String>) -> Rc<String> {
    mul(input, 2)
    // input * 100
}

fn mbtc_to_sats(input: Rc<String>) -> Rc<String> {
    mul(input, 5)
    // input * 100000
}

fn btc_to_sats(input: Rc<String>) -> Rc<String> {
    mul(input, 8)
    // input * 100000000
}