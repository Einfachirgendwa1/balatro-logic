use crate::{
    blind::BlindType,
    card::{Card, Suit::*},
    consumable::{
        Consumable::{SpectralCard, TarotCard},
        Spectral::Hex,
        Tarot::TheFool,
    },
    decks::{DEFAULT_CARDS, DeckType, DeckType::*},
    game_state::GameState,
    hands::HandType,
    misc::Also,
    run::{Run, RunData},
    seeding::{BalatroRng, random_element, random_seed},
    shop::{Shop, ShopItemType},
    stake::Stake,
    vouchers::{
        Voucher,
        Voucher::{CrystalBall, Overstock, PlanetMerchant, TarotMerchant, Telescope},
    },
};
use itertools::Itertools;
use strum::EnumCount;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct RunCreator {
    #[builder(default=DeckType::Red)]
    deck: DeckType,

    #[builder(default=Stake::White)]
    stake: Stake,

    #[builder(default, setter(strip_option))]
    seed: Option<String>,
}

impl RunCreator {
    #[must_use]
    pub fn create(self) -> Run {
        let Self { deck, stake, seed } = self;
        let seed = seed.unwrap_or_else(random_seed);
        let mut rng = BalatroRng::new(seed);

        let cards = match deck {
            Erratic => Self::gen_erratic(&mut rng),
            _ => DEFAULT_CARDS.clone(),
        };

        let mut data = RunData {
            rng,
            deck_type: deck,
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
            planet_unlocked: [
                true, true, true, true, true, true, true, true, true, false, false, false,
            ],
            shop: Shop::default(),
        };

        match deck {
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
}
