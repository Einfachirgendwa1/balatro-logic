use crate::{
    consumable::{Consumable, Spectral},
    run::RunData,
    seeding::math,
    shop::ShopItem,
};
use num_traits::FromPrimitive;
use std::{array, marker::PhantomData};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ConsumableCreator<'a, const N: usize, T, Res = Consumable>
where
    T: 'a,
    Res: From<T> + From<Spectral>,
{
    type_key: &'a str,
    origin_key: &'a str,

    #[builder(default, setter(strip_option))]
    filter: Option<&'a dyn Fn(&T) -> bool>,

    #[builder(default, setter(strip_option))]
    soul: Option<Spectral>,

    #[builder(default, setter(strip_option))]
    order: Option<[T; N]>,

    #[builder(default, setter(skip))]
    _phantom_t: PhantomData<T>,

    #[builder(default, setter(skip))]
    _phantom_res: PhantomData<Res>,
}

impl<'a, const N: usize, T, Res> ConsumableCreator<'a, N, T, Res>
where
    Res: From<T> + From<Spectral>,
    T: FromPrimitive + PartialEq + Copy,
    Consumable: From<T>,
{
    pub fn create(self, data: &mut RunData) -> Res {
        let Self { type_key, origin_key, filter, soul, order, .. } = self;

        if let Some(spectral) = soul {
            math::randomseed(data.rng.seed(&format!("soul_{type_key}{}", data.ante)));

            if math::random() > 0.997 {
                return spectral.into();
            }
        }

        let order = order.unwrap_or_else(|| array::from_fn(|idx| T::from_usize(idx).unwrap()));

        let available = order.map(|t| {
            let consumable = Consumable::from(t);
            let shop_item = ShopItem::Consumable(consumable);

            filter.as_ref().is_none_or(|f| f(&t))
                && !data.consumables.contains(&consumable)
                && !data.shop.inventory.contains(&shop_item)
        });

        let idx = data.poll(&available, &format!("{type_key}{origin_key}{}", data.ante));
        order[idx].into()
    }
}
