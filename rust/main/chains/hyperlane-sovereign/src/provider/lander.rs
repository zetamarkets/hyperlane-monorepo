use hyperlane_core::{ChainResult, H256};
use serde_json::Value;

use crate::types::{SimulateResult, SubmitTxResponse, Tx};
use crate::SovereignProvider;

/// Trait used by lander for Sovereign chain interactions.
#[async_trait::async_trait]
pub trait SovereignProviderForLander: Send + Sync {
    /// Simulate a transaction with the given call message.
    async fn simulate(&self, call_message: &Value) -> ChainResult<SimulateResult>;

    /// Build and submit a transaction to the rollup.
    async fn build_and_submit(&self, call_message: Value) -> ChainResult<(SubmitTxResponse, String)>;

    /// Get a transaction by its hash.
    async fn get_tx_by_hash(&self, tx_hash: H256) -> ChainResult<Tx>;
}

#[async_trait::async_trait]
impl SovereignProviderForLander for SovereignProvider {
    async fn simulate(&self, call_message: &Value) -> ChainResult<SimulateResult> {
        self.client.simulate(call_message).await
    }

    async fn build_and_submit(&self, call_message: Value) -> ChainResult<(SubmitTxResponse, String)> {
        self.client.build_and_submit(call_message).await
    }

    async fn get_tx_by_hash(&self, tx_hash: H256) -> ChainResult<Tx> {
        self.client.get_tx_by_hash(tx_hash).await
    }
}
