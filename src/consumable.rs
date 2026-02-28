use crate::hands::{
    HandType,
    HandType::{
        FiveOfAKind, Flush, FlushFive, FlushHouse, FourOfAKind, FullHouse, HighCard, Pair,
        Straight, StraightFlush, ThreeOfAKind, TwoPair,
    },
};
use derive_more::From;
use num_derive::FromPrimitive;
use strum::EnumCount;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, From)]
pub enum Consumable {
    TarotCard(Tarot),
    PlanetCard(Planet),
    SpectralCard(Spectral),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, FromPrimitive)]
pub enum Tarot {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmperor,
    TheHierophant,
    TheLovers,
    TheChariot,
    Justice,
    TheHermit,
    TheWheelOfFortune,
    Strength,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgement,
    TheWorld,
}

pub type Planet = HandType;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, FromPrimitive)]
pub enum Spectral {
    Familiar,
    Grim,
    Incantation,
    Talisman,
    Aura,
    Wraith,
    Sigil,
    Ouija,
    Ectoplasm,
    Immolate,
    Ankh,
    DejaVu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    TheSoul,
    BlackHole,
}

pub const PLANET_ORDER: [Planet; HandType::COUNT] = [
    Pair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    Flush,
    Straight,
    TwoPair,
    StraightFlush,
    HighCard,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
];
