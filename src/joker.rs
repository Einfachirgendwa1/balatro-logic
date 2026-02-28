use crate::{
    blind::Blind,
    event::DispatcherOrder,
    event_list::{CardScoredEventData, HandPlayedEventData},
    hands::{
        HandType,
        HandType::{Flush, Pair, Straight, ThreeOfAKind, TwoPair},
    },
    joker::JokerType::*,
    run::{Run, RunData},
};
use num_derive::FromPrimitive;
use std::{
    cell::LazyCell,
    fmt::{Display, Formatter},
    mem::discriminant,
};
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};

#[derive(PartialEq, Debug, Clone)]
pub struct Joker {
    pub data: JokerInternalState,
    pub joker_type: JokerType,
    pub edition: JokerEdition,
    pub stickers: Stickers,
    pub sell_value: u32,
    pub debuffed: bool,
    pub dispatcher_order: DispatcherOrder,
}

impl Display for Joker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.stickers.perishable {
            write!(f, "Perishable ")?;
        }

        if self.stickers.rental {
            write!(f, "Rental ")?;
        }

        if self.stickers.eternal {
            write!(f, "Eternal ")?;
        }

        if self.edition != JokerEdition::Base {
            write!(f, "{} ", self.edition)?;
        }

        write!(f, "{}", self.joker_type)?;

        if self.debuffed {
            write!(f, " (debuffed)")?;
        }

        Ok(())
    }
}
#[derive(PartialEq, Debug, Display, Copy, Clone)]
pub enum JokerEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Stickers {
    pub eternal: bool,
    pub perishable: bool,
    pub rental: bool,
}

thread_local! {
    static INTERNALS: LazyCell<Vec<JokerInternalState >> = LazyCell::new(|| JokerInternalState::iter().collect());
}

impl PartialEq<JokerInternalState> for JokerType {
    fn eq(&self, other: &JokerInternalState) -> bool {
        let usize_repr = INTERNALS
            .with(|internal| {
                internal.iter().position(|variant| discriminant(variant) == discriminant(&other))
            })
            .unwrap();

        *self as usize == usize_repr
    }
}

impl PartialEq<JokerType> for JokerInternalState {
    fn eq(&self, other: &JokerType) -> bool {
        other == self
    }
}

pub type PostExecCb = Box<dyn FnMut(usize, &mut Run)>;

impl Joker {
    #[must_use]
    pub fn blind_entered(&mut self) -> Option<PostExecCb> {
        match self.joker_type {
            CeremonialDagger => Some(Box::new(|index, run| {
                if run.jokers.len() == index + 1 || run.jokers[index + 1].cant_be_destroyed() {
                    return;
                }

                let joker = run.jokers.remove(index + 1);
                let JokerInternalState::CeremonialDagger { mult } = &mut run.jokers[index].data
                else {
                    panic!()
                };

                *mult += joker.sell_value;
            })),
            _ => None,
        }
    }

    const PLUS_MULT_HANDTYPE_JOKERS: [(JokerType, HandType, f64); 5] = [
        (JollyJoker, Pair, 8.),
        (ZanyJoker, ThreeOfAKind, 12.),
        (MadJoker, TwoPair, 10.),
        (CrazyJoker, Straight, 12.),
        (DrollJoker, Flush, 10.),
    ];

    const PLUS_CHIP_HANDTYPE_JOKERS: [(JokerType, HandType, f64); 5] = [
        (SlyJoker, Pair, 50.),
        (WilyJoker, ThreeOfAKind, 100.),
        (CleverJoker, TwoPair, 80.),
        (DeviousJoker, Straight, 100.),
        (CraftyJoker, Flush, 80.),
    ];

    pub fn scored(
        &mut self,
        data: &mut RunData,
        blind: &mut Blind,
        event: &mut HandPlayedEventData,
    ) -> Option<PostExecCb> {
        for (joker, hand_type, mult) in Self::PLUS_MULT_HANDTYPE_JOKERS {
            if self.data == joker {
                if event.hand.resolve(&data.cards).contains(hand_type) {
                    blind.mult += mult;
                }

                return None;
            }
        }

        for (joker, hand_type, chips) in Self::PLUS_CHIP_HANDTYPE_JOKERS {
            if self.data == joker {
                if event.hand.resolve(&data.cards).contains(hand_type) {
                    blind.chips += chips;
                }

                return None;
            }
        }

        match &self.joker_type {
            Joker => blind.mult += 4.,
            HalfJoker => {
                if event.hand.len <= 3 {
                    blind.mult += 20.
                }
            }
            Banner => blind.chips += blind.discards as f64 * 30.,
            MysticSummit => {
                if blind.discards == 0 {
                    blind.mult += 15.
                }
            }
            RaisedFist => {
                let smallest_rank = event.hand.resolve(&data.cards).ranks().min().unwrap() * 2;
                blind.mult += smallest_rank as f64
            }
            CeremonialDagger => {
                let JokerInternalState::CeremonialDagger { mult } = &mut self.data else {
                    unreachable!()
                };

                blind.mult += *mult as f64
            }
            _ => {}
        }

        None
    }

    fn card_scored(&mut self, blind: &mut Blind, event: &mut CardScoredEventData) {
        match &self.joker_type {
            SmearedJoker => {
                event.suit.spade |= event.suit.club;
                event.suit.club |= event.suit.spade;
                event.suit.heart |= event.suit.diamond;
                event.suit.diamond |= event.suit.heart;
            }
            WrathfulJoker => {
                if event.suit.spade {
                    blind.mult += 3.
                }
            }
            LustyJoker => {
                if event.suit.heart {
                    blind.mult += 3.
                }
            }
            GluttonousJoker => {
                if event.suit.club {
                    blind.mult += 3.
                }
            }
            GreedyJoker => {
                if event.suit.diamond {
                    blind.mult += 3.
                }
            }
            _ => {}
        }
    }
}

impl Joker {
    fn cant_be_destroyed(&self) -> bool {
        self.stickers.eternal
    }
}

pub static COMMON_JOKERS: [JokerType; 61] = [
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,
    HalfJoker,
    CreditCard,
    Banner,
    MysticSummit,
    EightBall,
    Misprint,
    RaisedFist,
    ChaosTheClown,
    ScaryFace,
    AbstractJoker,
    DelayedGratification,
    GrosMichel,
    EvenSteven,
    OddTodd,
    Scholar,
    BusinessCard,
    Supernova,
    RideTheBus,
    Egg,
    Runner,
    IceCream,
    Splash,
    BlueJoker,
    FacelessJoker,
    GreenJoker,
    Superposition,
    ToDoList,
    Cavendish,
    RedCard,
    SquareJoker,
    RiffRaff,
    Photograph,
    ReservedParking,
    MailInRebate,
    Hallucination,
    FortuneTeller,
    Juggler,
    Drunkard,
    GoldenJoker,
    Popcorn,
    WalkieTalkie,
    SmileyFace,
    GoldenTicket,
    Swashbuckler,
    HangingChad,
    ShootTheMoon,
];

pub static UNCOMMON_JOKERS: [JokerType; 64] = [
    JokerStencil,
    FourFingers,
    Mime,
    CeremonialDagger,
    MarbleJoker,
    LoyaltyCard,
    Dusk,
    Fibonacci,
    SteelJoker,
    Hack,
    Pareidolia,
    SpaceJoker,
    Burglar,
    Blackboard,
    SixthSense,
    Constellation,
    Hiker,
    CardSharp,
    Madness,
    Seance,
    Vampire,
    Shortcut,
    Hologram,
    Cloud9,
    Rocket,
    MidasMask,
    Luchador,
    GiftCard,
    TurtleBean,
    Erosion,
    ToTheMoon,
    StoneJoker,
    LuckyCat,
    Bull,
    DietCola,
    TradingCard,
    FlashCard,
    SpareTrousers,
    Ramen,
    Seltzer,
    Castle,
    MrBones,
    Acrobat,
    SockAndBuskin,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    Showman,
    FlowerPot,
    MerryAndy,
    OopsAll6s,
    TheIdol,
    SeeingDouble,
    Matador,
    Satellite,
    Cartomancer,
    Astronomer,
    Bootstraps,
];

pub static RARE_JOKERS: [JokerType; 20] = [
    DNA,
    Vagabond,
    Baron,
    Obelisk,
    BaseballCard,
    AncientJoker,
    Campfire,
    Blueprint,
    WeeJoker,
    HitTheRoad,
    TheDuo,
    TheTrio,
    TheFamily,
    TheOrder,
    TheTribe,
    Stuntman,
    InvisibleJoker,
    Brainstorm,
    DriversLicense,
    BurntJoker,
];

pub static LEGENDARY_JOKERS: [JokerType; 5] = [Canio, Triboulet, Yorick, Chicot, Perkeo];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive, Display)]
pub enum JokerType {
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,
    HalfJoker,
    JokerStencil,
    FourFingers,
    Mime,
    CreditCard,
    CeremonialDagger,
    Banner,
    MysticSummit,
    MarbleJoker,
    LoyaltyCard,
    EightBall,
    Misprint,
    Dusk,
    RaisedFist,
    ChaosTheClown,
    Fibonacci,
    SteelJoker,
    ScaryFace,
    AbstractJoker,
    DelayedGratification,
    Hack,
    Pareidolia,
    GrosMichel,
    EvenSteven,
    OddTodd,
    Scholar,
    BusinessCard,
    Supernova,
    RideTheBus,
    SpaceJoker,
    Egg,
    Burglar,
    Blackboard,
    Runner,
    IceCream,
    DNA,
    Splash,
    BlueJoker,
    SixthSense,
    Constellation,
    Hiker,
    FacelessJoker,
    GreenJoker,
    Superposition,
    ToDoList,
    Cavendish,
    CardSharp,
    RedCard,
    Madness,
    SquareJoker,
    Seance,
    RiffRaff,
    Vampire,
    Shortcut,
    Hologram,
    Vagabond,
    Baron,
    Cloud9,
    Rocket,
    Obelisk,
    MidasMask,
    Luchador,
    Photograph,
    GiftCard,
    TurtleBean,
    Erosion,
    ReservedParking,
    MailInRebate,
    ToTheMoon,
    Hallucination,
    FortuneTeller,
    Juggler,
    Drunkard,
    StoneJoker,
    GoldenJoker,
    LuckyCat,
    BaseballCard,
    Bull,
    DietCola,
    TradingCard,
    FlashCard,
    Popcorn,
    SpareTrousers,
    AncientJoker,
    Ramen,
    WalkieTalkie,
    Seltzer,
    Castle,
    SmileyFace,
    Campfire,
    GoldenTicket,
    MrBones,
    Acrobat,
    SockAndBuskin,
    Swashbuckler,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    HangingChad,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    Showman,
    FlowerPot,
    Blueprint,
    WeeJoker,
    MerryAndy,
    OopsAll6s,
    TheIdol,
    SeeingDouble,
    Matador,
    HitTheRoad,
    TheDuo,
    TheTrio,
    TheFamily,
    TheOrder,
    TheTribe,
    Stuntman,
    InvisibleJoker,
    Brainstorm,
    Satellite,
    ShootTheMoon,
    DriversLicense,
    Cartomancer,
    Astronomer,
    BurntJoker,
    Bootstraps,
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, EnumCount, EnumIter)]
pub enum JokerInternalState {
    None,
    DNA(bool),
    SixthSense(bool),
    TradingCard(bool),
    BurntJoker(bool),
    CeremonialDagger { mult: u32 },
    CardSharp { played_before: [bool; HandType::COUNT] },
    TurtleBean { hand_size: u32 },
}
