use num_derive::FromPrimitive;
use strum::{Display, EnumCount, EnumIter};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive, Display)]
pub enum Voucher {
    Overstock,
    OverstockPlus,
    ClearanceSale,
    Liquidation,
    Hone,
    GlowUp,
    RerollSurplus,
    RerollGlut,
    CrystalBall,
    OmenGlobe,
    Telescope,
    Observatory,
    Grabber,
    NachoTong,
    Wasteful,
    Recyclomancy,
    TarotMerchant,
    TarotTycoon,
    PlanetMerchant,
    PlanetTycoon,
    SeedMoney,
    MoneyTree,
    Blank,
    Antimatter,
    MagicTrick,
    Illusion,
    Hieroglyph,
    Petroglyph,
    DirectorsCut,
    Retcon,
    PaintBrush,
    Palette,
}

impl Voucher {
    #[inline]
    #[must_use]
    pub fn requirements_fulfilled(self, voucher_list: &[bool; Voucher::COUNT]) -> bool {
        (self as usize).is_multiple_of(2) || voucher_list[self as usize - 1]
    }
}
