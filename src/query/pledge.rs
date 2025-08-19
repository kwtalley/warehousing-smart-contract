use cosmwasm_std::{Deps, Env};

use crate::{error::ContractError, state::{PLEDGES, Pledge}};

pub fn pledge(deps: Deps, _env: Env, pledge_id: String) -> Result<Pledge, ContractError> {
    let pledge = PLEDGES.load(deps.storage, pledge_id.clone())?;
    Ok(pledge)
}