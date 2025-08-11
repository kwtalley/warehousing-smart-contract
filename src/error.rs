use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("NFT class already in use")]
    NftClassInUse,

    #[error("Sent funds are not correct for the configured warehouse denom")]
    InvalidFunds,

    #[error("Contract does not have marker permissions")]
    InvalidMarkerPermissions,

    #[error("Marker is not holding all of its supply")]
    InvalidMarkerHolding,

    #[error("Pledge is not in the correct status")]
    InvalidPledgeStatus,

    #[error("Pledge not found")]
    PledgeNotFound,
}
