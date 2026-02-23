use crate::{
    card::Card,
    consumable::{
        Consumable,
        Consumable::{PlanetCard, SpectralCard, TarotCard},
        Spectral, Tarot,
    },
    decks::DEFAULT_CARDS,
    hands::HandType,
    joker::{COMMON_JOKERS, Joker, JokerType, RARE_JOKERS, UNCOMMON_JOKERS},
    pools::JokerRarity::{Common, Rare, Uncommon},
    run::RunData,
    seeding::{math, random_element, random_idx},
    shop::ShopItemType,
    vouchers::Voucher,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, EnumCount)]
pub enum ShopItem {
    Consumable(Consumable),
    Joker(Joker),
    PlayingCard(Card),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
pub enum JokerRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl RunData {
    pub fn available_vouchers(&self) -> [bool; Voucher::COUNT] {
        let mut v = [false; Voucher::COUNT];

        for voucher in Voucher::iter() {
            let allowed = !self.vouchers[voucher as usize]
                && voucher.requirements_fulfilled(&self.vouchers)
                && !self.shop.vouchers.contains(&voucher);

            v[voucher as usize] = allowed;
        }

        if !v.iter().any(|available| *available) {
            v[Voucher::Blank as usize] = true;
        }

        v
    }

    fn available_jokers(&self, rarity_pool: &[JokerType]) -> Vec<bool> {
        rarity_pool.iter().map(|_| true).collect()
    }

    fn available_tarots(&self) -> [bool; Tarot::COUNT] {
        [true; Tarot::COUNT]
    }

    fn available_planets(&self) -> [bool; HandType::COUNT] {
        [true; HandType::COUNT]
    }

    fn available_spectral_cards(&self) -> [bool; Spectral::COUNT] {
        [true; Spectral::COUNT]
    }

    fn poll(&mut self, pool: &[bool], pool_key: &str) -> usize {
        let seed = self.rng.seed(pool_key);

        let mut idx = random_idx(pool, seed);
        let mut i = 1;

        while !pool[idx] {
            i += 1;

            let seed = self.rng.seed(&format!("{pool_key}_resample{i}"));
            idx = random_idx(pool, seed);
        }

        idx
    }

    pub fn poll_next_voucher(&mut self) -> Voucher {
        let available = self.available_vouchers();
        let pool_key = format!("Voucher{}", self.ante);

        let idx = self.poll(&available, &pool_key);
        Voucher::from_usize(idx).unwrap()
    }

    fn poll_next_joker(&mut self, key: &str) -> JokerType {
        let seed = self.rng.seed(&format!("rarity{}{key}", self.ante));
        math::randomseed(seed);

        let (rarity, pool) = match math::random() {
            seed if seed > 0.95 => (Rare, &RARE_JOKERS as &[JokerType]),
            seed if seed > 0.7 => (Uncommon, &UNCOMMON_JOKERS as _),
            _ => (Common, &COMMON_JOKERS as _),
        };

        let available = self.available_jokers(pool);
        let pool_key = format!("Joker{}{key}{}", rarity as u8 + 1, self.ante);

        let pool_idx = self.poll(&available, &pool_key);
        pool[pool_idx]
    }

    pub fn poll_next_shop_item(&mut self) -> ShopItem {
        let total_weight: f64 = self.shop.weights.iter().sum();
        math::randomseed(self.rng.seed(&format!("cdt{}", self.ante)));
        let polled_weight = math::random() * total_weight;

        let mut check_weight = 0.;
        for item_type in ShopItemType::iter() {
            let weight = self.shop.weights[item_type as usize];
            if check_weight < polled_weight && polled_weight < check_weight + weight {
                return match item_type {
                    ShopItemType::Joker => {
                        ShopItem::Joker(self.poll_next_joker("sho").construct_joker())
                    }
                    ShopItemType::Tarot => {
                        let available = self.available_tarots();
                        let idx = self.poll(&available, &format!("Tarotsho{}", self.ante));
                        let tarot = TarotCard(Tarot::from_usize(idx).unwrap());

                        ShopItem::Consumable(tarot)
                    }
                    ShopItemType::Planet => {
                        let available = self.available_planets();
                        let idx = self.poll(&available, &format!("Planetsho{}", self.ante));
                        let planet = PlanetCard(HandType::from_usize(idx).unwrap());

                        ShopItem::Consumable(planet)
                    }
                    ShopItemType::SpectralCard => {
                        let available = self.available_spectral_cards();
                        let idx = self.poll(&available, &format!("Spectralsho{}", self.ante));
                        println!("Idx: {idx}");
                        let planet = SpectralCard(Spectral::from_usize(idx).unwrap());

                        ShopItem::Consumable(planet)
                    }
                    ShopItemType::PlayingCard => {
                        let seed = self.rng.seed(&format!("frontsho{}", self.ante));
                        ShopItem::PlayingCard(random_element(&DEFAULT_CARDS, seed).clone())
                    }
                };
            }

            check_weight += weight;
        }

        unreachable!()
    }
}
