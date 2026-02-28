use crate::{
    builders::{
        consumable::ConsumableCreator,
        joker::{JokerCreator, JokerRarityMode},
    },
    card::Edition,
    consumable::{PLANET_ORDER, Planet, Spectral, Tarot},
    decks::DEFAULT_CARDS,
    joker::{
        JokerEdition,
        JokerEdition::{Base, Foil, Holographic, Negative, Polychrome},
    },
    run::{Run, RunData},
    seeding::{math, random_element, random_idx},
    shop::{ShopItem, ShopItemType},
    vouchers::Voucher,
};
use num_traits::FromPrimitive;
use strum::{EnumCount, IntoEnumIterator};

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

    pub(crate) fn poll(&mut self, pool: &[bool], pool_key: &str) -> usize {
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

    fn poll_joker_edition(&mut self, key: &str) -> JokerEdition {
        math::randomseed(self.rng.seed(key));

        match math::random() {
            poll if poll > 1. - 0.003 * self.shop.edition_rate => Negative,
            poll if poll > 1. - 0.006 * self.shop.edition_rate => Polychrome,
            poll if poll > 1. - 0.02 * self.shop.edition_rate => Holographic,
            poll if poll > 1. - 0.04 * self.shop.edition_rate => Foil,
            _ => Base,
        }
    }

    fn poll_card_edition(&mut self, key: &str, modifier: f64) -> Edition {
        math::randomseed(self.rng.seed(key));

        let rate = self.shop.edition_rate * modifier;
        match math::random() {
            poll if poll > 1. - 0.006 * rate => Edition::Polychrome,
            poll if poll > 1. - 0.02 * rate => Edition::Holographic,
            poll if poll > 1. - 0.04 * rate => Edition::Foil,
            _ => Edition::Base,
        }
    }
}

impl Run {
    pub fn poll_next_shop_item(&mut self) -> ShopItem {
        let Self { data, .. } = self;
        let total_weight: f64 = data.shop.weights.iter().sum();
        math::randomseed(data.rng.seed(&format!("cdt{}", data.ante)));
        let polled_weight = math::random() * total_weight;

        let mut check_weight = 0.;
        for item_type in ShopItemType::iter() {
            let weight = data.shop.weights[item_type as usize];
            if check_weight < polled_weight && polled_weight < check_weight + weight {
                return match item_type {
                    ShopItemType::Tarot => ConsumableCreator::<{ Tarot::COUNT }, Tarot>::builder()
                        .type_key("Tarot")
                        .origin_key("sho")
                        .build()
                        .create(data)
                        .into(),

                    ShopItemType::Planet => {
                        ConsumableCreator::<{ Planet::COUNT }, Planet>::builder()
                            .type_key("Planet")
                            .origin_key("sho")
                            .order(PLANET_ORDER)
                            .build()
                            .create(data)
                            .into()
                    }

                    ShopItemType::SpectralCard => {
                        ConsumableCreator::<{ Spectral::COUNT }, Spectral>::builder()
                            .type_key("Spectral")
                            .origin_key("sho")
                            .filter(&|spectral| *spectral != Spectral::TheSoul)
                            .build()
                            .create(data)
                            .into()
                    }

                    ShopItemType::Joker => JokerCreator::builder()
                        .origin_key("sho")
                        .joker_rarity(JokerRarityMode::RandomNonLegendary)
                        .build()
                        .create(self)
                        .into(),

                    ShopItemType::PlayingCard => {
                        let seed = data.rng.seed(&format!("frontsho{}", data.ante));
                        ShopItem::PlayingCard(random_element(&DEFAULT_CARDS, seed).clone())
                    }
                };
            }

            check_weight += weight;
        }

        unreachable!()
    }
}
