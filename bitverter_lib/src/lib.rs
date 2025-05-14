//! Bitverter Conversion Library
use phf::phf_map;

/// Map of denominations to their corresponding values
static CONV: phf::Map<&'static str, f64> = phf_map! {
    "sats"   => 1e-8,
    "finneys"=> 1e-7,
    "ubtc"   => 1e-6,
    "bits"   => 1e-5,
    "mbtc"   => 1e-3,
    "cbtc"   => 1e-2,
    "btc"    => 1e0,
    "dabtc"  => 1e2,
    "kbtc"   => 1e3,
};

/// Conversion function for changing denominations
pub fn convert(input_quantity: f64, base_unit: &str, quote_unit: &str) -> f64 {
    input_quantity * (CONV[&base_unit] / CONV[&quote_unit])
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_convert() {
        // test some units
        assert_eq!(1.0, convert(100000000.0, "sats", "btc"));
        assert_eq!(1.0, convert(10.0, "sats", "finneys"));
        assert_eq!(1000000.0, convert(1.0, "btc", "ubtc"));
        assert_eq!(100000.0, convert(1.0, "mbtc", "sats"));
        assert_eq!(1.0, convert(10000000.0, "finneys", "btc"));
        assert_eq!(0.001, convert(1.0, "ubtc", "mbtc"));

        // other tests
        assert_eq!(1.52135088, convert(152135088.0, "sats", "btc"));

        // all the bitcoin ever
        assert_eq!(2100000000000000.0, convert(21000000.0, "btc", "sats"));
    }
}
