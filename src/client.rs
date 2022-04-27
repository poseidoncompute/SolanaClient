use crate::{OnChainTransaction, RecentBlockHash, RpcInvalidResponse, RpcResponse};
use poseidon_common::{Base58BlockHash, Cluster};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SolClient {
    cluster: Cluster,
    recent_blockhash: Option<Base58BlockHash>,
}

impl SolClient {
    pub fn new() -> Self {
        SolClient {
            cluster: Cluster::default(),
            recent_blockhash: Option::default(),
        }
    }

    pub fn switch_cluster(&mut self, cluster: Cluster) -> &mut Self {
        self.cluster = cluster;

        self
    }

    pub async fn add_blockhash(&mut self) -> anyhow::Result<&mut Self> {
        let body = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getRecentBlockhash"
        };
        let response = self.http_op(body).await?;
        let deser_response: RpcResponse<RecentBlockHash> =
            serde_json::from_str(&response.as_str()?)?;
        self.recent_blockhash = Some(deser_response.result.value.blockhash);

        Ok(self)
    }

    pub async fn get_transaction(&self, signature: &str) -> anyhow::Result<OnChainTransaction> {
        let body = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getTransaction",
            params: json::array![
                json::JsonValue::String(signature.to_owned()),
                json::JsonValue::String("json".to_owned()),
            ],
        };

        let response = self.http_op(body).await?;

        dbg!(&response.as_str()?);

        let deser_response: Result<OnChainTransaction, _> =
            serde_json::from_str(&response.as_str()?);

        match deser_response {
            Ok(value) => Ok(value),
            Err(tx_deser_error) => {
                let deser_error: Result<RpcInvalidResponse, _> =
                    serde_json::from_str(&response.as_str()?);

                match deser_error {
                    Ok(error_value) => Err(anyhow::Error::new(error_value)),
                    Err(_) => Err(anyhow::Error::new(tx_deser_error)),
                }
            }
        }
    }

    pub async fn http_op(&self, body: json::JsonValue) -> Result<minreq::Response, minreq::Error> {
        minreq::post(self.cluster.url())
            .with_header("Content-Type", "application/json")
            .with_body(body.to_string().as_str())
            .send()
    }
}
