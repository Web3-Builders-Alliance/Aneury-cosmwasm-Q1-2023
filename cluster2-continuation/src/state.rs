use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;


#[cw_serde]
pub struct State{
   denom: Denom,
   coin: Vec<Coin>,
   current_addr: Addr
}

pub const STATE: Item<State> = Item::new("state");
