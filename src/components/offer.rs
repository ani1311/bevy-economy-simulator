use bevy::prelude::*;

use crate::components::common::{Currency, Goody};

#[derive(Clone)]
pub struct Offer {
    pub seller: Entity,
    pub goody: Goody,
    pub price: Currency,
}
