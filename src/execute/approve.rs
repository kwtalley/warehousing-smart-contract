use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::{asset::v1::{Asset, MsgCreateAsset}};
use provwasm_std::types::cosmos::nft::v1beta1::MsgSend;

use crate::{error::ContractError, state::{CONFIG, PledgeStatus, PLEDGES}, util, util::NftType};

pub fn approve(deps: DepsMut, env: Env, info: MessageInfo, pledge_id: String) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let pledge = PLEDGES.load(deps.storage, pledge_id.clone())?;

    // check if the pledge is in the correct status
    if pledge.status != PledgeStatus::Pledged {
        return Err(ContractError::InvalidPledgeStatus);
    }

    // check the funds sent match the pledge amount
    if info.funds.len() != 1 || info.funds[0] != pledge.amount {
        return Err(ContractError::InvalidFunds);
    }

    // update the pledge status
    PLEDGES.update(deps.storage, pledge_id.clone(), 
    |pledge| {
        match pledge {
            Some(mut pledge) => {
                pledge.status = PledgeStatus::Approved;
                Ok(pledge)
            }
            None => Err(ContractError::PledgeNotFound),
        }
    })?;

    // mint the lender nft to the sender
    let msg_mint_nft = MsgCreateAsset {
        asset: Some(Asset {
            id: util::get_nft_id(&env, &pledge.marker_address, &NftType::LENDER),
            class_id: config.nft_class_id.clone(),
            uri: "".to_string(),
            uri_hash: "".to_string(),
            data: "".to_string(),
        }),
        from_address: env.contract.address.to_string(),
    };

    // transfer the nft to the sender
    let msg_transfer_nft = MsgSend {
        class_id: config.nft_class_id.clone(),
        id: util::get_nft_id(&env, &pledge.marker_address, &NftType::LENDER),
        sender: env.contract.address.to_string(),
        receiver: info.sender.to_string(),
    };

    Ok(Response::new()
    .add_message(msg_mint_nft)
    .add_message(msg_transfer_nft)
    .add_attribute("method", "approve")
    .add_attribute("id", pledge_id))
}