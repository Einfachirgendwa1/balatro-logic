use crate::{
    blind::BlindType,
    card::{Card, Suit::*, ALPHABETICAL_RANK_ORDER, ALPHABETICAL_SUIT_ORDER},
    consumable::{Consumable, Consumable::SpectralCard, Spectral::Hex, Tarot},
    game_state::GameState,
    misc::{Also, UnpackedMap},
    run::{Run, RunData},
    seeding::{hash, random_element},
    stake::Stake,
    vouchers::Voucher,
};
use itertools::Itertools;
use std::{collections::HashMap, sync::LazyLock};
use strum::{EnumCount, EnumIter};
use Consumable::TarotCard;
use DeckType::*;
use Tarot::TheFool;
use Voucher::*;

pub static DEFAULT_CARDS: LazyLock<Vec<Card>> = LazyLock::new(|| {
    ALPHABETICAL_SUIT_ORDER
        .into_iter()
        .cartesian_product(ALPHABETICAL_RANK_ORDER)
        .map2(Card::new)
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

impl Deck {
    pub fn sorted(&self) -> Vec<&'_ Card> {
        let mut vec = self.cards.iter().collect::<Vec<&_>>();
        vec.sort_by(|a, b| a.cmp(b).reverse());
        vec
    }
}

impl DeckType {
    pub fn new_run(self, seed: String, stake: Stake) -> Run {
        let mut data = RunData {
            hashed_seed: hash(seed.as_bytes()) as _,
            deck_type: self,
            seed,
            stake,
            joker_slots: 5,
            consumables: Vec::new(),
            consumable_slots: 5,
            vouchers: [false; Voucher::COUNT],
            ante: 1,
            money: 4,
            hand_size: 8,
            starting_hands: 4,
            starting_discards: if stake >= Stake::Blue { 2 } else { 3 },
            cards: DEFAULT_CARDS.clone(),
            times_played: [0; 12],
            hand_levels: [1; 12],
            pseudorandom_state: HashMap::new(),
        };

        match self {
            Red => data.starting_discards += 1,
            Blue => data.starting_hands += 1,
            Yellow => data.money += 10,
            Black => {
                data.joker_slots += 1;
                data.starting_hands -= 1;
            }
            Magic => {
                data.apply_voucher_effects(CrystalBall);
                data.consumables.push(TarotCard(TheFool));
                data.consumables.push(TarotCard(TheFool));
            }
            Nebula => {
                data.apply_voucher_effects(Telescope);
                data.consumable_slots -= 1;
            }
            Ghost => {
                data.consumables.push(SpectralCard(Hex));
                todo!()
            }
            Zodiac => {
                data.apply_voucher_effects(TarotMerchant);
                data.apply_voucher_effects(PlanetMerchant);
                data.apply_voucher_effects(Overstock);
            }
            Painted => {
                data.hand_size += 2;
                data.joker_slots -= 1;
            }
            Checkered => {
                for card in &mut data.cards {
                    match card.suit {
                        Club => card.suit = Spade,
                        Diamond => card.suit = Heart,
                        _ => {}
                    }
                }
            }
            Erratic => {
                for idx in 0..data.cards.len() {
                    let seed = data.seed("erratic");
                    data.cards[idx] = random_element(&DEFAULT_CARDS, seed).clone();
                }
            }
            Abandoned => data.cards.retain(|card| !card.rank.is_face_card()),
            _ => todo!(),
        }

        Run {
            data,
            jokers: Vec::new(),
            game_state: GameState::Shop,
        }
        .also_mut(|run| run.new_blind(BlindType::Small))
    }
}
