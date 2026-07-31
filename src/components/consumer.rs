use std::collections::HashMap;

use bevy::prelude::*;

use crate::components::{
    Offer,
    common::{Currency, Day, Goody, Wallet},
};

#[derive(Component, Debug)]
pub struct Consumer {
    wallet: Wallet,
    job: Job,
    desires: HashMap<Goody, Desire>,
}

#[derive(Debug)]
pub enum PayFrequency {
    Monthly,
}

impl PayFrequency {
    fn days(&self) -> Day {
        match self {
            PayFrequency::Monthly => 30,
        }
    }
}

#[derive(Debug)]
pub struct Job {
    salary: Currency,
    pay_frequency: PayFrequency,
}

impl Job {
    fn _new(salary: Currency, pay_frequency: PayFrequency) -> Job {
        Job {
            salary: salary,
            pay_frequency: pay_frequency,
        }
    }

    fn new_random() -> Job {
        let salary = rand::random_range(100..150);
        Job {
            salary: salary,
            pay_frequency: PayFrequency::Monthly,
        }
    }

    fn is_pay_day(&self, day: Day) -> bool {
        day % self.pay_frequency.days() == 0
    }
}

#[derive(Debug)]
pub struct Desire {
    // 0.0 to 1.0
    level: f32,

    growth_per_day: f32,
}

impl Desire {
    pub fn new() -> Self {
        Desire {
            level: 0.0,
            growth_per_day: rand::random_range(0.1..0.2),
        }
    }
    pub fn update(&mut self) {
        self.level = (self.level + self.growth_per_day).clamp(0.0, 1.0);
    }
}

impl Consumer {
    pub fn new(savings: Currency, job: Job) -> Consumer {
        let desires = Goody::ALL.into_iter().map(|g| (g, Desire::new())).collect();

        Consumer {
            wallet: Wallet::new(savings),
            desires,
            job: job,
        }
    }

    pub fn new_random() -> Consumer {
        let desires = Goody::ALL.into_iter().map(|g| (g, Desire::new())).collect();
        Consumer {
            wallet: Wallet::new_random(),
            job: Job::new_random(),
            desires: desires,
        }
    }

    pub fn choose_offers(&self, offers: Vec<Offer>) -> Vec<Offer> {
        offers
    }

    pub fn purchase(&mut self, offer: Offer) {}

    pub fn can_afford(&self, amount: Currency) -> bool {
        self.wallet.get_balance() >= amount
    }

    pub fn pay_if_pay_day(&mut self, day: Day) {
        if self.job.is_pay_day(day) {
            self.wallet.pay(self.job.salary);
        }
    }

    pub fn update_desire(&mut self) {
        self.desires.values_mut().for_each(|d| d.update());
    }

    pub fn wallet_balance(&self) -> Currency {
        self.wallet.get_balance()
    }
}
