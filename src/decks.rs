use crate::{
    blind::BlindType,
    card::{ALPHABETICAL_RANK_ORDER, ALPHABETICAL_SUIT_ORDER, Card, Suit::*},
    consumable::{Consumable, Consumable::SpectralCard, Spectral::Hex, Tarot},
    game_state::GameState,
    hands::HandType,
    misc::{Also, UnpackedMap},
    run::{Run, RunData},
    seeding::{BalatroRng, random_element, random_seed},
    shop::{Shop, ShopItemType},
    stake::Stake,
    vouchers::Voucher,
};
use Consumable::TarotCard;
use DeckType::*;
use Tarot::TheFool;
use Voucher::*;
use itertools::Itertools;
use std::sync::LazyLock;
use strum::{EnumCount, EnumIter};

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
    #[must_use]
    pub fn sorted(&self) -> Vec<&'_ Card> {
        let mut vec = self.cards.iter().collect::<Vec<&_>>();
        vec.sort_by(|a, b| a.cmp(b).reverse());
        vec
    }
}

impl Run {
    #[must_use]
    pub fn new(deck_type: DeckType, stake: Stake, seed: String) -> Run {
        let mut rng = BalatroRng::new(seed);

        let cards = match deck_type {
            Erratic => Self::gen_erratic(&mut rng),
            _ => DEFAULT_CARDS.clone(),
        };

        let mut data = RunData {
            rng,
            deck_type,
            cards,
            base_chips: HandType::base_chips(),
            base_mult: HandType::base_mult(),
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
            times_played: [0; 12],
            hand_levels: [1; 12],
            shop: Shop::default(),
        };

        match deck_type {
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
                data.shop.weights[ShopItemType::SpectralCard as usize] = 2.;
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
            Abandoned => data.cards.retain(|card| !card.rank.is_face_card()),
            _ => {}
        }

        Run { data, jokers: Vec::new(), game_state: GameState::Shop }
            .also_mut(|run| run.new_blind(BlindType::Small))
    }

    #[must_use]
    pub fn gen_erratic(rng: &mut BalatroRng) -> Vec<Card> {
        let default_cards_sort_id = DEFAULT_CARDS.iter().enumerate().collect_vec();

        (0..52)
            .map(|_| random_element(&default_cards_sort_id, rng.seed("erratic")))
            .sorted_by_key(|(sort_id, _)| sort_id)
            .map(|(_, card)| (*card).clone())
            .collect()
    }

    #[must_use]
    pub fn new_with_random_seed(deck_type: DeckType, stake: Stake) -> Run {
        Self::new(deck_type, stake, random_seed())
    }
}
