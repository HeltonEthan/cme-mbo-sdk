use dbn::{Action, MboMsg, Side};

use crate::api::latency::{self, LatencyModel};

pub enum Request {
    Trade(TradeRequest),
    Cancel(CancelRequest),
    Modify(ModifyRequest),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ack {
    
}

impl Request {
    pub fn process<LM: LatencyModel>(self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        match self {
            Request::Trade(request) => request.submit(mbo, latency),
            Request::Cancel(request) => request.submit(mbo, latency),
            Request::Modify(request) => request.submit(mbo, latency),
        }
    }
}

#[derive(Debug)]
pub struct Order {
    ts_recv: u64,
    ts_event: u64,
    instrument_id: u32,
    action: Action,
    side: Side,
    price: Option<i64>,
    size: Option<u32>,
    order_id: Option<u64>,
    state: OrderState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderState {
    Pending,
    Live,
    Done,
    Rejected,
    Canceled,
}

impl Order {
    pub fn new(ts_event: u64, ts_recv: u64, instrument_id: u32, action: Action, side: Side, price: Option<i64>, size: Option<u32>) -> Self {
        Self {
            ts_recv,
            ts_event,
            instrument_id,
            action,
            side,
            price,
            size,
            order_id: None,
            state: OrderState::Pending,
        }
    }
}

pub trait Submit {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack;
}

#[derive(Debug)]
pub struct TradeRequest {
    pub instrument_id: u32,
    pub side: Side,
    pub price: i64,
    pub size: u32,
}

impl TradeRequest {}

impl Submit for TradeRequest {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        todo!()
    }
}

#[derive(Debug)]
pub struct CancelRequest {
    pub instrument_id: u32,
    pub order_id: u64,
}

impl CancelRequest {}

impl Submit for CancelRequest {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        todo!()
    }
}

#[derive(Debug)]
pub struct ModifyRequest {
    pub instrument_id: u32,
    pub order_id: u64,
    pub new_price: Option<i64>,
    pub new_size: Option<u32>,
}

impl ModifyRequest {}

impl Submit for ModifyRequest {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::latency::UnitNormalLatency;

    #[test]
    fn slippage() -> anyhow::Result<()> {
        let mut latency = UnitNormalLatency::new(25_000_000, 1_000_000);
        let ts_events: [u64; 5] = [1766368150000000000; 5];
        println!("=== Latency ===");
        for ts in ts_events {
            let ts_recv = latency.ts_recv_sim(ts);
            println!("{:#?}", ts_recv - ts);
        }
        Ok(())
    }
}
