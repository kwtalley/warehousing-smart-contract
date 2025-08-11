use cosmwasm_std::Env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NftType {
    LENDER,
    ORIGINATOR,
}

pub fn get_nft_id(env: &Env, marker_address: &str, nft_type: &NftType) -> String {
    format!("{}.{}.{:?}", env.contract.address, marker_address, nft_type)
}