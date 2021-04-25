//! # Core
//!
//! Common traits and types

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{Parameter, DispatchResult};
use sp_core::U256;
use sp_runtime::{TokenError, traits::Member};

pub mod imbalance;
pub mod consequence;

pub use consequence::{DepositConsequence, WithdrawConsequence};

pub type AssetId: Member + Parameter + Default + Copy;

pub trait Inspect<AccountId> {
	type AssetId: AssetId;

	fn exists(asset: Self::AssetId) -> bool;

	fn total_issuance(asset: Self::AssetId) -> U256;

	fn balance(asset: Self::AssetId, who: &AccountId) -> U256;

	fn can_deposit(
        asset: Self::AssetId,
        who: &AccountId,
        amount: U256,
    ) -> DepositConsequence;

	fn can_withdraw(
		asset: Self::AssetId,
		who: &AccountId,
		amount: U256,
	) -> WithdrawConsequence;
}

pub trait Create<AccountId>: Inspect<AccountId> {
	fn create(asset: Self::AssetId) -> DispatchResult;
}

pub trait Mutate<AccountId>: Inspect<AccountId> {
	fn mint(asset: Self::AssetId, who: &AccountId, amount: U256) -> DispatchResult;

	fn burn(asset: Self::AssetId, who: &AccountId, amount: U256) -> DispatchResult;

    fn transfer(
		asset: Self::AssetId,
		source: &AccountId,
		dest: &AccountId,
		amount: U256
	) -> DispatchResult;

    fn teleport(
		asset: Self::AssetId,
		source: &AccountId,
		dest: &AccountId,
		amount: U256
	) -> DispatchResult;
}

