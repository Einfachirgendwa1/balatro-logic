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
    pub seed: String,
    pub hashed_seed: f64,
    pub deck: Deck,
    pub stake: Stake,
    pub ante: i32,
    pub money: u32,
    pub hand_size: u32,
    pub jokers: Vec<Joker>,
    pub joker_slots: usize,
    pub consumables: Vec<Consumable>,
    pub consumable_slots: usize,
    pub vouchers: [bool; Voucher::COUNT],
    pub times_played: [u32; HandType::COUNT],
    pub hand_levels: [u32; HandType::COUNT],
    pub pseudorandom_state: HashMap<String, f64>,
    pub starting_hands: u32,
    pub starting_discards: u32,
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
