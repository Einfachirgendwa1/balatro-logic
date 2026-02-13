use crate::{
    consumable::{Consumable, Consumable::SpectralCard, Spectral::Hex, Tarot},
    run::Run,
    seeding::hash,
    stake::Stake,
    vouchers::Voucher,
};
use std::collections::HashMap;
use strum::{EnumCount, EnumIter};
use Consumable::TarotCard;
use Deck::*;
use Tarot::TheFool;
use Voucher::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
pub enum Deck {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Magic,
    Nebula,
    Ghost,
    Abandoned,
    Checked,
    Zodiac,
    Painted,
    Anaglyph,
    Plasma,
    Erratic,
}

impl Deck {
    pub fn new_run(self, seed: String, stake: Stake) -> Run {
        let mut run = Run {
            hashed_seed: hash(&seed),
            seed,
            deck: self,
            stake,
            ante: 1,
            money: 4,
            hand_size: 8,
            jokers: Vec::new(),
            joker_slots: 5,
            consumables: Vec::new(),
            consumable_slots: 5,
            vouchers: [false; Voucher::COUNT],
            times_played: [0; 12],
            hand_levels: [1; 12],
            pseudorandom_state: HashMap::new(),
            starting_hands: 4,
            starting_discards: if stake >= Stake::Blue { 2 } else { 3 },
        };

        match self {
            Red => run.starting_discards += 1,
            Blue => run.starting_hands += 1,
            Yellow => run.money += 10,
            Black => {
                run.joker_slots += 1;
                run.starting_hands -= 1;
            }
            Magic => {
                run.apply_voucher_effects(CrystalBall);
                run.consumables.push(TarotCard(TheFool));
                run.consumables.push(TarotCard(TheFool));
            }
            Nebula => {
                run.apply_voucher_effects(Telescope);
                run.consumable_slots -= 1;
            }
            Ghost => run.consumables.push(SpectralCard(Hex)),
            Zodiac => {
                run.apply_voucher_effects(TarotMerchant);
                run.apply_voucher_effects(PlanetMerchant);
                run.apply_voucher_effects(Overstock);
            }
            Painted => {
                run.hand_size += 2;
                run.joker_slots -= 1;
            }
            _ => {}
        }

        run
    }
}
