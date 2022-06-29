use serde::{Deserialize, Serialize};

use crate::miner::Miner;

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletDAO {
    pub address: String,
    pub club_name: String,
}
