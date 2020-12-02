use std::rc::Rc;
use rust_decimal::prelude::*;

pub fn div(input: Rc<String>, scale: u32) -> Rc<String> {
    let from_string = Decimal::from_str(&input);
    let num = Decimal::new(1, scale);

    match from_string {
        Ok(divisor) => {
            Rc::new(divisor.checked_div(num).unwrap().to_string())
        }
        Err(_) => Rc::new("Error".to_owned())
    }
}

pub fn mul(input: Rc<String>, scale: u32) -> Rc<String> {
    let from_string = Decimal::from_str(&input);
    let num = Decimal::new(1, scale);

    match from_string {
        Ok(muliplier) => {
            Rc::new(muliplier.checked_mul(num).unwrap().to_string())
        }
        Err(_) => Rc::new("Error".to_owned())
    }
}

pub fn sats_to_btc(input: Rc<String>) -> Rc<String> {
    mul(input, 8) // input * 100000000
}

pub fn sats_to_mbtc(input: Rc<String>) -> Rc<String> {
    mul(input, 5) // input * 100000
}

pub fn sats_to_bits(input: Rc<String>) -> Rc<String> {
    mul(input, 2) // input * 100
}

pub fn bits_to_sats(input: Rc<String>) -> Rc<String> {
    div(input, 2) // input / 100
}

pub fn mbtc_to_sats(input: Rc<String>) -> Rc<String> {
    div(input, 5) // input / 100000
}

pub fn btc_to_sats(input: Rc<String>) -> Rc<String> {
    div(input, 8) // input / 100000000
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
