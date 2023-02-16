


#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use crate::contract::queries::do_query;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::State;


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cluster2-continuation";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;



    Ok(Response::new()
        .add_attribute("action","instantiate")
        .add_attribute("status", "broken"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    let addr = _deps.api.addr_validate(env.contract.address.as_str())?;
    forward_my_token_to_addr(deps, env, addr.to_string())
}


fn forward_my_token_to_addr(deps: DepsMut, env:Env, addr: String) -> Result<Response, ContractError>{

    let msg = BankMsg::Send {
        to_address: addr ,
        amount: info.funds,
    };
    Ok(Response::new()
        .add_attribute("fw_token","from_test")
        .add_message(CosmosMsg::Bank(msg)))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    do_query(deps, env, msg)
}


pub mod queries{
    use cosmwasm_std::{Binary, Deps, Env, StdResult, to_binary};
    use crate::msg::QueryMsg;
    pub fn do_query(deps:Deps, env:Env, msg: QueryMsg)-> StdResult<Binary> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {}
