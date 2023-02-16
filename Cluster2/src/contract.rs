#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, BankMsg, Addr, ensure_eq, SubMsg, coin, coins, ReplyOn, Reply};
use cosmwasm_std::QueryRequest::Bank;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cluster2";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CALLBACK_ID: U64 = 1u64;


/*
   Explanation about  Liquidity pool.

 */


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {



     let state = State {
        count: msg.count,

    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply)->Result<Response, ContractError>{

    ///deps.api.addr_validate("r344343")?
    ///
    match msg.id{
        CALLBACK_ID => {
            STATE.save(deps.storage,  &true)?;
        },
        ANOTHER_CALLBACK_ID=>{
            ///Another Sa
        }
    }

   unimplemented!()
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {


    match msg {
        ExecuteMsg::Reset {count:0}=>{},
        ExecuteMsg::Decrement {}=>{},
        ExecuteMsg::Increment{}=>{},
        ExecuteMsg::DecrementBy {count:i1}=>{},
        ExecuteMsg::IncrementBy {count:i1}=>{},
        ExecuteMsg::ReflectFunds {}=>{

            if !info.funds.is_empty() {
                let bank_message = BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: info.funds
                };

                let funds = coins(100,"ustarts");

                let current_balance = deps.querier.query_balance(&info.sender, "ustars")?;

                let bank_message = BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: vec![current_balance],
                };

                let cosmosmsg = CosmosMsg::Bank(bank_message);

                let sub_msg = SubMsg{
                    id: CALLBACK_ID,
                    msg: cosmosmsg,
                    gas_limit: None,
                    reply_on: ReplyOn::Success
                };


                Ok(Response::new()
                    .add_submessage(sub_msg)
                    .add_message(cosmomsg)
                    .add_attribute("action", "reflect_funds"))
                    .add_attribute("amount", 123.to_string())
            }
            else{
                Err(ContractError::Unauthorized {})
            }
        }
        _ => {}
    }

    unimplemented!()




    // let bank = BankMsg::Send{
    //     to_address: "".to_string(),
    //     amount: vec![],
    // };
    // let msg_from_cosmo = CosmosMsg::from(bank);
    //
    // match msg {
    //     ExecuteMsg::Increment {} => execute::increment(deps),
    //     ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
    // }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {


   unimplemented!()
    //
    // match msg {
    //     QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    // }
}




pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }
}
pub mod execute {
    use super::*;
    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }
    pub fn increment_by(deps: DepsMut, val: i64) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += val;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }
    pub fn decrement(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count -= 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }
    pub fn decrement_by(deps: DepsMut, val: i64) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count -= val;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }
    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
