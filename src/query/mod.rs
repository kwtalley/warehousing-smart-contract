use cosmwasm_std::{Binary, Deps, Env};

use crate::{error::ContractError, msg::QueryMsg};

pub fn route(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    Ok(Binary::from("hello world".as_bytes()))
}
