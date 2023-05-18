use std::io::{stdin, stdout, Write};

/* bitverter - A user-driven Bitcoin unit converter */

// helper for getting units/denomination
fn get_denom_from_input() -> String {
    let mut denom = String::new();
    loop {
        print!(
            "1) sats\t2) finneys\n\
	     3) bits\t4) uBTC\n\
	     5) mBTC\t6) BTC\n\n\
	     => "
        );
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut denom)
            .expect("Failed to read your input");
        denom = match denom.trim().to_lowercase().as_str() {
            "1" | "s" | "sat" | "sats" => "sats".to_owned(),
            "2" | "f" | "finneys" => "finneys".to_owned(),
            "3" | "bit" => "bits".to_owned(),
            "4" | "u" | "ubtc" | "ubtcs" => "ubtc".to_owned(),
            "5" | "m" | "mbtc" | "mbtcs" => "mbtc".to_owned(),
            "6" | "b" | "btc" | "btcs" => "btc".to_owned(),
            _ => {
                println!("You must select an option listed below!\n");
                continue;
            }
        };
        println!("You have selected: {}\n", denom);
        break;
    }

    denom
}

// helper for getting a quantity of units
fn get_quantity(units: &str) -> f64 {
    let mut s = String::new();

    println!("Input your current value in {:?}", units.to_lowercase());
    stdin()
        .read_line(&mut s)
        .expect("\nFailed to read your input!");

    let quantity: f64 = s.trim().parse().unwrap();

    quantity
}

// return a quote from user-provided (base,quote,quantity)
fn main() {
    // welcome user
    println!("Hello!  Welcome to Bitverter!!");
    println!("Hola! Bienvenidos a Bitverter!!");
    println!("Привет! Добро пожаловать в Bitverter!!\n");

    // get base unit
    println!("Enter your current denomination");
    let base_unit = get_denom_from_input();

    // get quantity
    let quantity = get_quantity(&base_unit);

    // get quote unit
    println!("\nInput your desired denomination");
    let quote_unit = get_denom_from_input();

    // convert
    let quote = bitverter_lib::convert(quantity, &base_unit, &quote_unit);

    println!("{} {} = {} {}", quantity, &base_unit, quote, &quote_unit);
}
