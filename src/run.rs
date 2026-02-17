use crate::{
    blind::{
        Blind, BlindType,
        BlindType::{Big, Boss, ShowdownBoss, Small},
        BossBlindType::{TheManacle, TheNeedle, TheWall, TheWater},
        ShowdownBossBlindType::VioletVessel,
    },
    card::Card,
    consumable::Consumable,
    decks::DeckType,
    event::Event,
    event_list::HandPlayedEventData,
    game_state::GameState,
    hands::HandType,
    joker::{Joker, JokerType::Chicot, PostExecCb},
    stake::{
        Stake,
        Stake::{Green, Purple},
    },
    vouchers::Voucher,
};
use itertools::Itertools;
use std::{cmp::max, collections::HashMap, ops::Not};
use strum::EnumCount;
use Voucher::*;

pub struct Run {
    pub data: RunData,
    pub jokers: Vec<Joker>,
    pub game_state: GameState,
}

pub struct RunData {
    pub stake: Stake,
    pub hashed_seed: f64,
    pub seed: String,
    pub cards: Vec<Card>,
    pub deck_type: DeckType,
    pub joker_slots: usize,
    pub consumables: Vec<Consumable>,
    pub consumable_slots: usize,
    pub vouchers: [bool; Voucher::COUNT],
    pub starting_hands: u32,
    pub starting_discards: u32,
    pub money: u32,
    pub hand_size: u32,
    pub ante: i32,
    pub times_played: [u32; HandType::COUNT],
    pub hand_levels: [u32; HandType::COUNT],
    pub pseudorandom_state: HashMap<String, f64>,
}

pub static BLIND_REQUIREMENTS: [[f64; 3]; 9] = [
    [100., 100., 100.],
    [300., 300., 300.],
    [800., 900., 1000.],
    [2000., 2600., 3200.],
    [5000., 8000., 9000.],
    [11000., 20000., 25000.],
    [20000., 36000., 60000.],
    [35000., 60000., 110000.],
    [50000., 100000., 200000.],
];

impl RunData {
    pub fn base_chip_requirement(&self) -> f64 {
        let idx = match self.stake {
            stake if stake >= Purple => 2,
            stake if stake >= Green => 1,
            _ => 0,
        };

        match self.ante {
            ante if ante <= 8 => BLIND_REQUIREMENTS[ante.max(0) as usize][idx],
            ante => {
                let endless_mode_ante = ante as f64 - 8.;
                let first_pow = (0.75 * endless_mode_ante).powf(1. + 0.2 * endless_mode_ante);
                BLIND_REQUIREMENTS[8][idx] * (1.6 + first_pow).powf(endless_mode_ante)
            }
        }
    }

    pub fn change_hand_level(&mut self, hand: HandType, amount: i32) {
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

impl Run {
    pub fn new_blind(&mut self, blind_type: BlindType) {
        let base = self.data.base_chip_requirement();
        let requirement = match &blind_type {
            Small => base,
            Big => base * 1.5,
            Boss(TheWall) => base * 4.,
            Boss(TheNeedle) => base,
            Boss(_) => base * 2.,
            ShowdownBoss(VioletVessel) => base * 6.,
            ShowdownBoss(_) => base * 2.,
        };

        let discards = match blind_type {
            Boss(TheWater) => self.data.starting_discards * self.get_chicot_count(),
            _ => self.data.starting_discards,
        };

        let hands = match blind_type {
            Boss(TheNeedle) => 1 + (self.data.starting_hands - 1) * self.get_chicot_count(),
            _ => self.data.starting_hands,
        };

        if let Boss(TheManacle) = blind_type {
            self.data.hand_size += self.get_chicot_count() - 1
        }

        let blind = Blind {
            chips: 0.,
            mult: 0.,
            blind_type,
            requirement,
            hands,
            discards,
        };

        self.joker_event(Event::BlindEntered, &mut (), |joker, _, _| {
            joker.blind_entered()
        });

        self.game_state = GameState::Blind(blind);
    }

    pub fn get_chicot_count(&self) -> u32 {
        self.jokers
            .iter()
            .filter(|joker| joker.joker_type == Chicot)
            .count() as _
    }

    fn joker_event<F, T>(&mut self, event: Event, t: &mut T, mut f: F)
    where
        F: FnMut(&mut Joker, &mut RunData, &mut T) -> Option<PostExecCb>,
    {
        let event_usize = event as usize;

        self.jokers
            .iter_mut()
            .enumerate()
            .sorted_by_key(|(_, joker)| joker.dispatcher_order.events[event_usize])
            .filter_map(|(idx, joker)| f(joker, &mut self.data, t).map(|cb| (idx, cb)))
            .collect_vec()
            .into_iter()
            .for_each(|(idx, mut callback)| callback(idx, self));
    }

    pub fn hand_played(&mut self, blind: &mut Blind, mut event: HandPlayedEventData) {
        blind.hand_played(&mut self.data, &mut event);

        self.joker_event(Event::Scored, &mut event, |joker, data, event| {
            joker.scored(data, blind, event)
        });
    }
}
