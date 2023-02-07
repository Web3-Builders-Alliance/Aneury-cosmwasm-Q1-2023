
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use cw2::set_contract_version;
use schemars::_private::NoSerialize;
use schemars::_serde_json::Value;
use crate::contract::execute::{decrement_by, increment_by};
use crate::ContractError::{Std, Unauthorized};

use crate::error::ContractError;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{COUNTER_STATE, CounterState};

/*
    Task Requirement

    Create Execute Message for
    Increment -> this message increment a counter by 1;
    Decrement -> this message decrement a counter by 1;

    IncrementBy -> this message increment a counter by n;
    DecrementBy -> this message decrement a counter by n;

    ReflectFunds -> sends funds to a contract and then return that amount back to sender


 */


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cwcourse";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
     deps: DepsMut,
     env: Env,
     info: MessageInfo,
     msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let counter_value = CounterState{
         counter: msg.counter,
         sender_addr: msg.sender_addr
    };

    COUNTER_STATE.save(deps.storage, &counter_value).unwrap();

    Ok(Response::new()
        .add_attribute("action","instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::Decrement{}=> execute::decrement(deps),
        ExecuteMsg::Increment{}=> execute::increment(deps),
        ExecuteMsg::DecrementBy{value}=> execute::decrement_by(deps, value),
        ExecuteMsg::IncrementBy{value}=>  execute::increment_by(deps, value),
        ExecuteMsg::ReflectFunds {amt}=>execute::reflects_funds(deps, env, info, amt),
    }

    ///Ok(Response::new())

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query::get_current_counter(deps)?),
    }
}

pub mod query{
    use cosmwasm_std::{Deps, StdResult};
    use crate::msg::GetCurrentCounterResponse;
    use crate::state::COUNTER_STATE;

    pub fn get_current_counter(deps: Deps) -> StdResult<GetCurrentCounterResponse>{
        let current_counter = COUNTER_STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: current_counter.counter })
    }
}

pub mod execute{
    use super::*;
    use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Response};
    use crate::ContractError;
    use crate::state::COUNTER_STATE;

    pub fn increment(deps: DepsMut)->Result<Response,ContractError>{

        COUNTER_STATE.update(deps.storage, |mut counter_state| -> Result<_, ContractError> {
            counter_state.counter += 1;
            Ok(counter_state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }
    pub fn decrement(deps: DepsMut)->Result<Response,ContractError>{

        COUNTER_STATE.update(deps.storage, |mut counter_state| -> Result<_, ContractError> {
            counter_state.counter -= 1;
            Ok(counter_state)
        })?;

        Ok(Response::new().add_attribute("action", "decrement"))
    }
    pub  fn increment_by(deps: DepsMut, increment_value: i32) ->Result<Response,ContractError>{

        COUNTER_STATE.update(deps.storage, |mut counter_state| -> Result<_, ContractError> {
            counter_state.counter += increment_value;
            Ok(counter_state)
        })?;

        Ok(Response::new().add_attribute("action", "increment_by"))
    }
    pub  fn decrement_by(deps: DepsMut, decrement_value: i32) ->Result<Response,ContractError>{

        COUNTER_STATE.update(deps.storage, |mut counter_state| -> Result<_, ContractError> {
            counter_state.counter -= decrement_value;
            Ok(counter_state)
        })?;

        Ok(Response::new().add_attribute("action", "decrement_by"))
    }


    pub fn reflects_funds(deps:DepsMut, env: Env, info: MessageInfo, funds: i128)->Result<Response,ContractError>{

        //todo: Verify Sender Balance update? then send, using example that Javier show and then return msg with some payload?
        let mut response = Response::new();
        response.add_attribute("action", "reflect_funds");
        response.add_attribute("amount", amount.to_string());
        response.add_attribute("sender", info.sender);
        response.add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin
            {
                denom: "nano".to_string(),
                amount:amount,
            }
            ],
        }));
        Ok(response)
    }
}



#[cfg(test)]
mod tests {
     use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn test(){
        assert_eq!(1,1);
    }
    #[test]
    fn test_increment(){
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { counter: 43};
        let info = mock_info("creator", &coins(243, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("who_whish", &coins(43, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res =  execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentCounter {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(44, value.count);
    }
    fn test_decrement(){
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { counter: 43};
        let info = mock_info("creator", &coins(243, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("who_whish", &coins(43, "token"));
        let msg = ExecuteMsg::Decrement {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentCounter {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(42, value.count);
    }
    fn test_increment_by(){
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { counter: 43};
        let info = mock_info("creator", &coins(243, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("who_whish", &coins(43, "token"));
        let msg = ExecuteMsg::IncrementBy {value:1};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentCounter {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(44, value.count);
    }
    fn test_decrement_by()
    {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { counter: 43};
        let info = mock_info("creator", &coins(243, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("who_whish", &coins(43, "token"));
        let msg = ExecuteMsg::IncrementBy {value:1};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentCounter {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(44, value.count);
    }
    fn test_reflect_funds(){}


}

