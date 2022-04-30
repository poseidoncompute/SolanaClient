use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;
use poseidon_common::JsonError;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcInvalidResponse {
    pub jsonrpc: String,
    pub error: JsonError,
    pub id: u8,
}

impl fmt::Display for RpcInvalidResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for RpcInvalidResponse {}

impl From<RpcInvalidResponse> for JsonError {
    fn from(rpc_error: RpcInvalidResponse) -> Self {
        rpc_error.error
    }
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename = "camelCase")]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u8,
    pub result: RpcResult<T>,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename = "camelCase")]
pub struct RpcResult<T> {
    pub context: Context,
    pub value: T,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub slot: u64,
}
