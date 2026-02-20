use crate::{blind::Blind, run::RunData};

pub enum ShopAction {
    ExitShop,
}

pub enum BlindSelectionAction {
    PlayBlind,
}

pub enum BlindAction {
    SelectCard(usize),
    Play,
    Discard,
    Abort,
}
pub enum CashoutAction {
    ReturnToShop,
}

pub trait Controller {
    fn shop(&mut self) -> Vec<ShopAction> {
        vec![ShopAction::ExitShop]
    }

    fn blind_selection(&mut self) -> BlindSelectionAction {
        BlindSelectionAction::PlayBlind
    }

    fn blind(&mut self, blind: &mut Blind, data: &mut RunData) -> Vec<BlindAction>;
    fn cashout(&mut self) -> CashoutAction {
        CashoutAction::ReturnToShop
    }
}

#[derive(Debug, PartialEq)]
pub enum SimulationResult {
    Lost { blind: Blind },
    Aborted,
    Won,
}
