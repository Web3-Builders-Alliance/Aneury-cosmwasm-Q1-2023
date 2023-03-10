
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Poll, POLLS, Ballot, BALLOT};

const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = _msg.admin.unwrap_or(_info.sender.to_string());

    let validated_admin = _deps.api.addr_validate(&admin)?;

    let config = Config{
      admin: validated_admin.clone(),
    };

    CONFIG.save(_deps.storage, &config)?;


    Ok(Response::new()
       .add_attribute("action", "instantiate")
       .add_attribute("admin", validated_admin.to_string())
      )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {

     match _msg {
         ExecuteMsg::CreatePoll {
             poll_id,
             question,
             options
         }=> execute_create_poll(_deps, _env, _info, poll_id, question, options),
         ExecuteMsg::Vote {
             poll_id,vote
         }=>execute_vote(_deps, _env, _info, poll_id, vote),

     }

}

fn execute_create_poll(
    deps: DepsMut,
    _env: Env, // _env as we won't be using it
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {

    if options.len() > 10{
        return Err(ContractError::TooManyOptions {})
    }
    let mut opts: Vec<(String, u64)> =vec![];
    for option in options{
        opts.push((option,0))
    }

    let poll = Poll{
        creator: info.sender,
        question,
        options: opts
    };

    POLLS.save(deps.storage, poll_id, &poll)?;

    Ok(Response::new()
        .add_attribute("action", "poll_creted_and_added"))
}




// Previous code omitted
fn execute_vote(
    deps: DepsMut,
    env: Env, // underscored as not needed
    info: MessageInfo,
    poll_id: String,
    vote: String,
)->Result<Response, ContractError>{

    let poll = POLLS.may_load(deps.storage, poll_id.clone());

    match poll {
       Some(mut poll)=> {

            BALLOT.update(deps.storage,
                          info.sender,
                          |ballot: Option<Ballot>|-> StdResult<Ballot>{
                              match ballot {
                                  Some(ballot) => {
                                      // We need to revoke their old vote
                                      // Find the position
                                      let position_of_old_vote = poll
                                          .options
                                          .iter()
                                          .position(|option| option.0 == ballot.option)
                                          .unwrap();
                                      // Decrement by 1
                                      poll.options[position_of_old_vote].1 -= 1;
                                      // Update the ballot
                                      Ok(Ballot { option: vote.clone() })
                                  }
                                  None => {
                                      // Simply add the ballot
                                      Ok(Ballot { option: vote.clone() })
                                  }
                              }
                          },
                          )?;


            let position = poll
                .options
                .iter()
                .position(|option| option.0 == vote);


            if position.is_none() {
                return Err(ContractError::PollNotFound {});
            }

            let position = position.unwrap();

            poll.options[position].1 += 1;


            // Save the update
            POLLS.save(deps.storage, poll_id, &poll)?;


            Ok(Response::new()
                .add_attribute("poll_done", format!("poll has been done and save with poll_id->{poll_id}")))
        },
        None =>  Err(ContractError::PollNotFound {})
    }

}

//
// fn execute_vote2(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     poll_id: String,
//     vote: String,
// ) -> Result<Response, ContractError> {
//     let poll = POLLS.may_load(deps.storage, poll_id.clone())?;
//
//     match poll {
//         Some(mut poll) => { // The poll exists
//             BALLOTS.update(
//                 deps.storage,
//                 (info.sender, poll_id.clone()),
//                 |ballot| -> StdResult<Ballot> {
//                     match ballot {
//                         Some(ballot) => {
//                             // We need to revoke their old vote
//                             // Find the position
//                             let position_of_old_vote = poll
//                                 .options
//                                 .iter()
//                                 .position(|option| option.0 == ballot.option)
//                                 .unwrap();
//                             // Decrement by 1
//                             poll.options[position_of_old_vote].1 -= 1;
//                             // Update the ballot
//                             Ok(Ballot { option: vote.clone() })
//                         }
//                         None => {
//                             // Simply add the ballot
//                             Ok(Ballot { option: vote.clone() })
//                         }
//                     }
//                 },
//             )?;
//
//             // Find the position of the new vote option and increment it by 1
//             let position = poll
//                 .options
//                 .iter()
//                 .position(|option| option.0 == vote);
//             if position.is_none() {
//                 return Err(ContractError::Unauthorized {});
//             }
//             let position = position.unwrap();
//             poll.options[position].1 += 1;
//         },
//         None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
//     }
// }


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::attr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::contract::instantiate;
    use crate::msg::InstantiateMsg;

    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate(){
        let mut deps = mock_dependencies();

        let env  = mock_env();

        let info = mock_info(ADDR1, &vec![]);


        let msg = InstantiateMsg{ admin: None };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }


    #[test]
    fn test_instantiate_with_admin(){
        let mut deps = mock_dependencies();

        let env  = mock_env();

        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg{ admin: Some(ADDR2.to_string()) };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();



        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2)]
        )


    }
}
