use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u8,
    pub result: RpcResult<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct RpcResult<T> {
    pub context: Context,
    pub value: T,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub slot: u64,
}
