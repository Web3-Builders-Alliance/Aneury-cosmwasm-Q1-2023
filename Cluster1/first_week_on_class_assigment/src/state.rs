use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use cosmwasm_std::Addr;
use cosmwasm_schema::{cw_serde};

#[cw_serde]
pub struct CounterState{
    pub counter:  i32 ,
    pub sender_addr: Addr
}

pub const COUNTER_STATE: Item<CounterState> = Item::new("counter_state");
