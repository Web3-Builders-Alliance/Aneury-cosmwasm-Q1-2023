#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:rw-cosm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
      
        admin_address: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
     //unmpliemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    
    let version: Version = CONTRACT_VERSION.parse()?;
    
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

    if storage_version < version {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }

    /// OK()
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    unmpliemented!()
}

pub mod query {
    use super::*;

 
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {  };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

     // it worked, let's query the state
      ///  let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
      ///  let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }  







    #[test]
    pub fn test_query_balance(){

    } 

    #[test]
    pub fn test_query_report(){

    } 


    //CreateCampaign 
    #[test]
    pub fn test_create_valid_campaign(){}
    pub fn test_create_invalid_campaign(){}
    pub fn test_create_existing_campaign(){}
  
    //StopRewardCampaign 
    pub fn test_stop_valid_campaign(){}
    pub fn test_stop_invalid_campaign(){}
    pub fn test_stop_non_existing_campaign(){}
    
    //DelegateRewardCampaign 
    pub fn test_delegate_valid_campaign(){}
    pub fn test_delegate_invalid_campaign(){}
    pub fn test_delegate_non_existing_campaign(){}
   
   
    ///BurnRewardToken 
    /// ??? Todo y
    ///GiveReward 
    pub fn test_give_valid_reward(){}
    pub fn test_give_delegate_valid_reward(){}
    pub fn test_give_non_valid_reward_points(){}
   
   
    //TradeReward 


}
