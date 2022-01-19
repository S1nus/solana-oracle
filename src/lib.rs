pub mod decimal;

use decimal::Decimal;

pub fn get_price_with_tolerance() -> Decimal {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
