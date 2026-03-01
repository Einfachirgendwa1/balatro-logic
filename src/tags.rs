use crate::{
    run::RunData,
    tags::Tag::{
        BossTag, CharmTag, CouponTag, D6Tag, DoubleTag, EconomyTag, FoilTag, HolographicTag,
        InvestmentTag, JuggleTag, PolychromeTag, RareTag, SpeedTag, UncommonTag, VoucherTag,
    },
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::array;
use strum::{EnumCount, EnumIter};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
pub enum Tag {
    UncommonTag,
    RareTag,
    NegativeTag,
    FoilTag,
    HolographicTag,
    PolychromeTag,
    InvestmentTag,
    VoucherTag,
    BossTag,
    StandardTag,
    CharmTag,
    MeteorTag,
    BuffoonTag,
    HandyTag,
    GarbageTag,
    EtherealTag,
    CouponTag,
    DoubleTag,
    JuggleTag,
    D6Tag,
    TopUpTag,
    SpeedTag,
    OrbitalTag,
    EconomyTag,
}

const FIRST_ANTE_TAGS: [Tag; 15] = [
    BossTag,
    CharmTag,
    CouponTag,
    D6Tag,
    DoubleTag,
    EconomyTag,
    FoilTag,
    HolographicTag,
    InvestmentTag,
    JuggleTag,
    PolychromeTag,
    RareTag,
    SpeedTag,
    UncommonTag,
    VoucherTag,
];

impl RunData {
    pub fn next_random_tag(&mut self) -> Tag {
        let available: [bool; Tag::COUNT] = array::from_fn(|idx| {
            FIRST_ANTE_TAGS.contains(&Tag::from_usize(idx).unwrap()) || self.ante >= 2
        });

        Tag::from_usize(self.poll(&available, &format!("Tag{}", self.ante))).unwrap()
    }
}
