use crate::{
    card::{
        Card, Rank,
        Rank::{Ace, Rank2, Rank3, Rank4, Rank5},
    },
    hands::HandType::*,
};
use itertools::Itertools;
use std::mem::MaybeUninit;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Default)]
pub struct Hand {
    pub cards: [usize; 5],
    pub len: usize,
}

pub struct ResolvedHand<'a>(pub Vec<&'a Card>);

impl Hand {
    pub fn card_slice(&self) -> &[usize] {
        &self.cards[..self.len]
    }

    pub fn resolve<'a>(&self, cards: &'a [Card]) -> ResolvedHand<'a> {
        ResolvedHand(self.card_slice().iter().map(|idx| &cards[*idx]).collect())
    }
}

impl ResolvedHand<'_> {
    pub fn contains(&self, hand_type: HandType) -> bool {
        let mut rank_counts = [0; Rank::COUNT];

        for card in &self.0 {
            rank_counts[card.rank as usize] += 1;
        }

        match hand_type {
            HighCard => true,
            Pair => rank_counts.into_iter().any(|count| count >= 2),
            TwoPair => rank_counts.into_iter().filter(|count| *count >= 2).count() >= 2,
            ThreeOfAKind => rank_counts.into_iter().any(|count| count >= 3),
            Straight => {
                if self.0.len() < 5 {
                    return false;
                }

                let mut ranks: Vec<Rank> = self.0.iter().map(|card| card.rank).collect();
                ranks.sort();

                let mut ace_to_5_straight = vec![Ace, Rank2, Rank3, Rank4, Rank5];
                ace_to_5_straight.sort();

                if ranks == ace_to_5_straight {
                    return true;
                }

                ranks[0] as u8 - 1 == ranks[1] as u8
                    && ranks[1] as u8 - 1 == ranks[2] as u8
                    && ranks[2] as u8 - 1 == ranks[3] as u8
                    && ranks[3] as u8 - 1 == ranks[4] as u8
            }
            Flush => self.0.len() == 5 && self.0.iter().all(|card| card.suit == self.0[0].suit),
            FullHouse => rank_counts.contains(&3) && rank_counts.contains(&2),
            FourOfAKind => rank_counts.contains(&4),
            FiveOfAKind => rank_counts.contains(&5),
            StraightFlush => self.contains(Straight) && self.contains(Flush),
            FlushHouse => self.contains(Flush) && self.contains(FullHouse),
            FlushFive => self.contains(Flush) && self.contains(FiveOfAKind),
        }
    }

    pub fn ranks(&self) -> impl Iterator<Item = u8> {
        self.0.iter().map(|x| x.rank as _)
    }

    pub fn hand_type(&self) -> HandType {
        for hand_type in HandType::iter().rev() {
            if self.contains(hand_type) {
                return hand_type;
            }
        }

        unreachable!()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandType {
    fn gen_values<T: Default>(f: impl Fn(HandType) -> T) -> [T; HandType::COUNT] {
        let mut array = [const { MaybeUninit::uninit() }; HandType::COUNT];
        for (idx, hand_type) in HandType::iter().enumerate() {
            array[idx] = MaybeUninit::new(f(hand_type));
        }

        array.map(|elem| unsafe { elem.assume_init() })
    }

    pub fn base_chips() -> [u64; HandType::COUNT] {
        Self::gen_values(|hand| match hand {
            HighCard => 5,
            Pair => 10,
            TwoPair => 20,
            ThreeOfAKind | Straight => 30,
            Flush => 35,
            FullHouse => 40,
            FourOfAKind => 60,
            StraightFlush => 100,
            FiveOfAKind => 120,
            FlushHouse => 140,
            FlushFive => 160,
        })
    }

    pub fn base_mult() -> [u64; HandType::COUNT] {
        Self::gen_values(|hand| match hand {
            HighCard => 1,
            Pair | TwoPair => 2,
            ThreeOfAKind => 3,
            Straight | Flush | FullHouse => 4,
            FourOfAKind => 7,
            StraightFlush => 8,
            FiveOfAKind => 12,
            FlushHouse => 14,
            FlushFive => 16,
        })
    }
}
