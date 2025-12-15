use dbn::{
        decode::DbnMetadata,
        pretty, Action, BidAskPair, Dataset, MboMsg, Publisher, Record, Schema, Side, SymbolIndex,
        UNDEF_PRICE,
};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
};

use crate::orderbook::book::Book;

#[derive(Debug, Default)]
pub struct Market {
    books: HashMap<u32, Vec<(Publisher, Book)>>,
}
