use crate::card::Card;
use itertools::Itertools;
use std::borrow::Borrow;

pub enum SortMode {
    SortByRank,
    SortBySuit,
}

pub trait PrintCards {
    fn print_cards(&self);
    fn print_cards_compact(&self, sort_mode: SortMode);
}

impl<T> PrintCards for Vec<T>
where
    T: Borrow<Card>,
{
    fn print_cards(&self) {
        println!("=================== CARDS ===================");
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
        println!("=============================================");
    }

    fn print_cards_compact(&self, sort_mode: SortMode) {
        let sorting = move |a: &T, b: &T| match sort_mode {
            SortMode::SortByRank => (*a)
                .borrow()
                .rank
                .cmp(&(*b).borrow().rank)
                .then((*a).borrow().suit.cmp(&(*b).borrow().suit))
                .reverse(),

            SortMode::SortBySuit => (*a)
                .borrow()
                .suit
                .cmp(&(*b).borrow().suit)
                .reverse()
                .then((*a).borrow().rank.cmp(&(*b).borrow().rank))
                .reverse(),
        };

        let mut sorted = self.iter().sorted_by(|a, b| sorting(a, b)).collect_vec();

        let Some(last) = sorted.pop() else {
            return;
        };

        for card in sorted {
            print!("{}, ", (*card).borrow());
        }

        println!("{}", (*last).borrow());
    }
}
