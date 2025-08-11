use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cw2::ContractVersion;
use cw_storage_plus::{Item, Map};

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:warehousing-smart-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub enum PledgeStatus {
    Pledged,
    Approved,
    PaydownRequested,
    PaydownApproved,
}

#[cw_serde]
pub struct ContractConfiguration {
    pub denom: String,
    pub nft_class_id: String,
}

#[cw_serde]
pub struct Pledge {
    pub amount: Coin,
    pub marker_address: String,
    pub originator_nft_id: String,
    pub lender_nft_id: Option<String>,
    pub status: PledgeStatus,
}

// state items
// Contract Info uses the key "contract_info" per the cw2 spec
pub const CONTRACT_INFO: Item<ContractVersion> = Item::new("contract_info");
pub const CONFIG: Item<ContractConfiguration> = Item::new("contract_config");
pub const PLEDGES: Map<String, Pledge> = Map::new("pledges");
