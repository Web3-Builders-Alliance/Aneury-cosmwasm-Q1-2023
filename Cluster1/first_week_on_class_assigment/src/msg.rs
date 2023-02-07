use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use crate::state::Counter;

#[cw_serde]
pub struct InstantiateMsg {
    pub  counter: Counter,
   // pub sender_addr: Addr
}

#[cw_serde]
pub enum ExecuteMsg {
    Decrement,
    Increment,
    IncrementBy{value: i32},
    DecrementBy{value: i32},
    ReflectFunds{amt: i128} /// it could be Coin ?
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetCurrentCounterResponse)]
    GetCurrentCounter{}
}
// We define a custom struct for each query response
#[cw_serde]
pub struct GetCurrentCounterResponse {
    pub counter: i32,
}
