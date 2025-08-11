use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::{asset::v1::{Asset, MsgCreateAsset}, marker::v1::{Access, MarkerQuerier}};
use provwasm_std::types::cosmos::nft::v1beta1::MsgSend;

use crate::{error::ContractError, state::{CONFIG, Pledge, PledgeStatus, PLEDGES}, util, util::NftType};

pub fn pledge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    amount: Coin,
    marker_addr: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // check if the denom is correct
    if info.funds.len() != 1 || info.funds[0].denom != config.denom {
        return Err(ContractError::InvalidFunds);
    }
    
    let marker_querier = MarkerQuerier::new(&deps.querier);
    
    // check if the marker permission is correct
    let marker_access_grant = {
        let response = marker_querier.access(marker_addr.clone())?;
        response.accounts.iter().find(|account| account.address.eq(env.contract.address.as_str())).cloned()
    };
    match marker_access_grant {
        Some(marker_access_grant) => {
            // verify Access::Admin
            if marker_access_grant.permissions.contains(&(Access::Admin as i32)) {
                return Err(ContractError::InvalidMarkerPermissions);
            }
        }
        None => {
            return Err(ContractError::InvalidMarkerPermissions);
        }
    }

    // check if the marker is holding all the denom
    let holding_accts = marker_querier.holding(marker_addr.clone(), None)?.balances;
    if holding_accts.len() != 1 || holding_accts[0].address != marker_addr {
        return Err(ContractError::InvalidMarkerHolding);
    }
    
    // store the pledge in state
    let pledge = Pledge {
        amount: amount.clone(),
        lender_nft_id: None,
        marker_address: marker_addr.clone(),
        originator_nft_id: util::get_nft_id(&env, &marker_addr, &NftType::ORIGINATOR),
        status: PledgeStatus::Pledged,
    };
    PLEDGES.save(deps.storage, id.clone(), &pledge)?;

    // mint the originator nft
    let msg_mint_nft = MsgCreateAsset {
        asset: Some(Asset {
            class_id: config.nft_class_id.clone(),
            id: util::get_nft_id(&env, &marker_addr, &NftType::ORIGINATOR),
            uri: "".to_string(),
            uri_hash: "".to_string(),
            data: "".to_string(),
        }),
        from_address: env.contract.address.to_string(),
    };

    // transfer the nft to the sender
    let msg_transfer_nft = MsgSend {
        class_id: config.nft_class_id.clone(),
        id: util::get_nft_id(&env, &marker_addr, &NftType::ORIGINATOR),
        sender: env.contract.address.to_string(),
        receiver: info.sender.to_string(),
    };

    Ok(Response::new()
    .add_message(msg_mint_nft)
    .add_message(msg_transfer_nft)
    .add_attribute("method", "pledge")
    .add_attribute("id", id)
    .add_attribute("amount", amount.to_string()))
}
