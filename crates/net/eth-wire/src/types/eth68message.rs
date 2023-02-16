#![allow(missing_docs)]
use super::{
    broadcast::NewBlockHashes, BlockBodies, BlockHeaders, GetBlockBodies, GetBlockHeaders,
    GetPooledTransactions, GetReceipts, NewBlock, NewPooledTransactionHashes, PooledTransactions,
    Receipts, Transactions,
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
pub enum Eth68Message {
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
    GetReceipts(RequestPair<GetReceipts>),
    Receipts(RequestPair<Receipts>),
}

impl EthMessage for Eth68Message {
    /// Returns the message's ID.
    fn message_id(&self) -> EthMessageID {
        match self {
            Eth68Message::NewBlockHashes(_) => EthMessageID::NewBlockHashes,
            Eth68Message::NewBlock(_) => EthMessageID::NewBlock,
            Eth68Message::Transactions(_) => EthMessageID::Transactions,
            Eth68Message::NewPooledTransactionHashes(_) => EthMessageID::NewPooledTransactionHashes,
            Eth68Message::GetBlockHeaders(_) => EthMessageID::GetBlockHeaders,
            Eth68Message::BlockHeaders(_) => EthMessageID::BlockHeaders,
            Eth68Message::GetBlockBodies(_) => EthMessageID::GetBlockBodies,
            Eth68Message::BlockBodies(_) => EthMessageID::BlockBodies,
            Eth68Message::GetPooledTransactions(_) => EthMessageID::GetPooledTransactions,
            Eth68Message::PooledTransactions(_) => EthMessageID::PooledTransactions,
            Eth68Message::GetReceipts(_) => EthMessageID::GetReceipts,
            Eth68Message::Receipts(_) => EthMessageID::Receipts,
        }
    }

    fn decode(message_id: EthMessageID, buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        Ok(match message_id {
            EthMessageID::NewBlockHashes => {
                Eth68Message::NewBlockHashes(NewBlockHashes::decode(buf)?)
            }
            EthMessageID::NewBlock => Eth68Message::NewBlock(Box::new(NewBlock::decode(buf)?)),
            EthMessageID::Transactions => Eth68Message::Transactions(Transactions::decode(buf)?),
            EthMessageID::NewPooledTransactionHashes => {
                Eth68Message::NewPooledTransactionHashes(NewPooledTransactionHashes::decode(buf)?)
            }
            EthMessageID::GetBlockHeaders => {
                let request_pair = RequestPair::<GetBlockHeaders>::decode(buf)?;
                Eth68Message::GetBlockHeaders(request_pair)
            }
            EthMessageID::BlockHeaders => {
                let request_pair = RequestPair::<BlockHeaders>::decode(buf)?;
                Eth68Message::BlockHeaders(request_pair)
            }
            EthMessageID::GetBlockBodies => {
                let request_pair = RequestPair::<GetBlockBodies>::decode(buf)?;
                Eth68Message::GetBlockBodies(request_pair)
            }
            EthMessageID::BlockBodies => {
                let request_pair = RequestPair::<BlockBodies>::decode(buf)?;
                Eth68Message::BlockBodies(request_pair)
            }
            EthMessageID::GetPooledTransactions => {
                let request_pair = RequestPair::<GetPooledTransactions>::decode(buf)?;
                Eth68Message::GetPooledTransactions(request_pair)
            }
            EthMessageID::PooledTransactions => {
                let request_pair = RequestPair::<PooledTransactions>::decode(buf)?;
                Eth68Message::PooledTransactions(request_pair)
            }
            EthMessageID::GetReceipts => {
                let request_pair = RequestPair::<GetReceipts>::decode(buf)?;
                Eth68Message::GetReceipts(request_pair)
            }
            EthMessageID::Receipts => {
                let request_pair = RequestPair::<Receipts>::decode(buf)?;
                Eth68Message::Receipts(request_pair)
            }
            _ => return Err(reth_rlp::DecodeError::Custom("invalid message id")),
        })
    }
}

impl Encodable for Eth68Message {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            Eth68Message::NewBlockHashes(new_block_hashes) => new_block_hashes.encode(out),
            Eth68Message::NewBlock(new_block) => new_block.encode(out),
            Eth68Message::Transactions(transactions) => transactions.encode(out),
            Eth68Message::NewPooledTransactionHashes(hashes) => hashes.encode(out),
            Eth68Message::GetBlockHeaders(request) => request.encode(out),
            Eth68Message::BlockHeaders(headers) => headers.encode(out),
            Eth68Message::GetBlockBodies(request) => request.encode(out),
            Eth68Message::BlockBodies(bodies) => bodies.encode(out),
            Eth68Message::GetPooledTransactions(request) => request.encode(out),
            Eth68Message::PooledTransactions(transactions) => transactions.encode(out),
            Eth68Message::GetReceipts(request) => request.encode(out),
            Eth68Message::Receipts(receipts) => receipts.encode(out),
        }
    }
    fn length(&self) -> usize {
        match self {
            Eth68Message::NewBlockHashes(new_block_hashes) => new_block_hashes.length(),
            Eth68Message::NewBlock(new_block) => new_block.length(),
            Eth68Message::Transactions(transactions) => transactions.length(),
            Eth68Message::NewPooledTransactionHashes(hashes) => hashes.length(),
            Eth68Message::GetBlockHeaders(request) => request.length(),
            Eth68Message::BlockHeaders(headers) => headers.length(),
            Eth68Message::GetBlockBodies(request) => request.length(),
            Eth68Message::BlockBodies(bodies) => bodies.length(),
            Eth68Message::GetPooledTransactions(request) => request.length(),
            Eth68Message::PooledTransactions(transactions) => transactions.length(),
            Eth68Message::GetReceipts(request) => request.length(),
            Eth68Message::Receipts(receipts) => receipts.length(),
        }
    }
}
