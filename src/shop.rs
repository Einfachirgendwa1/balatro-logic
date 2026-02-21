use crate::vouchers::Voucher;

pub struct Shop {
    pub joker_weight: f64,
    pub tarot_weight: f64,
    pub planet_weight: f64,
    pub playing_card_weight: f64,
    pub spectral_card_weight: f64,

    pub price_multiplier: f64, // the game calls this `discount_percent`
    pub reroll_price: f64,

    pub vouchers: Vec<Voucher>,
}

impl Default for Shop {
    fn default() -> Self {
        Self {
            joker_weight: 20.,
            tarot_weight: 4.,
            planet_weight: 4.,
            playing_card_weight: 0.,
            spectral_card_weight: 0.,
            price_multiplier: 1.,
            reroll_price: 5.,
            vouchers: Vec::new(),
        }
    }
}
