use strum::EnumCount;

#[derive(Debug, Clone, PartialEq, Eq, EnumCount)]
pub enum Event {
    BlindEntered,
    Scored,
    CardScored,
    CardScoredSpade,
    CardScoredHeart,
    CardScoredClub,
    CardScoredDiamond,
}

#[derive(Default)]
pub struct DispatcherOrder {
    pub events: [i64; Event::COUNT],
}
