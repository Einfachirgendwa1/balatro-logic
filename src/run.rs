use crate::{
    blind::Blind
    ,
    hands::HandType,
    joker::{Joker, JokerType::Chicot},
    stake::Stake,
};
use std::{cmp::max, ops::Not};

pub struct Run {
    pub(crate) stake: Stake,
    pub(crate) money: u32,
    pub(crate) hand_size: u32,
    pub(crate) jokers: Vec<Joker>,
    pub(crate) joker_slots: usize,
    pub(crate) times_played: [u32; HandType::COUNT],
    pub(crate) hand_levels: [u32; HandType::COUNT],
}

impl Run {
    pub(crate) fn emit_on_blind_entered(&mut self, blind: &mut Blind) {}

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
}
