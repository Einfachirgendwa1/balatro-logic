use crate::{
    event::DispatcherOrder,
    joker::{
        COMMON_JOKERS, Joker, JokerEdition, JokerInternalState, JokerType, LEGENDARY_JOKERS,
        RARE_JOKERS, Stickers, UNCOMMON_JOKERS,
    },
    run::Run,
    seeding::math,
    shop::ShopItem,
};
use JokerRarity::*;
use derive_more::From;
use itertools::Itertools;
use num_derive::FromPrimitive;
use strum::{EnumCount, EnumIter};
use typed_builder::TypedBuilder;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
pub enum JokerRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl JokerRarity {
    pub const fn pool(self) -> &'static [JokerType] {
        match self {
            Common => &COMMON_JOKERS,
            Uncommon => &UNCOMMON_JOKERS,
            Rare => &RARE_JOKERS,
            Legendary => &LEGENDARY_JOKERS,
        }
    }
}

#[derive(TypedBuilder)]
pub struct JokerCreator<'a> {
    origin_key: &'a str,
    joker_rarity: JokerRarityMode,

    #[builder(default, setter(strip_option))]
    filter: Option<&'a dyn Fn(&JokerType) -> bool>,

    #[builder(setter(strip_bool(fallback = set_dont_filter_on_showman)))]
    dont_filter_on_showman: bool,

    #[builder(default)]
    sell_value: u32,

    #[builder(setter(strip_bool(fallback = set_debuffed)))]
    debuffed: bool,

    #[builder(default=JokerEdition::Base)]
    edition: JokerEdition,

    #[builder(default)]
    stickers: Stickers,

    #[builder(setter(strip_bool(fallback = set_rental)))]
    rental: bool,

    #[builder(setter(strip_bool(fallback = set_perishable)))]
    perishable: bool,

    #[builder(setter(strip_bool(fallback = set_eternal)))]
    eternal: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumCount, From)]
pub enum JokerRarityMode {
    Single(JokerRarity),
    RandomNonLegendary,
}

impl<'a> JokerCreator<'a> {
    pub fn create(self, Run { data, jokers, .. }: &mut Run) -> Joker {
        let Self {
            origin_key,
            joker_rarity,
            filter,
            dont_filter_on_showman,
            edition,
            mut stickers,
            sell_value,
            debuffed,
            rental,
            eternal,
            perishable,
        } = self;

        stickers.eternal |= eternal;
        stickers.rental |= rental;
        stickers.perishable |= perishable;

        let rarity = match joker_rarity {
            JokerRarityMode::Single(rarity) => rarity,
            JokerRarityMode::RandomNonLegendary => {
                let seed = data.rng.seed(&format!("rarity{}{origin_key}", data.ante));
                math::randomseed(seed);

                match math::random() {
                    seed if seed > 0.95 => Rare,
                    seed if seed > 0.7 => Uncommon,
                    _ => Common,
                }
            }
        };

        let pool = rarity.pool();
        let pool_key = format!("Joker{}{origin_key}{}", rarity as u8 + 1, data.ante);

        let available = pool
            .iter()
            .map(|joker| {
                ((dont_filter_on_showman && data.showman) || filter.is_none_or(|func| func(joker)))
                    && (data.showman
                        || (!data.shop.inventory.iter().any(|shop_joker| match shop_joker {
                            ShopItem::Joker(shop_joker_type) => {
                                shop_joker_type.joker_type == *joker
                            }
                            _ => false,
                        }) && !jokers
                            .iter()
                            .any(|already_owned_joker| already_owned_joker.joker_type == *joker)))
            })
            .collect_vec();

        let joker_type = data.poll(&available, &pool_key);
        Joker {
            joker_type: pool[joker_type],
            data: JokerInternalState::None,
            edition,
            stickers,
            sell_value,
            debuffed,
            dispatcher_order: DispatcherOrder::default(),
        }
    }
}
