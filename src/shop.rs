use crate::{
    boosters::BoosterPackType, card::Card, consumable::Consumable, joker::Joker, run::Run,
    vouchers::Voucher,
};
use derive_more::From;
use itertools::Itertools;
use num_derive::FromPrimitive;
use std::{
    array,
    fmt::{Debug, Display, Formatter},
    iter::repeat_with,
};
use strum::{EnumCount, EnumIter};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, EnumCount, From)]
pub enum ShopItem {
    Consumable(Consumable),
    Joker(Joker),
    PlayingCard(Card),
}

impl Display for ShopItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShopItem::Consumable(x) => Display::fmt(x, f),
            ShopItem::Joker(x) => Display::fmt(x, f),
            ShopItem::PlayingCard(x) => Display::fmt(x, f),
        }
    }
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
    pub packs: [Option<BoosterPackType>; 2],
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
            packs: [None; 2],
        }
    }
}

impl Display for Shop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pack_string = self
            .packs
            .iter()
            .map(|opt| opt.map(|pack| pack.to_string()).unwrap_or("None".to_string()))
            .join(", ");

        writeln!(f, "Inventory: {}", self.inventory.iter().join(", "))?;
        writeln!(f, "Booster Packs: {}", pack_string)?;
        write!(f, "Vouchers: {}", self.vouchers.iter().join(", "))
    }
}

impl Run {
    pub fn enter_shop(&mut self, new_ante: bool) {
        if new_ante {
            self.data.shop.vouchers = vec![self.data.poll_next_voucher()];
        }

        self.regenerate_shop_inventory();
        self.data.shop.packs = array::from_fn(|_| Some(self.next_shop_booster_pack()));
    }

    pub fn reroll(&mut self) {
        self.regenerate_shop_inventory();

        self.data.money -= self.data.shop.reroll_price;
        self.data.shop.reroll_price += 1.;
    }

    fn regenerate_shop_inventory(&mut self) {
        let shop_size = self.data.shop.size;
        let inv = repeat_with(|| self.poll_next_shop_item()).take(shop_size).collect();
        self.data.shop.inventory = inv;
    }
}
