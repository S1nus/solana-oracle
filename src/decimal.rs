use std::fmt;

#[derive(PartialEq, Default, Debug)]
pub struct Decimal {
    // 17
    pub val: u128, // 16
    pub scale: u8, // 1
}

const fn generate_powers_of_10() -> [u128; Decimal::MAX_PRECISION as usize] {
    let mut powers_10: [u128; Decimal::MAX_PRECISION as usize] =
        [0u128; Decimal::MAX_PRECISION as usize];
    let mut exponent = 0;
    while exponent < (Decimal::MAX_PRECISION) {
        powers_10[exponent as usize] = 10u128.pow(exponent as u32);
        exponent += 1;
    }
    powers_10
}

pub const POWERS_10: [u128; Decimal::MAX_PRECISION as usize] = generate_powers_of_10();

impl Decimal {
    pub const MAX_PRECISION: u8 = 38;

    pub fn new(val: u128, scale: u8) -> Self {
        Self { val, scale }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6}", f64::from(self))
    }
}

impl From<&Decimal> for f64 {
    fn from(x: &Decimal) -> Self {
        (x.val as f64) / (POWERS_10[x.scale as usize] as f64)
    }
}
