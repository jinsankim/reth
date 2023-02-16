#![allow(missing_docs)]
use super::{
    broadcast::NewBlockHashes, BlockBodies, BlockHeaders, GetBlockBodies, GetBlockHeaders,
    GetNodeData, GetPooledTransactions, GetReceipts, NewBlock, NewPooledTransactionHashes,
    NodeData, PooledTransactions, Receipts, Transactions,
};
use crate::{
    message::{EthMessage, RequestPair},
    EthMessageID,
};
use bytes::BufMut;
use reth_rlp::{Decodable, Encodable};
use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a message in the eth wire protocol, versions 66 and 67.
///
/// The ethereum wire protocol is a set of messages that are broadcasted to the network in two
/// styles:
///  * A request message sent by a peer (such as [`GetPooledTransactions`]), and an associated
///  response message (such as [`PooledTransactions`]).
///  * A message that is broadcast to the network, without a corresponding request.
///
///  The newer `eth/66` is an efficiency upgrade on top of `eth/65`, introducing a request id to
///  correlate request-response message pairs. This allows for request multiplexing.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Eth66Message {
    /// The following messages are broadcast to the network
    NewBlockHashes(NewBlockHashes),
    NewBlock(Box<NewBlock>),
    Transactions(Transactions),
    NewPooledTransactionHashes(NewPooledTransactionHashes),

    // The following messages are request-response message pairs
    GetBlockHeaders(RequestPair<GetBlockHeaders>),
    BlockHeaders(RequestPair<BlockHeaders>),
    GetBlockBodies(RequestPair<GetBlockBodies>),
    BlockBodies(RequestPair<BlockBodies>),
    GetPooledTransactions(RequestPair<GetPooledTransactions>),
    PooledTransactions(RequestPair<PooledTransactions>),
    GetNodeData(RequestPair<GetNodeData>),
    NodeData(RequestPair<NodeData>),
    GetReceipts(RequestPair<GetReceipts>),
    Receipts(RequestPair<Receipts>),
}

impl EthMessage for Eth66Message {
    /// Returns the message's ID.
    fn message_id(&self) -> EthMessageID {
        match self {
            Eth66Message::NewBlockHashes(_) => EthMessageID::NewBlockHashes,
            Eth66Message::NewBlock(_) => EthMessageID::NewBlock,
            Eth66Message::Transactions(_) => EthMessageID::Transactions,
            Eth66Message::NewPooledTransactionHashes(_) => EthMessageID::NewPooledTransactionHashes,
            Eth66Message::GetBlockHeaders(_) => EthMessageID::GetBlockHeaders,
            Eth66Message::BlockHeaders(_) => EthMessageID::BlockHeaders,
            Eth66Message::GetBlockBodies(_) => EthMessageID::GetBlockBodies,
            Eth66Message::BlockBodies(_) => EthMessageID::BlockBodies,
            Eth66Message::GetPooledTransactions(_) => EthMessageID::GetPooledTransactions,
            Eth66Message::PooledTransactions(_) => EthMessageID::PooledTransactions,
            Eth66Message::GetNodeData(_) => EthMessageID::GetNodeData,
            Eth66Message::NodeData(_) => EthMessageID::NodeData,
            Eth66Message::GetReceipts(_) => EthMessageID::GetReceipts,
            Eth66Message::Receipts(_) => EthMessageID::Receipts,
        }
    }

    fn decode(message_id: EthMessageID, buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        Ok(match message_id {
            EthMessageID::NewBlockHashes => {
                Eth66Message::NewBlockHashes(NewBlockHashes::decode(buf)?)
            }
            EthMessageID::NewBlock => Eth66Message::NewBlock(Box::new(NewBlock::decode(buf)?)),
            EthMessageID::Transactions => Eth66Message::Transactions(Transactions::decode(buf)?),
            EthMessageID::NewPooledTransactionHashes => {
                Eth66Message::NewPooledTransactionHashes(NewPooledTransactionHashes::decode(buf)?)
            }
            EthMessageID::GetBlockHeaders => {
                let request_pair = RequestPair::<GetBlockHeaders>::decode(buf)?;
                Eth66Message::GetBlockHeaders(request_pair)
            }
            EthMessageID::BlockHeaders => {
                let request_pair = RequestPair::<BlockHeaders>::decode(buf)?;
                Eth66Message::BlockHeaders(request_pair)
            }
            EthMessageID::GetBlockBodies => {
                let request_pair = RequestPair::<GetBlockBodies>::decode(buf)?;
                Eth66Message::GetBlockBodies(request_pair)
            }
            EthMessageID::BlockBodies => {
                let request_pair = RequestPair::<BlockBodies>::decode(buf)?;
                Eth66Message::BlockBodies(request_pair)
            }
            EthMessageID::GetPooledTransactions => {
                let request_pair = RequestPair::<GetPooledTransactions>::decode(buf)?;
                Eth66Message::GetPooledTransactions(request_pair)
            }
            EthMessageID::PooledTransactions => {
                let request_pair = RequestPair::<PooledTransactions>::decode(buf)?;
                Eth66Message::PooledTransactions(request_pair)
            }
            EthMessageID::GetNodeData => {
                let request_pair = RequestPair::<GetNodeData>::decode(buf)?;
                Eth66Message::GetNodeData(request_pair)
            }
            EthMessageID::NodeData => {
                let request_pair = RequestPair::<NodeData>::decode(buf)?;
                Eth66Message::NodeData(request_pair)
            }
            EthMessageID::GetReceipts => {
                let request_pair = RequestPair::<GetReceipts>::decode(buf)?;
                Eth66Message::GetReceipts(request_pair)
            }
            EthMessageID::Receipts => {
                let request_pair = RequestPair::<Receipts>::decode(buf)?;
                Eth66Message::Receipts(request_pair)
            }
            _ => return Err(reth_rlp::DecodeError::Custom("invalid message id")),
        })
    }
}

impl Encodable for Eth66Message {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            Eth66Message::NewBlockHashes(new_block_hashes) => new_block_hashes.encode(out),
            Eth66Message::NewBlock(new_block) => new_block.encode(out),
            Eth66Message::Transactions(transactions) => transactions.encode(out),
            Eth66Message::NewPooledTransactionHashes(hashes) => hashes.encode(out),
            Eth66Message::GetBlockHeaders(request) => request.encode(out),
            Eth66Message::BlockHeaders(headers) => headers.encode(out),
            Eth66Message::GetBlockBodies(request) => request.encode(out),
            Eth66Message::BlockBodies(bodies) => bodies.encode(out),
            Eth66Message::GetPooledTransactions(request) => request.encode(out),
            Eth66Message::PooledTransactions(transactions) => transactions.encode(out),
            Eth66Message::GetNodeData(request) => request.encode(out),
            Eth66Message::NodeData(data) => data.encode(out),
            Eth66Message::GetReceipts(request) => request.encode(out),
            Eth66Message::Receipts(receipts) => receipts.encode(out),
        }
    }
    fn length(&self) -> usize {
        match self {
            Eth66Message::NewBlockHashes(new_block_hashes) => new_block_hashes.length(),
            Eth66Message::NewBlock(new_block) => new_block.length(),
            Eth66Message::Transactions(transactions) => transactions.length(),
            Eth66Message::NewPooledTransactionHashes(hashes) => hashes.length(),
            Eth66Message::GetBlockHeaders(request) => request.length(),
            Eth66Message::BlockHeaders(headers) => headers.length(),
            Eth66Message::GetBlockBodies(request) => request.length(),
            Eth66Message::BlockBodies(bodies) => bodies.length(),
            Eth66Message::GetPooledTransactions(request) => request.length(),
            Eth66Message::PooledTransactions(transactions) => transactions.length(),
            Eth66Message::GetNodeData(request) => request.length(),
            Eth66Message::NodeData(data) => data.length(),
            Eth66Message::GetReceipts(request) => request.length(),
            Eth66Message::Receipts(receipts) => receipts.length(),
        }
    }
}
