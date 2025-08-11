use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use cw2::ContractVersion;
use provwasm_std::types::provenance::asset::v1::{AssetClass, MsgCreateAssetClass};
use provwasm_std::types::cosmos::nft::v1beta1::NftQuerier;

use crate::{
    error::ContractError,
    msg::InstantiateMsg,
    state::{ContractConfiguration, CONFIG, CONTRACT_INFO, CONTRACT_NAME, CONTRACT_VERSION},
};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Check if nft class exists
    let nft_querier = NftQuerier::new(&deps.querier);
    match nft_querier.class(msg.nft_class_id.clone()) {
        Ok(class) => {
            if class.class.is_some() {
                return Err(ContractError::NftClassInUse);
            }
        }
        Err(err) => {
            match err {
                // This is the error we expect when the class does not exist
                StdError::GenericErr { msg, backtrace: _ } => {
                    if msg.contains("codespace: nft, code: 4") {
                        // Class not found is expected - we can proceed
                    } else {
                        // This is a different error, not just "class not found"
                        return Err(ContractError::Std(StdError::generic_err(format!(
                            "NFT querier error: {}",
                            msg
                        ))));
                    }
                }
                _ => {
                    return Err(ContractError::Std(StdError::generic_err(format!(
                        "NFT querier error: {}",
                        err
                    ))));
                }
            }
        }
    }

    // save contract info
    let contract_info = ContractVersion {
        contract: CONTRACT_NAME.to_string(),
        version: CONTRACT_VERSION.to_string(),
    };
    CONTRACT_INFO.save(deps.storage, &contract_info)?;

    // TODO: create the nft class
    let nft_class_id = msg.nft_class_id.clone();

    // save config
    let config = ContractConfiguration {
        denom: msg.denom,
        nft_class_id: nft_class_id.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    // create the asset/nft class
    let msg_create_asset_class = MsgCreateAssetClass {
        asset_class: Some(AssetClass {
            id: nft_class_id.clone(),
            name: "WSC".to_string(),
            symbol: "WSC".to_string(),
            description: "WSC".to_string(),
            uri: "".to_string(),
            uri_hash: "".to_string(),
            data: "".to_string(),
        }),
        from_address: env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_message(msg_create_asset_class)
        .add_attribute("method", "instantiate")
        .add_attribute("denom", config.denom.clone())
        .add_attribute("nft_class_id", nft_class_id))
}
