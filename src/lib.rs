use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 25,
        agency_name: "World Trade Organization (WTO)".to_string(),
        project_name: "WTO TradeRule AI".to_string(),
        portfolio: "Multilateral Trade Agreements & Rules".to_string(),
        strategy: "Tokenless Cryptographic Audit Anchor -- Multilateral Trade Dispute Evidence and Tariff Line Hash Depository".to_string(),
        target_chain: "Internet Computer (ICP) Native".to_string(),
        is_active: true,
    }
}
