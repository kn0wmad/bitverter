use std::{ io, rc::Rc };

// Take a value and apply the correct conversion to sats, based on denomination provided

fn from_denom_to_sats(starting_denom: &str, val: Rc<String>) -> Option<Rc<String>> {
    match starting_denom.to_lowercase().trim() {
        "sats" => Some(val),
        "bits" => Some(bitverter_lib::bits_to_sats(val)),
        "mbtc" => Some(bitverter_lib::mbtc_to_sats(val)),
        "btc" => Some(bitverter_lib::btc_to_sats(val)),
        _ => None
    }
}

// Take sats calculated and denomination provided and return a string based on denomination provided

fn to_string_from_denom(desired_denom: &str, sats: Rc<String>) -> Option<String> {
    match desired_denom.to_lowercase().trim() {
        "sats" => Some(format!("{} {}", sats, desired_denom)),
        "bits" => Some(format!("{} {}", bitverter_lib::sats_to_bits(sats), desired_denom)),
        "mbtc" => Some(format!("{} {}", bitverter_lib::sats_to_mbtc(sats), desired_denom)),
        "btc" => Some(format!("{} {}", bitverter_lib::sats_to_btc(sats), desired_denom)),
        _ => None
    }
}

fn main() {

// Welcome user
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");
    println!("Привет! Добро пожаловать в Bitverter!!\n");

// Create mutable variable starting_denom and bind to a new, empty instance of a String
// Call read_line method to get user input, pass the &mut starting_denom argument to read_line
// read_line takes standard input and places it into a String - trim the result and print back to user
    let mut starting_denom = String::new();

    println!("Enter your current denomination (Sats, Bits, mBTC, or BTC): ");
    io::stdin().read_line(&mut starting_denom).expect("Failed to read your input");

    let starting_denom = starting_denom.trim();
    println!("You have selected: {:?}\n", starting_denom);

// Repeat above process to request current value in the user's starting denomination, then perform sats conversion
    let mut starting_value = String::new();

    println!("Input your current value in {:?}", starting_denom.to_lowercase());
    io::stdin().read_line(&mut starting_value).expect("Failed to read your input");

    let starting_value = Rc::new(starting_value.trim().to_owned());
    let starting_sats = from_denom_to_sats(&starting_denom, Rc::clone(&starting_value))
        .expect("Please enter a valid denomination");

// Repeat process from above again to get user's desired denomination
    let mut desired_denom = String::new();

    println!("Input your desired denomination (Sats, Bits, mBTC, or BTC): ");
    io::stdin().read_line(&mut desired_denom).expect("Failed to read your input");

    let desired_denom = desired_denom.trim();
    println!("You have selected {:?}\n", desired_denom);

// Take string of calculated value and return the full conversion to the user

    let desired_value = to_string_from_denom(&desired_denom, Rc::clone(&starting_sats))
        .expect("Please enter a valid denomination");
    println!("{} {} = {}", starting_value, starting_denom, desired_value);
}
