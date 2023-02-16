#![allow(missing_docs)]
use super::NewBlock;
use crate::{SharedTransactions, Status};
use bytes::{Buf, BufMut};
use reth_rlp::{length_of_length, Decodable, Encodable, Header};
use std::{fmt::Debug, sync::Arc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An `eth` protocol message, containing a message ID and payload.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProtocolMessage<T: EthMessage> {
    pub message_type: EthMessageID,
    pub message: T,
}

impl<T: EthMessage> ProtocolMessage<T> {
    /// Create a new ProtocolMessage from a message type and message rlp bytes.
    pub fn decode(buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        let message_type = EthMessageID::decode(buf)?;
        let message = T::decode(message_type, buf)?;
        Ok(ProtocolMessage { message_type, message })
    }
}

/// Encodes the protocol message into bytes.
/// The message type is encoded as a single byte and prepended to the message.
impl<T: EthMessage> Encodable for ProtocolMessage<T> {
    fn encode(&self, out: &mut dyn BufMut) {
        self.message_type.encode(out);
        self.message.encode(out);
    }
    fn length(&self) -> usize {
        self.message_type.length() + self.message.length()
    }
}

impl<T: EthMessage> From<T> for ProtocolMessage<T> {
    fn from(message: T) -> Self {
        ProtocolMessage { message_type: message.message_id(), message }
    }
}

/// Represents messages that can be sent to multiple peers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolBroadcastMessage {
    pub message_type: EthMessageID,
    pub message: EthBroadcastMessage,
}

/// Encodes the protocol message into bytes.
/// The message type is encoded as a single byte and prepended to the message.
impl Encodable for ProtocolBroadcastMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        self.message_type.encode(out);
        self.message.encode(out);
    }
    fn length(&self) -> usize {
        self.message_type.length() + self.message.length()
    }
}

impl From<EthBroadcastMessage> for ProtocolBroadcastMessage {
    fn from(message: EthBroadcastMessage) -> Self {
        ProtocolBroadcastMessage { message_type: message.message_id(), message }
    }
}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub trait EthMessage: Debug + Encodable {
    fn message_id(&self) -> EthMessageID;
    fn decode(message_id: EthMessageID, buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError>
    where
        Self: Sized;
}

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
pub enum EthStatusMessage {
    /// Status is required for the protocol handshake
    Status(Status),
}

impl EthMessage for EthStatusMessage {
    /// Returns the message's ID.
    fn message_id(&self) -> EthMessageID {
        match self {
            EthStatusMessage::Status(_) => EthMessageID::Status,
        }
    }

    fn decode(message_id: EthMessageID, buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        Ok(match message_id {
            EthMessageID::Status => EthStatusMessage::Status(Status::decode(buf)?),
            _ => return Err(reth_rlp::DecodeError::Custom("invalid message id")),
        })
    }
}

impl Encodable for EthStatusMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            EthStatusMessage::Status(status) => status.encode(out),
        }
    }
    fn length(&self) -> usize {
        match self {
            EthStatusMessage::Status(status) => status.length(),
        }
    }
}

/// Represents broadcast messages of [`EthMessage`] with the same object that can be sent to
/// multiple peers.
///
/// Messages that contain a list of hashes depend on the peer the message is sent to. A peer should
/// never receive a hash of an object (block, transaction) it has already seen.
///
/// Note: This is only useful for outgoing messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EthBroadcastMessage {
    NewBlock(Arc<NewBlock>),
    Transactions(SharedTransactions),
}

// === impl EthBroadcastMessage ===

impl EthBroadcastMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> EthMessageID {
        match self {
            EthBroadcastMessage::NewBlock(_) => EthMessageID::NewBlock,
            EthBroadcastMessage::Transactions(_) => EthMessageID::Transactions,
        }
    }
}

impl Encodable for EthBroadcastMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            EthBroadcastMessage::NewBlock(new_block) => new_block.encode(out),
            EthBroadcastMessage::Transactions(transactions) => transactions.encode(out),
        }
    }

    fn length(&self) -> usize {
        match self {
            EthBroadcastMessage::NewBlock(new_block) => new_block.length(),
            EthBroadcastMessage::Transactions(transactions) => transactions.length(),
        }
    }
}

/// Represents message IDs for eth protocol messages.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EthMessageID {
    Status = 0x00,
    NewBlockHashes = 0x01,
    Transactions = 0x02,
    GetBlockHeaders = 0x03,
    BlockHeaders = 0x04,
    GetBlockBodies = 0x05,
    BlockBodies = 0x06,
    NewBlock = 0x07,
    NewPooledTransactionHashes = 0x08,
    GetPooledTransactions = 0x09,
    PooledTransactions = 0x0a,
    GetNodeData = 0x0d,
    NodeData = 0x0e,
    GetReceipts = 0x0f,
    Receipts = 0x10,
}

impl Encodable for EthMessageID {
    fn encode(&self, out: &mut dyn BufMut) {
        out.put_u8(*self as u8);
    }
    fn length(&self) -> usize {
        1
    }
}

impl Decodable for EthMessageID {
    fn decode(buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        let id = buf.first().ok_or(reth_rlp::DecodeError::InputTooShort)?;
        let id = match id {
            0x00 => EthMessageID::Status,
            0x01 => EthMessageID::NewBlockHashes,
            0x02 => EthMessageID::Transactions,
            0x03 => EthMessageID::GetBlockHeaders,
            0x04 => EthMessageID::BlockHeaders,
            0x05 => EthMessageID::GetBlockBodies,
            0x06 => EthMessageID::BlockBodies,
            0x07 => EthMessageID::NewBlock,
            0x08 => EthMessageID::NewPooledTransactionHashes,
            0x09 => EthMessageID::GetPooledTransactions,
            0x0a => EthMessageID::PooledTransactions,
            0x0d => EthMessageID::GetNodeData,
            0x0e => EthMessageID::NodeData,
            0x0f => EthMessageID::GetReceipts,
            0x10 => EthMessageID::Receipts,
            _ => return Err(reth_rlp::DecodeError::Custom("Invalid message ID")),
        };
        buf.advance(1);
        Ok(id)
    }
}

impl TryFrom<usize> for EthMessageID {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(EthMessageID::Status),
            0x01 => Ok(EthMessageID::NewBlockHashes),
            0x02 => Ok(EthMessageID::Transactions),
            0x03 => Ok(EthMessageID::GetBlockHeaders),
            0x04 => Ok(EthMessageID::BlockHeaders),
            0x05 => Ok(EthMessageID::GetBlockBodies),
            0x06 => Ok(EthMessageID::BlockBodies),
            0x07 => Ok(EthMessageID::NewBlock),
            0x08 => Ok(EthMessageID::NewPooledTransactionHashes),
            0x09 => Ok(EthMessageID::GetPooledTransactions),
            0x0a => Ok(EthMessageID::PooledTransactions),
            0x0d => Ok(EthMessageID::GetNodeData),
            0x0e => Ok(EthMessageID::NodeData),
            0x0f => Ok(EthMessageID::GetReceipts),
            0x10 => Ok(EthMessageID::Receipts),
            _ => Err("Invalid message ID"),
        }
    }
}

/// This is used for all request-response style `eth` protocol messages.
/// This can represent either a request or a response, since both include a message payload and
/// request id.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RequestPair<T> {
    /// id for the contained request or response message
    pub request_id: u64,

    /// the request or response message payload
    pub message: T,
}

/// Allows messages with request ids to be serialized into RLP bytes.
impl<T> Encodable for RequestPair<T>
where
    T: Encodable,
{
    fn encode(&self, out: &mut dyn reth_rlp::BufMut) {
        let header =
            Header { list: true, payload_length: self.request_id.length() + self.message.length() };

        header.encode(out);
        self.request_id.encode(out);
        self.message.encode(out);
    }

    fn length(&self) -> usize {
        let mut length = 0;
        length += self.request_id.length();
        length += self.message.length();
        length += length_of_length(length);
        length
    }
}

/// Allows messages with request ids to be deserialized into RLP bytes.
impl<T> Decodable for RequestPair<T>
where
    T: Decodable,
{
    fn decode(buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        let _header = Header::decode(buf)?;
        Ok(Self { request_id: u64::decode(buf)?, message: T::decode(buf)? })
    }
}

#[cfg(test)]
mod test {
    use crate::types::message::RequestPair;
    use hex_literal::hex;
    use reth_rlp::{Decodable, Encodable};

    fn encode<T: Encodable>(value: T) -> Vec<u8> {
        let mut buf = vec![];
        value.encode(&mut buf);
        buf
    }

    #[test]
    fn request_pair_encode() {
        let request_pair = RequestPair { request_id: 1337, message: vec![5u8] };

        // c5: start of list (c0) + len(full_list) (length is <55 bytes)
        // 82: 0x80 + len(1337)
        // 05 39: 1337 (request_id)
        // === full_list ===
        // c1: start of list (c0) + len(list) (length is <55 bytes)
        // 05: 5 (message)
        let expected = hex!("c5820539c105");
        let got = encode(request_pair);
        assert_eq!(expected[..], got, "expected: {expected:X?}, got: {got:X?}",);
    }

    #[test]
    fn request_pair_decode() {
        let raw_pair = &hex!("c5820539c105")[..];

        let expected = RequestPair { request_id: 1337, message: vec![5u8] };

        let got = RequestPair::<Vec<u8>>::decode(&mut &*raw_pair).unwrap();
        assert_eq!(expected.length(), raw_pair.len());
        assert_eq!(expected, got);
    }
}
