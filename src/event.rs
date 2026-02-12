use crate::{hands::HandType, run::Run};

pub(crate) struct OnHandPlayed<'a> {
    pub(crate) run: &'a mut Run,
    pub(crate) hand_type: HandType,
    pub(crate) not_allowed: Box<dyn FnOnce()>,
}

pub(crate) struct OnBlindEntered<'a> {
    pub(crate) run: &'a mut Run,
}
