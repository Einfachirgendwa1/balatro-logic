use std::{
    fmt::{Debug, Display},
    iter::Map,
};

impl<T> Also for T {}
pub trait Also: Sized {
    fn also(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }

    fn also_mut(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}

impl<T: Debug> Log for T {}
pub trait Log: Sized + Debug {
    fn log(self) -> Self {
        self.also(|ts| println!("{ts:?}"))
    }
    fn msg(self, msg: impl Display) -> Self {
        self.also(|ts| println!("{msg} {ts:?}"))
    }
}

impl<A, B, T: Iterator<Item = (A, B)>> UnpackedMap<A, B> for T {}
pub trait UnpackedMap<A, B>: Iterator<Item = (A, B)> + Sized {
    fn map2<F, C>(self, mut func: F) -> Map<Self, impl FnMut(Self::Item) -> C>
    where
        F: FnMut(A, B) -> C + 'static,
    {
        self.map(move |(a, b)| func(a, b))
    }
}

pub fn curry_mut<F, P1, P2, R>(mut f: F, p1: P1) -> impl for<'a> FnMut(&'a mut P2) -> R
where
    P1: Clone + Copy + 'static,
    F: FnMut(P1, &mut P2) -> R + 'static,
{
    move |p2| f(p1, p2)
}
