use crate::vouchers::Voucher;
use num_derive::FromPrimitive;
use strum::{EnumCount, EnumIter};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
pub enum ShopItemType {
    Joker,
    Tarot,
    Planet,
    PlayingCard,
    SpectralCard,
}

pub struct Shop {
    pub weights: [f64; ShopItemType::COUNT],

    pub price_multiplier: f64, // the game calls this `discount_percent`
    pub reroll_price: f64,

    pub vouchers: Vec<Voucher>,
}

impl Default for Shop {
    fn default() -> Self {
        Self {
            weights: [20., 4., 4., 0., 0.],
            price_multiplier: 1.,
            reroll_price: 5.,
            vouchers: Vec::new(),
        }
    }
}
