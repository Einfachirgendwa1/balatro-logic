use crate::card::{Enhancement::WildCard, Rank::*, Suit::*};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter}
    ,
};
use strum::{EnumCount, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Enhancement,
    pub edition: Edition,
    pub seal: Seal,
    pub chips: u32,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            rank,
            suit,
            enhancement: Enhancement::None,
            edition: Edition::Base,
            seal: Seal::None,
            chips: match rank {
                Rank2 => 2,
                Rank3 => 3,
                Rank4 => 4,
                Rank5 => 5,
                Rank6 => 6,
                Rank7 => 7,
                Rank8 => 8,
                Rank9 => 9,
                Rank10 | Jack | Queen | King => 10,
                Ace => 11,
            },
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, EnumCount, EnumIter)]
pub enum Rank {
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
    Rank9,
    Rank10,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank2 => write!(f, "2"),
            Rank3 => write!(f, "3"),
            Rank4 => write!(f, "4"),
            Rank5 => write!(f, "5"),
            Rank6 => write!(f, "6"),
            Rank7 => write!(f, "7"),
            Rank8 => write!(f, "8"),
            Rank9 => write!(f, "9"),
            Rank10 => write!(f, "10"),
            Jack => write!(f, "Jack"),
            Queen => write!(f, "Queen"),
            King => write!(f, "King"),
            Ace => write!(f, "Ace"),
        }
    }
}

pub static ALPHABETICAL_RANK_ORDER: [Rank; Rank::COUNT] = [
    Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8, Rank9, Ace, Jack, King, Queen, Rank10,
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

pub static ALPHABETICAL_SUIT_ORDER: [Suit; Suit::COUNT] = [Club, Diamond, Heart, Spade];

pub struct MultiSuit {
    pub spade: bool,
    pub heart: bool,
    pub club: bool,
    pub diamond: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
pub enum Enhancement {
    None,
    BonusCard,
    MultCard,
    WildCard,
    GlassCard,
    SteelCard,
    StoneCard,
    GoldCard,
    LuckyCard,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
pub enum Edition {
    Base,
    Foil,
    Holographic,
    Polychrome,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
pub enum Seal {
    None,
    Gold,
    Red,
    Blue,
    Purple,
}

impl Card {
    pub(crate) fn is_suit(&self, suit: Suit) -> bool {
        self.suit == suit || self.enhancement == WildCard
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.seal != Seal::None {
            write!(f, "{:?}", self.seal)?;
        }

        if self.edition != Edition::Base {
            write!(f, "{:?}", self.edition)?;
        }

        if self.enhancement != Enhancement::None {
            write!(f, "{:?}", self.enhancement)?;
        }

        write!(f, "{} of {:?}s", self.rank, self.suit)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.suit
            .cmp(&other.suit)
            .reverse()
            .then(self.rank.cmp(&other.rank))
    }
}

impl Rank {
    pub(crate) fn is_face_card(&self) -> bool {
        *self == Jack || *self == Queen || *self == King
    }
}
