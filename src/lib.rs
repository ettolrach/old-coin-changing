use std::{fmt::Display, iter::Sum};

const CURRENCIES_AS_HALFPENCE: [usize; 11] = [1, 2, 6, 12, 24, 48, 60, 120, 480, 2400, 4800];

/// The currencies that were in use before decimalisation. Note, the crown wasn't used that much in
/// real day-to-day life.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Halfpenny,
    Penny,
    Threepence,
    Sixpence,
    Shilling,
    Florin,
    HalfCrown,
    Crown,
    OnePound,
    FivePound,
    TenPound,
}

impl Currency {
    /// Convert from halfpence value. For example, 48 gives [`Currency::Florin`]. If no currency
    /// matches the given halfpence value, [`None`] is returned.
    pub fn from_halfpence(halfpence: usize) -> Option<Self> {
        match halfpence {
            1 => Some(Self::Halfpenny),
            2 => Some(Self::Penny),
            6 => Some(Self::Threepence),
            12 => Some(Self::Sixpence),
            24 => Some(Self::Shilling),
            48 => Some(Self::Florin),
            60 => Some(Self::HalfCrown),
            120 => Some(Self::Crown),
            480 => Some(Self::OnePound),
            2400 => Some(Self::FivePound),
            4800 => Some(Self::TenPound),
            _ => None,
        }
    }
}

/// Like a wallet, a container for various coins and notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wallet {
    pub halfpence: usize,
    pub pennies: usize,
    pub threepence: usize,
    pub sixpence: usize,
    pub shillings: usize,
    pub florins: usize,
    pub half_crowns: usize,
    pub crowns: usize,
    pub one_pounds: usize,
    pub five_pounds: usize,
    pub ten_pounds: usize,
}

impl Wallet {
    /// Add a coin or note to the wallet.
    pub fn add_currency(&mut self, currency: Currency) {
        match currency {
            Currency::Halfpenny => self.halfpence += 1,
            Currency::Penny => self.pennies += 1,
            Currency::Threepence => self.threepence += 1,
            Currency::Sixpence => self.sixpence += 1,
            Currency::Shilling => self.shillings += 1,
            Currency::Florin => self.florins += 1,
            Currency::HalfCrown => self.half_crowns += 1,
            Currency::Crown => self.crowns += 1,
            Currency::OnePound => self.one_pounds += 1,
            Currency::FivePound => self.five_pounds += 1,
            Currency::TenPound => self.ten_pounds += 1,
        }
    }

    /// Remove a coin or note to the wallet.
    pub fn remove_currency(&mut self, currency: Currency) {
        match currency {
            Currency::Halfpenny => self.halfpence -= 1,
            Currency::Penny => self.pennies -= 1,
            Currency::Threepence => self.threepence -= 1,
            Currency::Sixpence => self.sixpence -= 1,
            Currency::Shilling => self.shillings -= 1,
            Currency::Florin => self.florins -= 1,
            Currency::HalfCrown => self.half_crowns -= 1,
            Currency::Crown => self.crowns -= 1,
            Currency::OnePound => self.one_pounds -= 1,
            Currency::FivePound => self.five_pounds -= 1,
            Currency::TenPound => self.ten_pounds -= 1,
        }
    }

    /// Get the halfpence value of the wallet.
    pub fn to_halfpence(&self) -> usize {
        self.halfpence
        + self.pennies * 2
        + self.threepence * 6
        + self.sixpence * 12
        + self.shillings * 24
        + self.florins * 48
        + self.half_crowns * 60
        + self.crowns * 120
        + self.one_pounds * 480
        + self.five_pounds * 2400
        + self.ten_pounds * 4800
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            halfpence: 0,
            pennies: 0,
            threepence: 0,
            sixpence: 0,
            shillings: 0,
            florins: 0,
            half_crowns: 0,
            crowns: 0,
            one_pounds: 0,
            five_pounds: 0,
            ten_pounds: 0,
        }
    }
}

impl From<Price> for Wallet {
    fn from(value: Price) -> Self {
        let mut to_return = Self::default();
        let currencies: Vec<Currency> = coin_change(&CURRENCIES_AS_HALFPENCE, value.to_halfpence())
            .iter()
            .map(|&c| Currency::from_halfpence(c).unwrap())
            .collect();
        for currency in currencies {
            to_return.add_currency(currency);
        }
        to_return
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price {
    pounds: usize,
    shillings: usize, 
    halfpence: usize,
}

impl Price {
    pub fn new(pounds: usize, shillings: usize, halfpence: usize) -> Self {
        Price { pounds, shillings, halfpence }
    }

    /// Convert a halfpence value to a more readable price.
    pub fn from_halfpence(halfpence: usize) -> Self {
        let halfpence = halfpence % 24;
        let temp = halfpence / 24;
        let shillings = temp % 20;
        Price { pounds: temp / 20, shillings, halfpence }
    }

    /// Convert a pence value to a more readable price. Internally calls [`Price::from_halfpence`].
    pub fn from_pence(pence: usize) -> Self {
        Self::from_halfpence(pence * 2)
    }

    /// Convert price to halfpence value.
    pub fn to_halfpence(&self) -> usize {
        self.pounds * 480 + self.shillings * 24 + self.halfpence
    }

    /// Add a price to this one.
    pub fn add(&self, rhs: Price) -> Self {
        let mut temp = self.halfpence + rhs.halfpence;
        let halfpence = temp % 24;
        temp = temp / 24 + self.shillings + rhs.shillings;
        let shillings = temp % 20;
        Price { pounds: temp / 20 + self.pounds + rhs.pounds, shillings, halfpence }
    }
}

impl Default for Price {
    fn default() -> Self {
        Price { pounds: 0, shillings: 0, halfpence: 0 }
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "£{} {}s {}d", self.pounds, self.shillings, self.halfpence / 2)
    }
}

/// Construct a [`Price`] using the more commonly used slash notation.
/// 
/// # Examples
/// 
/// ```
/// use coin_changing::{ Price, price };
/// 
/// assert_eq!(price!(5/2).to_string(), "£0 5s 2d".to_string());
/// assert_eq!(price!(1/4/-).to_string(), "£1 4s 0d".to_string());
/// assert_eq!(price!(-/2).to_string(), "£0 0s 2d".to_string());
/// ```
#[macro_export]
macro_rules! price {
    ( 0 ) => {
        Price::new(0, 0, 0)
    };
    ( -/-/- ) => {
        Price::new(0, 0, 0)
    };
    ( -/$pence:literal ) => {
        Price::new(0, 0, $pence * 2)
    };
    ( $shillings:literal/- ) => {
        Price::new(0, $shillings, 0)
    };
    ( -/-/$pence:literal ) => {
        Price::new(0, 0, $pence * 2)
    };
    ( -/$shillings:literal/- ) => {
        Price::new(0, $shilligns, 0)
    };
    ( $pounds:literal/-/- ) => {
        Price::new($pounds, $shilligns, 0)
    };
    ( $pounds:literal/$shillings:literal/- ) => {
        Price::new($pounds, $shillings, 0)
    };
    ( $shillings:literal/$pence:literal ) => {
        Price::new(0, $shillings, $pence * 2)
    };
    ( $pounds:literal/$shillings:literal/- ) => {
        Price::new($pounds, $shilligns, 0)
    };
    ( $pounds:literal/$shillings:literal/$pence:literal ) => {
        Price::new($pounds, $shillings, $pence * 2)
    };
}

impl From<Currency> for Price {
    fn from(value: Currency) -> Price {
        match value {
            Currency::Halfpenny => Price { pounds: 0, shillings: 0, halfpence: 1 },
            Currency::Penny => Price { pounds: 0, shillings: 0, halfpence: 2 },
            Currency::Threepence => Price { pounds: 0, shillings: 0, halfpence: 3 },
            Currency::Sixpence => Price { pounds: 0, shillings: 0, halfpence: 6 },
            Currency::Shilling => Price { pounds: 0, shillings: 1, halfpence: 0 },
            Currency::Florin => Price { pounds: 0, shillings: 2, halfpence: 0 },
            Currency::HalfCrown => Price { pounds: 0, shillings: 2, halfpence: 6 },
            Currency::Crown => Price { pounds: 0, shillings: 5, halfpence: 0 },
            Currency::OnePound => Price { pounds: 1, shillings: 0, halfpence: 0 },
            Currency::FivePound => Price { pounds: 5, shillings: 0, halfpence: 0 },
            Currency::TenPound => Price { pounds: 10, shillings: 0, halfpence: 0 },
        }
    }
}

impl Sum for Price {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Price::default(), |acc, p| {
            acc.add(p)
        })
    }
}

impl From<Wallet> for Price {
    fn from(value: Wallet) -> Self {
        Self::from_halfpence(value.to_halfpence())
    }
}

/// Calculates change for a given target.
/// 
/// # Example
/// 
/// ```
/// use coin_changing::coin_change;
/// 
/// assert_eq!(coin_change(&[1, 5, 7], 20), vec![7, 7, 5, 1]);
/// ```
pub fn coin_change(coins: &[usize], target: usize) -> Vec<usize> {
    // Taken from my lecture notes. Eventually I'll clean it up :P
    if target == 0 {
        return vec![];
    }
    let mut v = target;
    let mut k = coins.len();
    #[allow(non_snake_case)]
    let mut C: Vec<usize> = (0..=v).collect();
    #[allow(non_snake_case)]
    let mut P = vec![0; v + 1];
    #[allow(non_snake_case)]
    let mut S = vec![0; k];
    let mut to_return: Vec<usize> = Vec::new();
    
    C[1] = 1;
    for w in 2..=v {
        for i in 0..k {
            if (coins[i] <= w) && (C[w - coins[i]] + 1 < C[w]) {
                C[w] = C[w - coins[i]] + 1;
                P[w] = i;
            }
        }
    }
    let mut i;
    while v > 0 {
        i = P[v];
        S[i] += 1;
        v -= coins[i];
    }
    while k > 0 {
        while S[k - 1] > 0 {
            to_return.push(coins[k - 1]);
            S[k - 1] -= 1;
        }
        k -= 1;
    }
    to_return
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn price_sum() {
        let prices: Vec<Price> = vec![price!(5/2), price!(1/3), price!(1/17/5), price!(2/1)];
        let expected_sum = price!(2/5/11);
        assert_eq!(prices.iter().copied().sum::<Price>(), expected_sum);
    }

    #[test]
    fn add_and_change() {
        let price1 = price!(3/16/11);
        let price2 = price!(5/15/10);
        let total = price!(9/12/9);
        assert_eq!(price1.add(price2), total);
        let change = Wallet {
            halfpence: 0,
            pennies: 0,
            threepence: 1,
            sixpence: 0,
            half_crowns: 1,
            crowns: 2,
            shillings: 0,
            florins: 0,
            one_pounds: 4,
            five_pounds: 1,
            ten_pounds: 0,
        };
        assert_eq!(Wallet::from(total), change);
    }
}
