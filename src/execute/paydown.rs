use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::cosmos::nft::v1beta1::NftQuerier;

use crate::{error::ContractError, state::{CONFIG, PledgeStatus, PLEDGES}, util, util::NftType};

pub fn paydown(deps: DepsMut, env: Env, info: MessageInfo, pledge_id: String) -> Result<Response, ContractError> {
    let pledge = PLEDGES.load(deps.storage, pledge_id.clone())?;
    let config = CONFIG.load(deps.storage)?;

    // check if sender owns the originator nft
    let originator_nft = util::get_nft_id(&env, &pledge.marker_address, &NftType::ORIGINATOR);
    let nft_querier = NftQuerier::new(&deps.querier);
    let nft_owner_response = nft_querier.owner(config.nft_class_id, originator_nft)?;
    if nft_owner_response.owner.ne(&info.sender.to_string()) {
        return Err(ContractError::Unauthorized);
    }

    // check if the pledge is in the correct status
    if pledge.status != PledgeStatus::Approved {
        return Err(ContractError::InvalidPledgeStatus);
    }

    // update the pledge status
    PLEDGES.update(deps.storage, pledge_id.clone(), 
    |pledge| {
        match pledge {
            Some(mut pledge) => {
                pledge.status = PledgeStatus::PaydownRequested;
                Ok(pledge)
            }
            None => Err(ContractError::PledgeNotFound),
        }
    })?;

    Ok(Response::new()
    .add_attribute("method", "paydown")
    .add_attribute("id", pledge_id))
}