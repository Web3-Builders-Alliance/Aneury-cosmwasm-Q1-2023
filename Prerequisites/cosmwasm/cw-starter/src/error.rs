use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Too many poll options")]
    TooManyOptions{},

    #[error("this is not authorized")]
    Unauthorized{},

    #[error("this is poll_id can\'t be found in the list...")]
    PollNotFound{}
}
