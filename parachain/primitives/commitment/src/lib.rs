#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::{H160, H256, RuntimeDebug};
use sp_runtime::traits::Hash;
use sp_std::prelude::*;
use ethabi::{self, Token};

use artemis_core::ChannelId;
use codec::{Encode, Decode};

/// Wire-format for committed messages
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug)]
pub struct Message {
	/// Target application on the Ethereum side.
	pub target: H160,
	/// A nonce for replay protection and ordering.
	pub nonce: u64,
	/// Payload for target application.
	pub payload: Vec<u8>,
}

pub fn make_offchain_key(prefix: &[u8], channel_id: ChannelId, commitment: H256) -> Vec<u8> {
	(prefix, channel_id, commitment).encode()
}

pub fn make_commitment<Hashing>(block_number: u32, messages: &[Message]) -> (H256, usize)
    where Hashing: Hash<Output = H256>
{
    let mut payload_size = 0usize;
    let messages: Vec<Token> = messages
        .iter()
        .map(|message| {
            payload_size += message.payload.len();
            Token::Tuple(vec![
                Token::Address(message.target),
                Token::Uint(message.nonce.into()),
                Token::Bytes(message.payload.clone())
            ])
        })
        .collect();

    let input = ethabi::encode(&vec![
        Token::Uint(block_number.into()),
        Token::Array(messages)]
    );

    (<Hashing as Hash>::hash(&input), payload_size)
}
