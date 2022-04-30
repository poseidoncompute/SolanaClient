use crate::{MessageHeader, RpcCompiledInstruction};
use borsh::{BorshDeserialize, BorshSerialize};
use poseidon_common::{Base58BlockHash, Base58PublicKey, Base58Signature};
use serde::{Deserialize, Serialize};
#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub signatures: Vec<Base58Signature>,
    pub message: Message,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub header: MessageHeader,
    pub account_keys: Vec<Base58PublicKey>,
    pub recent_blockhash: Base58BlockHash,
    pub instructions: Vec<RpcCompiledInstruction>,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RecentBlockHash {
    pub blockhash: String,
    pub fee_calculator: FeeCalculator,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FeeCalculator {
    pub lamports_per_signature: u64,
}
