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
}
pub enum CashoutAction {
    ReturnToShop,
}

pub trait Controller {
    fn shop(&mut self) -> Vec<ShopAction>;
    fn blind_selection(&mut self) -> BlindSelectionAction;
    fn blind(&mut self, blind: &mut Blind, data: &mut RunData) -> Vec<BlindAction>;
    fn cashout(&mut self) -> CashoutAction;
}

#[derive(Debug)]
pub enum SimulationResult {
    Lost { blind: Blind },
    Won,
}
