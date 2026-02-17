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
use strum::{EnumCount, EnumIter};

pub struct Joker {
    pub joker_type: JokerType,
    pub edition: JokerEdition,
    pub stickers: Stickers,
    pub sell_value: u32,
    pub debuffed: bool,
    pub dispatcher_order: DispatcherOrder,
}

pub enum JokerEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Stickers {
    eternal: bool,
    perishable: bool,
    rental: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
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
    CeremonialDagger {
        mult: u32,
    },
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
    DNA {
        ready: bool,
    },
    Splash,
    BlueJoker,
    SixthSense {
        ready: bool,
    },
    Constellation,
    Hiker,
    FacelessJoker,
    GreenJoker,
    Superposition,
    ToDoList,
    Cavendish,
    CardSharp {
        played_before: [bool; HandType::COUNT],
    },
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
    TurtleBean {
        hand_size: u32,
    },
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
    TradingCard {
        ready: bool,
    },
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
    BurntJoker {
        ready: bool,
    },
    Bootstraps,
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

pub type PostExecCb = Box<dyn FnMut(usize, &mut Run)>;

impl Joker {
    #[must_use]
    pub fn blind_entered(&mut self) -> Option<PostExecCb> {
        match self.joker_type {
            CeremonialDagger { .. } => Some(Box::new(|index, run| {
                if run.jokers.len() == index + 1 || run.jokers[index + 1].cant_be_destroyed() {
                    return;
                }

                let joker = run.jokers.remove(index + 1);

                let CeremonialDagger { mult } = &mut run.jokers[index].joker_type else {
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
            if self.joker_type == joker {
                if event.hand.resolve(&data.cards).contains(hand_type) {
                    blind.mult += mult;
                }

                return None;
            }
        }

        for (joker, hand_type, chips) in Self::PLUS_CHIP_HANDTYPE_JOKERS {
            if self.joker_type == joker {
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
            CeremonialDagger { mult } => blind.mult += *mult as f64,
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
