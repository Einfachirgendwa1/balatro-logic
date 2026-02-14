use crate::{blind::Blind, event::OnBlindEntered, hands::HandType, joker::JokerType::*};
use strum::{EnumCount, EnumIter};

pub struct Joker {
    pub(crate) joker_type: JokerType,
    pub(crate) edition: JokerEdition,
    pub(crate) stickers: Stickers,
    pub(crate) sell_value: u32,
    pub(crate) debuffed: bool,
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
        multiplier: u32,
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

impl Joker {
    pub(crate) fn on_blind_entered(idx: usize, event: &mut OnBlindEntered, blind: &mut Blind) {
        match &mut event.run.jokers[idx].joker_type {
            CeremonialDagger { .. } => {
                let sell_value = event.run.jokers[idx + 1].sell_value;

                let CeremonialDagger { multiplier } = &mut event.run.jokers[idx].joker_type else {
                    panic!()
                };

                *multiplier += sell_value * 2;
                event.run.jokers.remove(idx + 1);
            }

            MarbleJoker => todo!(),

            Burglar => {
                blind.hands += 3;
                blind.discards = 0;
            }

            DNA { ready } => *ready = true,
            SixthSense { ready } => *ready = true,
            TradingCard { ready } => *ready = true,
            BurntJoker { ready } => *ready = true,

            CardSharp { played_before } => *played_before = [false; HandType::COUNT],

            Madness => todo!(),
            RiffRaff => todo!(),

            TurtleBean { hand_size } => event.run.hand_size += *hand_size,

            Drunkard => blind.discards += 1,
            Troubadour => blind.hands -= 1,

            Certificate => todo!(),

            MerryAndy => blind.discards += 3,

            Brainstorm => todo!(),
            Blueprint => todo!(),
            Cartomancer => todo!(),

            _ => {}
        }
    }
}
