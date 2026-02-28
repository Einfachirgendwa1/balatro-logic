use crate::{
    blind::{
        Blind, BlindType,
        BlindType::{Big, Boss, Small},
        BossBlindType,
        BossBlindType::{TheManacle, TheNeedle, TheWall, TheWater},
    },
    card::Card,
    consumable::{Consumable, Planet},
    controller::{
        BlindAction, BlindSelectionAction, CashoutAction, Controller, ShopAction, SimulationResult,
    },
    decks::DeckType,
    event::Event,
    event_list::HandPlayedEventData,
    game_state::GameState,
    hands::{Hand, HandType},
    joker::{Joker, JokerType::Chicot},
    misc,
    seeding::{BalatroRng, shuffle},
    shop::{
        Shop, ShopItemType,
        ShopItemType::{PlayingCard, Tarot},
    },
    stake::{
        Stake,
        Stake::{Green, Purple},
    },
    vouchers::Voucher,
};
use BossBlindType::VioletVessel;
use Voucher::*;
use itertools::Itertools;
use std::{cmp::max, mem::take, ops::Not};
use strum::EnumCount;

pub struct Run {
    pub data: RunData,
    pub jokers: Vec<Joker>,
    pub game_state: GameState,
}

pub struct RunData {
    pub stake: Stake,
    pub rng: BalatroRng,
    pub shop: Shop,
    pub cards: Vec<Card>,
    pub deck_type: DeckType,
    pub joker_slots: usize,
    pub consumables: Vec<Consumable>,
    pub consumable_slots: usize,
    pub vouchers: [bool; Voucher::COUNT],
    pub starting_hands: u32,
    pub starting_discards: u32,
    pub money: f64,
    pub hand_size: u32,
    pub ante: i32,
    pub times_played: [u32; HandType::COUNT],
    pub base_chips: [u64; HandType::COUNT],
    pub base_mult: [u64; HandType::COUNT],
    pub hand_levels: [u32; HandType::COUNT],
    pub planet_unlocked: [bool; Planet::COUNT],
    pub times_boss_used: [usize; BossBlindType::COUNT],
    pub this_antes_boss: BossBlindType,
    pub showman: bool,
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
    pub fn resolve(&self, cards: &[usize]) -> Vec<&Card> {
        cards.iter().map(|idx| &self.cards[*idx]).collect()
    }

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

    pub fn apply_voucher_effects(&mut self, voucher: Voucher) {
        self.vouchers[voucher as usize] = true;

        match voucher {
            CrystalBall => self.consumable_slots += 1,
            Grabber | NachoTong => self.starting_hands += 1,
            Wasteful | Recyclomancy => self.starting_discards += 1,
            PaintBrush | Palette => self.hand_size += 1,
            RerollSurplus | RerollGlut => self.shop.reroll_price -= 2.,
            ClearanceSale | Liquidation => self.shop.price_multiplier -= 0.25,
            Antimatter => self.joker_slots += 1,
            Hieroglyph => {
                self.ante -= 1;
                self.starting_hands -= 1;
            }
            Petroglyph => {
                self.ante -= 1;
                self.starting_discards -= 1;
            }
            MagicTrick => self.shop.weights[PlayingCard as usize] = 4.,
            TarotMerchant => self.shop.weights[Tarot as usize] = 9.6,
            PlanetMerchant => self.shop.weights[ShopItemType::Planet as usize] = 8.6,
            TarotTycoon => self.shop.weights[Tarot as usize] = 32.,
            PlanetTycoon => self.shop.weights[ShopItemType::Planet as usize] = 32.,
            Overstock | OverstockPlus => self.shop.size += 1,
            Hone => self.shop.edition_rate = 2.,
            GlowUp => self.shop.edition_rate = 5.,
            OmenGlobe => {}
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
            Boss(VioletVessel) => base * 6.,
            _ => base * 2.,
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

        let mut cards = (0..self.data.cards.len()).collect_vec();
        shuffle(&mut cards, self.data.rng.seed(&format!("nr{}", self.data.ante)));

        let mut blind = Blind {
            chips: 0.,
            mult: 1.,
            score: 0.,
            cards,
            held: Vec::new(),
            selected: Hand::default(),
            blind_data: blind_type.default_data(),
            blind_type,
            requirement,
            hands,
            discards,
        };

        blind.draw(&self.data);

        let event = Event::BlindEntered;
        let event_usize = event as usize;

        self.jokers
            .iter_mut()
            .enumerate()
            .sorted_by_key(|(_, joker)| joker.dispatcher_order.events[event_usize])
            .filter_map(|(idx, joker)| joker.blind_entered().map(|cb| (idx, cb)))
            .collect_vec()
            .into_iter()
            .for_each(|(idx, mut callback)| callback(idx, self));

        self.game_state = GameState::Blind(blind);
    }

    pub fn get_chicot_count(&self) -> u32 {
        self.jokers.iter().filter(|joker| joker.joker_type == Chicot).count() as _
    }

    pub fn simulate(mut self, mut controller: impl Controller) -> SimulationResult {
        loop {
            type Callback = Box<dyn FnMut(&mut Run)>;
            let mut cbs: Vec<Callback> = Vec::new();

            match &mut self.game_state {
                GameState::Shop => {
                    for action in controller.shop() {
                        match action {
                            ShopAction::ExitShop => {
                                self.game_state = GameState::BlindSelection;
                            }
                        }
                    }
                }
                GameState::BlindSelection => match controller.blind_selection() {
                    BlindSelectionAction::PlayBlind => {
                        self.new_blind(Small);
                    }
                },
                GameState::Blind(blind) => {
                    if blind.hands == 0 || blind.held.is_empty() {
                        return SimulationResult::Lost { blind: take(blind) };
                    }

                    for action in controller.blind(blind, &mut self.data) {
                        match action {
                            BlindAction::SelectCard(card) => blind.select(card),
                            BlindAction::Discard => {
                                if blind.discard().is_some() {
                                    todo!()
                                }
                            }
                            BlindAction::Play => {
                                let Some(hand) = blind.prepare_play(&self.data) else {
                                    continue;
                                };

                                let mut event_data = HandPlayedEventData { hand, allowed: true };

                                blind.hand_played(&mut self.data, &mut event_data);

                                let cards = event_data.hand.resolve(&self.data.cards).0;
                                println!("Played {}", cards.iter().join(", "));

                                for card in cards {
                                    blind.score += card.chips as f64;
                                }

                                let event_usize = Event::Scored as usize;
                                cbs = self
                                    .jokers
                                    .iter_mut()
                                    .enumerate()
                                    .sorted_by_key(|(_, joker)| {
                                        joker.dispatcher_order.events[event_usize]
                                    })
                                    .map_while(|(idx, joker)| {
                                        event_data.allowed.then(|| {
                                            joker
                                                .scored(&mut self.data, blind, &mut event_data)
                                                .map(|x| {
                                                    Box::new(misc::curry_mut(x, idx)) as Callback
                                                })
                                        })
                                    })
                                    .flatten()
                                    .collect_vec();

                                if !event_data.allowed {
                                    continue;
                                }

                                blind.draw(&self.data);
                                blind.score += blind.chips * blind.mult;

                                if blind.score >= blind.requirement {
                                    if matches!(blind.blind_type, Boss(_)) {
                                        if self.data.ante == 8 {
                                            return SimulationResult::Won;
                                        }

                                        self.data.ante += 1;
                                    }

                                    cbs.push(Box::new(|run| run.game_state = GameState::CashOut))
                                }
                            }
                            BlindAction::Abort => return SimulationResult::Aborted,
                        }
                    }
                }
                GameState::CashOut => match controller.cashout() {
                    CashoutAction::ReturnToShop => {
                        self.game_state = GameState::Shop;
                    }
                },
            };

            for mut cb in cbs {
                cb(&mut self);
            }
        }
    }
}
