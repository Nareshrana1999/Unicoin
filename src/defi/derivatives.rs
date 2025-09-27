//! Derivatives implementation for Unicoin DeFi

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derivative {
    pub id: Hash,
    pub name: String,
    pub underlying_asset: Hash,
    pub price: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsContract {
    pub id: Hash,
    pub strike_price: u64,
    pub expiration: u64,
    pub option_type: OptionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesContract {
    pub id: Hash,
    pub delivery_date: u64,
    pub contract_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerpetualSwap {
    pub id: Hash,
    pub funding_rate: i16,
    pub mark_price: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginTrading {
    pub id: Hash,
    pub trader: PublicKey,
    pub leverage: u8,
    pub margin: u64,
}
