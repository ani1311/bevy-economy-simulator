use std::{any::Any, collections::HashMap};

use bevy::prelude::*;

use crate::components::{
    Offer,
    common::{Currency, Day, Goody, Wallet},
};

#[derive(Component, Debug)]
pub struct Producer {
    pub wallet: Wallet,
    pub products: HashMap<Goody, ProductLine>,
}

#[derive(Debug)]
pub struct ProductLine {
    pub inventory: u32,
    pub selling_price: Currency,
    pub active_order: Option<ProductionOrder>,
}

#[derive(Debug)]
pub struct ProductionOrder {
    pub quantity: u32,
    pub cost: Currency,
    pub delivery_date: Day,
}

impl Producer {
    pub fn new(seed_capital: Currency) -> Producer {
        Producer {
            wallet: Wallet::new(seed_capital),
            products: HashMap::new(),
        }
    }

    pub fn new_random() -> Producer {
        let products = Goody::ALL
            .into_iter()
            .map(|g| (g, ProductLine::new_random()))
            .collect();

        Producer {
            wallet: Wallet::new_random(),
            products: products,
        }
    }

    pub fn get_offers(&self, seller: Entity) -> Vec<Offer> {
        self.products
            .iter()
            .filter_map(|(g, pl)| {
                (pl.inventory > 0).then_some(Offer {
                    seller: seller,
                    goody: g.clone(),
                    price: pl.selling_price,
                })
            })
            .collect()
    }

    pub fn can_sell(&self, offer: Offer) -> bool {
        self.products
            .get(&offer.goody)
            .is_some_and(|pl| pl.inventory > 0 && pl.selling_price == offer.price)
    }

    pub fn sell(&mut self, offer: Offer) {}

    pub fn setup_shop(&mut self) {
        //we update prices here
    }

    pub fn wallet_balance(&self) -> Currency {
        self.wallet.get_balance()
    }

    pub fn keyboard_inventory(&self) -> Option<u32> {
        self.products.get(&Goody::Keyboard).map(|p| p.selling_price)
    }
    pub fn keyboard_price(&self) -> Option<u32> {
        self.products.get(&Goody::Keyboard).map(|p| p.selling_price)
    }
}

impl ProductLine {
    pub fn new_random() -> Self {
        Self {
            inventory: rand::random_range(5000..50000),
            selling_price: rand::random_range(10..20),
            active_order: None,
        }
    }
}
