use crate::{
    blind::{BlindType::*, BossBlindType::*},
    event::{OnBlindEntered, OnHandPlayed},
    hands::HandType,
};

pub struct Blind {
    pub(crate) blind_type: BlindType,
    pub(crate) hands: u32,
    pub(crate) discards: u32,
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
        only_playable_hand: Option<HandType>,
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
    pub(crate) fn blind_entered(&mut self, event: &mut OnBlindEntered) {
        match &mut self.blind_type {
            Boss(TheWater) => self.discards *= event.run.get_chicot_count(),
            Boss(TheNeedle) => self.hands = 1 + (self.hands - 1) * event.run.get_chicot_count(),
            Boss(TheManacle) => event.run.hand_size += event.run.get_chicot_count() - 1,
            _ => {}
        }
    }

    pub(crate) fn hand_played(&mut self, event: OnHandPlayed) {
        match &mut self.blind_type {
            Boss(TheArm) => event.run.change_hand_level(event.hand_type, -1),
            Boss(TheEye { was_already_played }) => {
                if was_already_played[event.hand_type as usize] {
                    (event.not_allowed)()
                }

                was_already_played[event.hand_type as usize] = true;
            }

            Boss(TheMouth { only_playable_hand }) => match only_playable_hand {
                None => *only_playable_hand = Some(event.hand_type),

                Some(hand) => {
                    if event.hand_type != *hand {
                        (event.not_allowed)()
                    }
                }
            },

            _ => {}
        }
    }
}
