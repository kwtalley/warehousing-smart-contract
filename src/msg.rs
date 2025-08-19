use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct NftInfo {
    pub token_id: String,
    pub class_id: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub nft_class_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Pledge {
        amount: Coin,
        id: String,
        marker_addr: String,
    },
    ApprovePledge {
        pledge_id: String,
    },
    Paydown {
        pledge_id: String,
    },
    ApprovePaydown {
        pledge_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
