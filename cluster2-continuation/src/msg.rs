use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    denom: Denom,
    coin: Vec<Coin>
}

#[cw_serde]
pub enum ExecuteMsg {
  FowardTokensToAddr{addr: Addr}
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
