use std::collections::HashMap;

use crate::api::action::Order;

pub struct Orders {
    queue_by_id: HashMap<u64, Order>,
    active_by_id: HashMap<u64, Order>,
}

impl Orders {
    
}
