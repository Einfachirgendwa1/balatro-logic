use crate::{pools::ShopItem, run::RunData, vouchers::Voucher};
use num_derive::FromPrimitive;
use std::iter::repeat_with;
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
    pub size: usize,

    pub edition_rate: f64,
    pub vouchers: Vec<Voucher>,
}

impl Default for Shop {
    fn default() -> Self {
        Self {
            weights: [20., 4., 4., 0., 0.],
            price_multiplier: 1.,
            reroll_price: 5.,
            size: 2,
            edition_rate: 1.,
            vouchers: Vec::new(),
        }
    }
}

impl RunData {
    pub fn next_shop_batch(&mut self) -> Vec<ShopItem> {
        let shop_size = self.shop.size;
        repeat_with(|| self.poll_next_shop_item()).take(shop_size).collect()
    }
}
