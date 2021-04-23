use codec::{Encode, Decode};
use sp_core::{RuntimeDebug, H160};

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, PartialOrd, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AssetId {
	Ether,
	Token(H160)
}
