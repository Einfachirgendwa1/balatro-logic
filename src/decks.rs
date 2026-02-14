use crate::{
    card::{
        Card, Edition, Enhancement, Rank, Seal, Suit,
        Suit::{Heart, Spade},
    },
    consumable::{Consumable, Consumable::SpectralCard, Spectral::Hex, Tarot},
    misc::Log,
    run::Run,
    seeding::{hash, random_element},
    stake::Stake,
    vouchers::Voucher,
};
use itertools::Itertools;
use std::{collections::HashMap, sync::LazyLock};
use strum::{EnumCount, EnumIter, IntoEnumIterator};
use Consumable::TarotCard;
use DeckType::*;
use Tarot::TheFool;
use Voucher::*;

pub static DEFAULT_CARDS: LazyLock<Vec<Card>> = LazyLock::new(|| {
    Suit::iter()
        .cartesian_product(Rank::iter())
        .map(|(suit, rank)| Card {
            suit,
            rank,
            enhancement: Enhancement::None,
            edition: Edition::Base,
            seal: Seal::None,
        })
        .collect()
});

pub struct Deck {
    pub deck_type: DeckType,
    pub cards: Vec<Card>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
pub enum DeckType {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Magic,
    Nebula,
    Ghost,
    Abandoned,
    Checkered,
    Zodiac,
    Painted,
    Anaglyph,
    Plasma,
    Erratic,
}

impl DeckType {
    pub fn new_run(self, seed: String, stake: Stake) -> Run {
        let mut run = Run {
            hashed_seed: hash(seed.as_bytes()) as _,
            seed,
            deck: Deck {
                deck_type: self,
                cards: DEFAULT_CARDS.clone(),
            },
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
            Checkered => {
                for card in &mut run.deck.cards {
                    match card.suit {
                        Suit::Club => card.suit = Spade,
                        Suit::Diamond => card.suit = Heart,
                        _ => {}
                    }
                }
            }
            Erratic => {
                for idx in 0..run.deck.cards.len() {
                    let seed = run.seed("erratic");
                    run.deck.cards[idx] = random_element(&DEFAULT_CARDS, seed).clone().log();
                }
            }
            _ => {}
        }

        run
    }
}
