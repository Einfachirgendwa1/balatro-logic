use crate::{
    card::{Card, MultiSuit},
    hands::Hand,
};

pub struct HandPlayedEventData {
    pub hand: Hand,
    pub allowed: bool,
}

pub struct CardScoredEventData<'a> {
    pub card: &'a Card,
    pub hand_played: &'a mut HandPlayedEventData,
    pub suit: MultiSuit,
    pub face_card: bool,
}
