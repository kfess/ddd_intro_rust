#![allow(dead_code)]

use anyhow::Error;

#[derive(Debug, PartialEq, Eq)]
struct Money {
    amount: usize,
    currency: String,
}

impl Money {
    fn new(amount: usize, currency: &str) -> Self {
        Money {
            amount,
            currency: currency.to_string(),
        }
    }

    // Add trait では、Result 型を返すことができないため、独自に実装
    fn try_add(&self, other: &Self) -> Result<Self, Error> {
        if &self.currency == &other.currency {
            Ok(Money {
                amount: self.amount + other.amount,
                currency: self.currency.clone(),
            })
        } else {
            Err(anyhow::Error::msg("通貨が異なるため、計算できません。"))
        }
    }

    // Add trait では、Result 型を返すことができないため、独自に実装
    fn try_minus(&self, other: &Self) -> Result<Self, Error> {
        if &self.currency == &other.currency {
            Ok(Money {
                amount: self.amount - other.amount,
                currency: self.currency.clone(),
            })
        } else {
            Err(Error::msg("通貨が異なるため、計算できません。"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_money() {
        let money_1 = Money::new(1000, "JPY");
        let money_2 = Money::new(2000, "JPY");
        let sum = money_1.try_add(&money_2);
        assert!(sum.is_ok());
        assert_eq!(sum.unwrap().amount, 3000);
    }

    fn minus_two_money() {
        let money_1 = Money::new(2000, "JPY");
        let money_2 = Money::new(1000, "JPY");
        let sum = money_1.try_minus(&money_2);
        assert!(sum.is_ok());
        assert_eq!(sum.unwrap().amount, 1000);
    }

    fn different_currencies() {
        let money_1 = Money::new(1000, "JPY");
        let money_2 = Money::new(2000, "USD");
        let sum = money_1.try_add(&money_2);
        assert!(sum.is_err());

        let sum2 = money_1.try_minus(&money_2);
        assert!(sum2.is_err());
    }
}
