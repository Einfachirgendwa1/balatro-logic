use crate::{
    card::{ALPHABETICAL_RANK_ORDER, ALPHABETICAL_SUIT_ORDER, Card},
    misc::UnpackedMap,
};
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
