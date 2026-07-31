pub type Currency = u32;
pub type Day = u32;

#[derive(Debug)]
pub struct Wallet {
    balance: Currency,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Goody {
    Keyboard,
}

impl Goody {
    pub const ALL: [Self; 1] = [Self::Keyboard];
}

impl Wallet {
    pub fn new(balance: Currency) -> Wallet {
        Wallet { balance }
    }

    pub fn new_random() -> Wallet {
        let balance = rand::random_range(1000..2000);
        Wallet { balance: balance }
    }

    pub fn pay(&mut self, amount: Currency) {
        self.balance += amount
    }

    pub fn get_balance(&self) -> Currency {
        self.balance
    }
}
