use crate::{
    blind::{BlindType::*, BossBlindType::*},
    event_list::HandPlayedEventData,
    hands::HandType,
    run::{Run, RunData},
};
use strum::EnumCount;
use ShowdownBossBlindType::VioletVessel;

pub struct Blind {
    pub chips: f64,
    pub mult: f64,
    pub blind_type: BlindType,
    pub requirement: f64,
    pub hands: u32,
    pub discards: u32,
}

pub enum BlindType {
    Small,
    Big,
    Boss(BossBlindType),
    ShowdownBoss(ShowdownBossBlindType),
}

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

pub enum ShowdownBossBlindType {
    AmberAcorn,
    VerdantLeaf,
    VioletVessel,
    CrimsonHeart,
    CeruleanBell,
}

impl Blind {
    fn hand_played(&mut self, data: &mut RunData, event: &mut HandPlayedEventData) {
        let hand_type = event.hand.resolve(&data.cards).hand_type();

        match &mut self.blind_type {
            Boss(TheArm) => data.change_hand_level(hand_type, -1),

            Boss(TheEye { was_already_played }) => {
                if was_already_played[hand_type as usize] {
                    (event.not_allowed)()
                }

                was_already_played[hand_type as usize] = true;
            }

            Boss(TheMouth { allowed_hand }) => {
                if hand_type != *allowed_hand.get_or_insert(hand_type) {
                    (event.not_allowed)()
                }
            }

            _ => {}
        }
    }
    pub fn new(run: &mut Run, blind_type: BlindType) {
        let base = run.data.base_chip_requirement();
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
            Boss(TheWater) => run.data.starting_discards * run.get_chicot_count(),
            _ => run.data.starting_discards,
        };

        let hands = match blind_type {
            Boss(TheNeedle) => 1 + (run.data.starting_hands - 1) * run.get_chicot_count(),
            _ => run.data.starting_hands,
        };

        match blind_type {
            Boss(TheManacle) => run.data.hand_size += run.get_chicot_count() - 1,
            _ => {}
        }

        let blind = Blind {
            chips: 0.,
            mult: 0.,
            blind_type,
            requirement,
            hands,
            discards,
        };

        run.enter_blind(blind);
    }
}
