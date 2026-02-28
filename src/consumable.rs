use crate::hands::{
    HandType,
    HandType::{
        FiveOfAKind, Flush, FlushFive, FlushHouse, FourOfAKind, FullHouse, HighCard, Pair,
        Straight, StraightFlush, ThreeOfAKind, TwoPair,
    },
};
use derive_more::From;
use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter};
use strum::{Display, EnumCount};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, From)]
pub enum Consumable {
    TarotCard(Tarot),
    PlanetCard(Planet),
    SpectralCard(Spectral),
}

impl Display for Consumable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Consumable::TarotCard(x) => Display::fmt(x, f),
            Consumable::PlanetCard(x) => Display::fmt(x, f),
            Consumable::SpectralCard(x) => Display::fmt(x, f),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumCount, FromPrimitive)]
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

impl Display for Planet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string())
    }
}

impl Planet {
    pub const fn as_string(self) -> &'static str {
        match self {
            HighCard => "Pluto",
            Pair => "Mercury",
            TwoPair => "Uranus",
            ThreeOfAKind => "Venus",
            Straight => "Saturn",
            Flush => "Jupiter",
            FullHouse => "Earth",
            FourOfAKind => "Mars",
            StraightFlush => "Neptune",
            FiveOfAKind => "Planet X",
            FlushHouse => "Ceres",
            FlushFive => "Eris",
        }
    }
}

#[repr(u8)]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, EnumCount, FromPrimitive)]
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
