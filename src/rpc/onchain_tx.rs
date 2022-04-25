use poseidon_common::Base58PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnChainTransaction {
    jsonrpc: String,
    id: u8,
    result: RpcResult,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResult {
    pub block_time: u64,
    pub meta: RpcMeta,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMeta {
    pub err: Option<RpcErr>,
    pub fee: u32,
    pub inner_instructions: Vec<RpcInnerInstructions>,
    pub log_messages: Vec<String>,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub pre_token_balances: Vec<TokenBalances>,
    pub post_token_balances: Vec<TokenBalances>,
    pub rewards: Vec<Reward>,
    //TODO apply a function to convert from JsonValue if an error is encountered
    pub status: Result<(), RpcErr>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcErr {
    #[serde(rename = "InstructionError")]
    instruction_error: Vec<serde_json::Value>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcInnerInstructions {
    pub index: u8,
    pub instructions: Vec<RpcCompiledInstruction>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalances {
    pub account_index: u8,
    pub mint: Base58PublicKey,
    pub owner: Base58PublicKey,
    pub ui_token_amount: TokenAmount,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: f64,
    pub ui_amount_string: String,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: RewardType,
    pub commission: u8,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct RpcCompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String,
}
