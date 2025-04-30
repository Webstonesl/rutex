use crate::error::Error;
pub mod mac_roman;
pub use mac_roman::MacRoman;

trait CharacterSetTable<T: Sized + Ord + Copy + 'static> {
    const CHARACTER_MAP: &[(char, T)];
    fn into_encoder() -> CharacterSetEncoder<T> {
        let mut decmap = BTreeMap::new();
        let mut encmap = BTreeMap::new();
        for (chr, val) in Self::CHARACTER_MAP {
            decmap.insert(*val, *chr);
        }
        for (chr, val) in Self::CHARACTER_MAP {
            encmap.insert(*chr, *val);
        }

        CharacterSetEncoder { decmap, encmap }
    }
}
struct CharacterSetEncoder<T: Sized + Ord + Copy> {
    decmap: BTreeMap<T, char>,
    encmap: BTreeMap<char, T>,
}

pub trait Encoding<T: Sized> {
    fn decode(&self, input: T) -> Result<char, Error>;
    fn encode(&self, output: char) -> Result<T, Error>;
}

pub struct DecodingIterator<T: Sized, E: Encoding<T>> {
    inner: Box<dyn Iterator<Item = T>>,
    decoder: Box<E>,
}
impl<T: Sized, E: Encoding<T>> DecodingIterator<T, E> {
    pub fn new(inner: Box<dyn Iterator<Item = T>>, decoder: E) -> Self {
        Self {
            inner,
            decoder: Box::new(decoder),
        }
    }
}
impl<T: Sized, E: Encoding<T>> Iterator for DecodingIterator<T, E> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some(v) => Some(self.decoder.decode(v)),
        }
    }
}
use std::{cell::LazyCell, collections::BTreeMap, marker::PhantomData};

pub struct EncodingIterator<T: Sized, E: Encoding<T>> {
    inner: Box<dyn Iterator<Item = char>>,
    encoder: Box<E>,
    _marker: PhantomData<T>,
}

impl<T: Sized, E: Encoding<T>> Iterator for EncodingIterator<T, E> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some(v) => Some(self.encoder.encode(v)),
        }
    }
}
