use crate::{
    blind::{BlindType::*, BossBlindType::*},
    event_list::HandPlayedEventData,
    hands::{Hand, HandType},
    misc::Also,
    run::RunData,
    seeding::random_element,
};
use itertools::Itertools;
use strum::{Display, EnumCount, EnumIter};

#[derive(Debug, Default, PartialEq)]
pub struct Blind {
    pub chips: f64,
    pub mult: f64,
    pub blind_type: BlindType,
    pub blind_data: Option<BossBlindData>,
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
}

impl BlindType {
    pub fn default_data(&self) -> Option<BossBlindData> {
        match self {
            Boss(TheEye) => Some(BossBlindData::TheEye { was_already_played: [false; _] }),
            Boss(TheMouth) => Some(BossBlindData::TheMouth { allowed_hand: None }),
            _ => None,
        }
    }
}

impl RunData {
    pub fn current_boss(&mut self) -> BossBlindType {
        let bosses: &[BossBlindType] = match self.ante % 8 == 0 && self.ante != 0 {
            true => &SHOWDOWN_BOSSES,
            false => &NORMAL_BOSSES,
        };

        let min = bosses.iter().map(|boss| self.times_boss_used[*boss as usize]).min().unwrap();

        let mut allowed_bosses = bosses
            .iter()
            .filter(|boss| {
                self.times_boss_used[**boss as usize] == min && boss.min_ante() <= self.ante
            })
            .collect_vec();

        allowed_bosses.sort_by_key(|boss| boss.game_name());

        **random_element(&allowed_bosses, self.rng.seed("boss"))
            .also(|boss| self.times_boss_used[***boss as usize] += 1)
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone, Display, EnumCount, EnumIter)]
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
    TheEye,
    TheMouth,
    ThePlant,
    TheSerpent,
    ThePillar,
    TheNeedle,
    TheHead,
    TheTooth,
    TheFlint,
    TheMark,
    AmberAcorn,
    VerdantLeaf,
    VioletVessel,
    CrimsonHeart,
    CeruleanBell,
}

pub static NORMAL_BOSSES: [BossBlindType; BossBlindType::COUNT - SHOWDOWN_BOSSES.len()] = [
    TheHook, TheOx, TheHouse, TheWall, TheWheel, TheArm, TheClub, TheFish, ThePsychic, TheGoad,
    TheWater, TheWindow, TheManacle, TheEye, TheMouth, ThePlant, TheSerpent, ThePillar, TheNeedle,
    TheHead, TheTooth, TheFlint, TheMark,
];

pub const SHOWDOWN_BOSSES: [BossBlindType; 5] =
    [AmberAcorn, VerdantLeaf, VioletVessel, CrimsonHeart, CeruleanBell];

const ANY: i32 = i32::MIN;
impl BossBlindType {
    #[inline]
    #[must_use]
    pub const fn min_ante(&self) -> i32 {
        match self {
            TheHook | TheClub | ThePsychic | TheGoad | TheWindow | TheManacle | ThePillar
            | TheHead => ANY,
            TheHouse | TheWall | TheWheel | TheArm | TheFish | TheWater | TheMouth | TheNeedle
            | TheFlint | TheMark => 2,
            TheEye | TheTooth => 3,
            ThePlant => 4,
            TheSerpent => 5,
            TheOx => 6,
            AmberAcorn | VerdantLeaf | VioletVessel | CrimsonHeart | CeruleanBell => 8,
        }
    }

    #[inline]
    #[must_use]
    pub const fn game_name(&self) -> &'static str {
        match self {
            TheHook => "bl_hook",
            TheOx => "bl_ox",
            TheHouse => "bl_house",
            TheWall => "bl_wall",
            TheWheel => "bl_wheel",
            TheArm => "bl_arm",
            TheClub => "bl_club",
            TheFish => "bl_fish",
            ThePsychic => "bl_psychic",
            TheGoad => "bl_goad",
            TheWater => "bl_water",
            TheWindow => "bl_window",
            TheManacle => "bl_manacle",
            TheEye => "bl_eye",
            TheMouth => "bl_mouth",
            ThePlant => "bl_planet",
            TheSerpent => "bl_serpent",
            ThePillar => "bl_pillar",
            TheNeedle => "bl_needle",
            TheHead => "bl_head",
            TheTooth => "bl_tooth",
            TheFlint => "bl_flint",
            TheMark => "bl_mark",
            AmberAcorn => "bl_final_acorn",
            VerdantLeaf => "bl_final_leaf",
            VioletVessel => "bl_final_vessel",
            CrimsonHeart => "bl_final_heart",
            CeruleanBell => "bl_final_bell",
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum BossBlindData {
    TheEye { was_already_played: [bool; HandType::COUNT] },
    TheMouth { allowed_hand: Option<HandType> },
}

impl Blind {
    pub fn hand_played(&mut self, data: &mut RunData, event: &mut HandPlayedEventData) {
        let hand_type = event.hand.resolve(&data.cards).hand_type();

        match &mut self.blind_type {
            Boss(TheArm) => data.change_hand_level(hand_type, -1),

            Boss(TheEye) => {
                let Some(BossBlindData::TheEye { was_already_played }) = &mut self.blind_data
                else {
                    unreachable!()
                };

                if was_already_played[hand_type as usize] {
                    event.allowed = false;
                    return;
                }

                was_already_played[hand_type as usize] = true;
            }

            Boss(TheMouth) => {
                let Some(BossBlindData::TheMouth { allowed_hand }) = &mut self.blind_data else {
                    unreachable!()
                };

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
