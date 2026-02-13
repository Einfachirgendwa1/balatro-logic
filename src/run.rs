use crate::{
    consumable::Consumable,
    decks::Deck,
    hands::HandType,
    joker::{Joker, JokerType::Chicot},
    stake::Stake,
    vouchers::Voucher,
};
use std::{cmp::max, collections::HashMap, ops::Not};
use strum::EnumCount;
use Voucher::*;

pub struct Run {
    pub(crate) seed: String,
    pub(crate) hashed_seed: f64,
    pub(crate) deck: Deck,
    pub(crate) stake: Stake,
    pub(crate) ante: i32,
    pub(crate) money: u32,
    pub(crate) hand_size: u32,
    pub(crate) jokers: Vec<Joker>,
    pub(crate) joker_slots: usize,
    pub(crate) consumables: Vec<Consumable>,
    pub(crate) consumable_slots: usize,
    pub(crate) vouchers: [bool; Voucher::COUNT],
    pub(crate) times_played: [u32; HandType::COUNT],
    pub(crate) hand_levels: [u32; HandType::COUNT],
    pub(crate) pseudorandom_state: HashMap<String, f64>,
    pub(crate) starting_hands: u32,
    pub(crate) starting_discards: u32,
}

impl Run {
    pub(crate) fn change_hand_level(&mut self, hand: HandType, amount: i32) {
        let hand_level = &mut self.hand_levels[hand as usize];
        *hand_level = max(*hand_level as i32 + amount, 1) as u32;
    }

    pub(crate) fn is_most_played_hand(&self, hand_in_question: HandType) -> bool {
        let times_hand_was_played = self.times_played[hand_in_question as usize];

        self.times_played
            .iter()
            .enumerate()
            .filter_map(|(hand, amount)| (hand != hand_in_question as usize).then_some(*amount))
            .any(|amount| amount > times_hand_was_played)
            .not()
    }

    pub(crate) fn get_chicot_count(&self) -> u32 {
        self.jokers
            .iter()
            .filter(|joker| joker.joker_type == Chicot)
            .count() as _
    }

    pub(crate) fn apply_voucher_effects(&mut self, voucher: Voucher) {
        if cfg!(debug_assertions) {
            assert!(!self.vouchers[voucher as usize]);

            if voucher as u8 % 2 == 1 {
                assert!(self.vouchers[voucher as usize - 1]);
            }
        }

        self.vouchers[voucher as usize] = true;
        match voucher {
            CrystalBall => self.consumable_slots += 1,
            Grabber | NachoTong => self.starting_hands += 1,
            Wasteful | Recyclomancy => self.starting_discards += 1,
            PaintBrush | Palette => self.hand_size += 1,
            Antimatter => self.joker_slots += 1,
            Hieroglyph => {
                self.ante -= 1;
                self.starting_hands -= 1;
            }
            Petroglyph => {
                self.ante -= 1;
                self.starting_discards -= 1;
            }
            _ => {}
        }
    }
}
