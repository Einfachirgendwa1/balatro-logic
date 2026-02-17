use crate::{
    card::{Card, MultiSuit},
    hands::Hand,
    joker::Joker,
};

pub struct HandPlayedEventData {
    pub hand: Hand,
    pub not_allowed: Box<dyn FnMut()>,
}

pub struct CardScoredEventData<'a> {
    pub card: &'a Card,
    pub hand_played: &'a mut HandPlayedEventData,
    pub suit: MultiSuit,
    pub face_card: bool,
}

pub struct AdditionalDataEventJoker<'a> {
    pub jokers_left: &'a mut [Joker],
    pub jokers_right: &'a mut [Joker],
    pub position: usize,
}
