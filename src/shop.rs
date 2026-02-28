use crate::{
    boosters::BoosterPackType, card::Card, consumable::Consumable, joker::Joker, run::Run,
    vouchers::Voucher,
};
use derive_more::From;
use num_derive::FromPrimitive;
use std::iter::repeat_with;
use strum::{EnumCount, EnumIter};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, EnumCount, From)]
pub enum ShopItem {
    Consumable(Consumable),
    Joker(Joker),
    PlayingCard(Card),
}

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
    pub pack_weights: [f64; BoosterPackType::COUNT],

    pub price_multiplier: f64, // the game calls this `discount_percent`
    pub reroll_price: f64,
    pub size: usize,

    pub edition_rate: f64,
    pub inventory: Vec<ShopItem>,
    pub vouchers: Vec<Voucher>,

    pub first_shop_buffoon: bool,
}

impl Default for Shop {
    fn default() -> Self {
        Self {
            weights: [20., 4., 4., 0., 0.],
            pack_weights: [4., 2., 0.5, 4., 2., 0.5, 4., 2., 0.5, 1.2, 0.6, 0.15, 0.6, 0.3, 0.07],
            price_multiplier: 1.,
            reroll_price: 5.,
            size: 2,
            edition_rate: 1.,
            vouchers: Vec::new(),
            first_shop_buffoon: false,
            inventory: Vec::new(),
        }
    }
}

impl Run {
    pub fn next_shop_batch(&mut self) -> Vec<ShopItem> {
        let shop_size = self.data.shop.size;
        repeat_with(|| self.poll_next_shop_item()).take(shop_size).collect()
    }
}
