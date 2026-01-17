use std::sync::Arc;

use hyperlane_core::{ChainCommunicationError, H256, H512};
use hyperlane_sovereign::{Receipt, Tx, TxData, TxResult};

use crate::adapter::AdaptsChain;
use crate::transaction::TransactionStatus;
use crate::{LanderError, TransactionDropReason};

use super::tests_common::{adapter, MockSovereignProvider};

fn make_tx(result: TxResult) -> Tx {
    Tx {
        number: 1,
        hash: H256::zero(),
        events: vec![],
        batch_number: 1,
        receipt: Receipt {
            result,
            data: TxData { gas_used: vec![100] },
        },
    }
}

fn h256_to_h512(h: H256) -> H512 {
    let mut bytes = [0u8; 64];
    bytes[32..].copy_from_slice(h.as_bytes());
    H512::from_slice(&bytes)
}

#[tokio::test]
async fn get_tx_hash_status_successful() {
    let mut provider = MockSovereignProvider::new();

    provider
        .expect_get_tx_by_hash()
        .returning(|_| Ok(make_tx(TxResult::Successful)));

    let provider_arc = Arc::new(provider);
    let adapter = adapter(provider_arc);

    let hash = h256_to_h512(H256::zero());
    let tx_status = adapter
        .get_tx_hash_status(hash)
        .await
        .expect("Failed to get tx hash status");

    assert_eq!(tx_status, TransactionStatus::Finalized);
}

#[tokio::test]
async fn get_tx_hash_status_reverted() {
    let mut provider = MockSovereignProvider::new();

    provider
        .expect_get_tx_by_hash()
        .returning(|_| Ok(make_tx(TxResult::Reverted)));

    let provider_arc = Arc::new(provider);
    let adapter = adapter(provider_arc);

    let hash = h256_to_h512(H256::zero());
    let tx_status = adapter
        .get_tx_hash_status(hash)
        .await
        .expect("Failed to get tx hash status");

    assert_eq!(
        tx_status,
        TransactionStatus::Dropped(TransactionDropReason::DroppedByChain)
    );
}

#[tokio::test]
async fn get_tx_hash_status_skipped() {
    let mut provider = MockSovereignProvider::new();

    provider
        .expect_get_tx_by_hash()
        .returning(|_| Ok(make_tx(TxResult::Skipped)));

    let provider_arc = Arc::new(provider);
    let adapter = adapter(provider_arc);

    let hash = h256_to_h512(H256::zero());
    let tx_status = adapter
        .get_tx_hash_status(hash)
        .await
        .expect("Failed to get tx hash status");

    assert_eq!(
        tx_status,
        TransactionStatus::Dropped(TransactionDropReason::DroppedByChain)
    );
}

#[tokio::test]
async fn get_tx_hash_status_not_found() {
    let mut provider = MockSovereignProvider::new();

    provider.expect_get_tx_by_hash().returning(|_| {
        Err(ChainCommunicationError::CustomError(
            "Transaction not found".to_string(),
        ))
    });

    let provider_arc = Arc::new(provider);
    let adapter = adapter(provider_arc);

    let hash = h256_to_h512(H256::zero());
    let tx_status = adapter.get_tx_hash_status(hash).await;

    match tx_status {
        Err(LanderError::TxHashNotFound(_)) => {}
        val => panic!("Expected TxHashNotFound, got {val:?}"),
    }
}

#[tokio::test]
async fn get_tx_hash_status_error() {
    let mut provider = MockSovereignProvider::new();

    provider.expect_get_tx_by_hash().returning(|_| {
        Err(ChainCommunicationError::CustomError(
            "Connection failed".to_string(),
        ))
    });

    let provider_arc = Arc::new(provider);
    let adapter = adapter(provider_arc);

    let hash = h256_to_h512(H256::zero());
    let tx_status = adapter.get_tx_hash_status(hash).await;

    match tx_status {
        Err(LanderError::ChainCommunicationError(_)) => {}
        val => panic!("Expected ChainCommunicationError, got {val:?}"),
    }
}
