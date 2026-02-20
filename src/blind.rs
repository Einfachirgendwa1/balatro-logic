use crate::{
    blind::{BlindType::*, BossBlindType::*},
    event_list::HandPlayedEventData,
    hands::{Hand, HandType},
    run::RunData,
};
use strum::EnumCount;

#[derive(Debug, Default, PartialEq)]
pub struct Blind {
    pub chips: f64,
    pub mult: f64,
    pub blind_type: BlindType,
    pub cards: Vec<usize>,
    pub selected: Hand,
    pub held: Vec<usize>,
    pub score: f64,
    pub requirement: f64,
    pub hands: u32,
    pub discards: u32,
}

#[derive(Debug, Default, PartialEq)]
pub enum BlindType {
    #[default]
    Small,
    Big,
    Boss(BossBlindType),
    ShowdownBoss(ShowdownBossBlindType),
}

impl BlindType {
    pub fn boss(&self) -> bool {
        matches!(self, Boss(_) | ShowdownBoss(_))
    }
}

#[derive(Debug, PartialEq)]
pub enum BossBlindType {
    TheHook,
    TheOx,
    TheHouse,
    TheWall,
    TheWheel,
    TheArm,
    TheClub,
    TheFish,
    ThePsychic,
    TheGoad,
    TheWater,
    TheWindow,
    TheManacle,
    TheEye {
        was_already_played: [bool; HandType::COUNT],
    },
    TheMouth {
        allowed_hand: Option<HandType>,
    },
    ThePlant,
    TheSerpent,
    ThePillar,
    TheNeedle,
    TheHead,
    TheTooth,
    TheFlint,
    TheMark,
}

#[derive(Debug, PartialEq)]
pub enum ShowdownBossBlindType {
    AmberAcorn,
    VerdantLeaf,
    VioletVessel,
    CrimsonHeart,
    CeruleanBell,
}

impl Blind {
    pub fn hand_played(&mut self, data: &mut RunData, event: &mut HandPlayedEventData) {
        let hand_type = event.hand.resolve(&data.cards).hand_type();

        match &mut self.blind_type {
            Boss(TheArm) => data.change_hand_level(hand_type, -1),

            Boss(TheEye { was_already_played }) => {
                if was_already_played[hand_type as usize] {
                    event.allowed = false;
                    return;
                }

                was_already_played[hand_type as usize] = true;
            }

            Boss(TheMouth { allowed_hand }) => {
                if hand_type != *allowed_hand.get_or_insert(hand_type) {
                    event.allowed = false;
                }
            }

            _ => {}
        }
    }

    pub fn draw(&mut self, data: &RunData) {
        for _ in self.held.len()..data.hand_size as usize {
            let Some(card) = self.cards.pop() else {
                return;
            };

            self.held.push(card);
        }
    }

    pub fn discard(&mut self) -> Option<()> {
        if self.discards == 0 {
            return None;
        }

        self.remove_selected_from_hand();
        self.selected = Hand::default();
        self.discards -= 1;

        Some(())
    }

    pub fn prepare_play(&mut self, data: &RunData) -> Option<Hand> {
        if self.hands == 0 {
            return None;
        }

        let hand_type = self.selected.resolve(&data.cards).hand_type();
        self.chips = data.base_chips[hand_type as usize] as f64;
        self.mult = data.base_mult[hand_type as usize] as f64;

        let old_selected = self.selected.clone();
        self.remove_selected_from_hand();
        self.hands -= 1;
        self.selected = Hand::default();

        Some(old_selected)
    }

    pub fn select(&mut self, idx: usize) {
        if self.selected.len == 5 || self.selected.card_slice().contains(&idx) {
            return;
        }

        self.selected.cards[self.selected.len] = self.held[idx];
        self.selected.len += 1;
    }

    fn remove_selected_from_hand(&mut self) {
        for card in self.selected.card_slice() {
            let mut idxes = Vec::new();
            for (idx, held_card) in self.held.iter().enumerate() {
                if card == held_card {
                    idxes.push(idx);
                }
            }

            for idx in idxes {
                self.held.remove(idx);
            }
        }
    }
}
