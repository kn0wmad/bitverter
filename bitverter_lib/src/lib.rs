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
    use super::*;

    #[test]
    fn check_divide() {
        assert_eq!(Rc::new("152135088".to_string()), btc_to_sats(Rc::new("1.52135088".to_string())));
        assert_eq!(Rc::new("100000000".to_string()), btc_to_sats(Rc::new("1".to_string())));
        // assert_eq!(Rc::new("5786134457").to_string()), btc_to_
    }

    #[test]
    fn check_multiply() {
        assert_eq!(Rc::new("1.000000000".to_string()), sats_to_btc(Rc::new("100000000.0".to_string())));
        assert_eq!(Rc::new("1.52135088".to_string()), sats_to_btc(Rc::new("152135088".to_string())));
        assert_eq!(Rc::new("0.10000".to_string()), sats_to_mbtc(Rc::new("10000".to_string())));
    }
}
