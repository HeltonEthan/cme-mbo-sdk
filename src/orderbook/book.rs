use dbn::{
        decode::DbnMetadata,
        pretty, Action, BidAskPair, Dataset, MboMsg, Publisher, Record, Schema, Side, SymbolIndex,
        UNDEF_PRICE,
};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
};

#[derive(Debug, Default)]
pub struct Book {
    orders_by_id: HashMap<u64, (Side, i64)>,
    offers: BTreeMap<i64, Level>,
    bids: BTreeMap<i64, Level>
}

type Level = VecDeque<MboMsg>;
