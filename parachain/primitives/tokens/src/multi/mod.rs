use frame_support::dispatch::{DispatchResult, DispatchError};

use sp_core::U256;
use codec::FullCodec;

use crate::consequence::{DepositConsequence, WithdrawConsequence};

mod imbalance;
mod balanced;


pub use imbalance::{Imbalance, HandleImbalanceDrop, CreditOf, DebtOf};
pub use balanced::{Balanced, Unbalanced};

/// Simple amalgamation trait to collect together properties for an AssetId under one roof.
pub trait AssetId: FullCodec + Copy + Default + Eq + PartialEq {}
impl<T: FullCodec + Copy + Default + Eq + PartialEq> AssetId for T {}

pub trait Inspect<AccountId> {
	type AssetId: AssetId;

	fn exists(id: Self::AssetId) -> bool;

	fn total_issuance(id: Self::AssetId) -> U256;

	fn balance(id: Self::AssetId, who: &AccountId) -> U256;

	fn can_deposit(
        id: Self::AssetId,
        who: &AccountId,
        amount: U256,
    ) -> DepositConsequence;

	fn can_withdraw(
		id: Self::AssetId,
		who: &AccountId,
		amount: U256,
	) -> WithdrawConsequence;
}

pub trait Mutate<AccountId>: Inspect<AccountId> {
	fn mint(id: Self::AssetId, who: &AccountId, amount: U256) -> DispatchResult;

	fn burn(id: Self::AssetId, who: &AccountId, amount: U256) -> DispatchResult;

    fn transfer(
		id: Self::AssetId,
		source: &AccountId,
		dest: &AccountId,
		amount: U256
	) -> DispatchResult;
}

pub trait Create<AccountId>: Inspect<AccountId> {
	fn create(id: Self::AssetId) -> DispatchResult;
}
