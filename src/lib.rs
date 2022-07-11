#![doc = include_str!("../README.md")]

use std::marker::PhantomData;

pub trait Iterify: Sized {
    /// Takes any type `T` and through a closure `op` (which takes a mutable reference to `T`
    /// as its only parameter) yields an item `Option<I>`
    fn iterify<I, F: FnMut(&mut Self) -> Option<I>>(&mut self, op: F) -> Iterated<Self, I, F> {
        Iterated(self, op, PhantomData)
    }
}

pub struct Iterated<'a, K, I, F: FnMut(&mut K) -> Option<I>>(&'a mut K, F, PhantomData<K>);
impl<'a, K, I, F: FnMut(&mut K) -> Option<I>> Iterator for Iterated<'a, K, I, F> {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        (self.1)(self.0)
    }
}

impl<T> Iterify for T {}
