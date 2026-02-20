use crate::card::Card;
use itertools::Itertools;
use std::borrow::Borrow;

pub trait PrintCards {
    fn print_cards(&self);
}

impl<T> PrintCards for Vec<T>
where
    T: Borrow<Card>,
{
    fn print_cards(&self) {
        for (_, mut cards) in self
            .iter()
            .into_group_map_by(|card| (*card).borrow().suit)
            .into_iter()
            .sorted_by_key(|(suit, _)| *suit)
        {
            if cards.is_empty() {
                continue;
            }

            cards.sort_by(|a, b| (*a).borrow().rank.cmp(&(*b).borrow().rank).reverse());

            for card in &cards[..cards.len() - 1] {
                print!("{}, ", (*card).borrow());
            }
            println!("{}", (*cards.last().unwrap()).borrow());
        }
    }
}
