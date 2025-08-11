use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, msg::ExecuteMsg};

mod approve_paydown;
mod approve;
mod paydown;
mod pledge;

pub fn route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Pledge {id, amount, marker_addr} => {
            pledge::pledge(deps, env, info, id, amount, marker_addr)
        }
        ExecuteMsg::ApprovePledge { pledge_id } => {
            approve::approve(deps, env, info, pledge_id)
        }
        ExecuteMsg::Paydown { pledge_id } => {
            paydown::paydown(deps, env, info, pledge_id)
        }
        ExecuteMsg::ApprovePaydown { pledge_id } => {
            approve_paydown::approve_paydown(deps, env, info, pledge_id)
        }
    }
}
