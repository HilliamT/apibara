//! Abstraction over chain providers.
//!
//! Chain providers are used to fetch and subscribe to
//! blockchain data.
use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::chain::{
    filter::EventFilter,
    types::{BlockEvents, BlockHash, BlockHeader},
};

/// Provide information about blocks and events/logs on a chain.
/// Send and Sync are used to allow for concurrency,
/// refer to https://doc.rust-lang.org/nomicon/send-and-sync.html
#[async_trait]
pub trait ChainProvider: Send + Sync + 'static {
    /// Get the most recent (head) block.
    async fn get_head_block(&self) -> Result<BlockHeader>;

    /// Get a specific block by its hash.
    async fn get_block_by_hash(&self, hash: &BlockHash) -> Result<Option<BlockHeader>>;

    /// Subscribe to blocks via stream
    fn subscribe_blocks(&self) -> Result<Pin<Box<dyn Stream<Item = BlockHeader> + Send>>>;

    async fn get_events_in_range(
        &self,
        from_block: u64,
        to_block: u64,
        filters: &[EventFilter],
    ) -> Result<Vec<BlockEvents>>;
}
