use cosmwasm_std::{to_json_binary, Binary, Deps, Env};

use crate::{error::ContractError, msg::QueryMsg};

mod pledge;

pub fn route(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Pledge { id } => {
            let pledge = pledge::pledge(deps, env, id)?;
            Ok(to_json_binary(&pledge)?)
        }
    }
}
