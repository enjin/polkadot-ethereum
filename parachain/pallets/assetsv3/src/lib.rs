#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use sp_std::prelude::*;
use frame_support::{
	traits::tokens::{WithdrawConsequence, DepositConsequence},
};
use codec::HasCompact;
use sp_runtime::traits::StaticLookup;
use sp_core::U256;

pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use super::*;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct AssetDetails {
		/// The total supply across all accounts.
		pub(super) supply: U256,
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct AssetBalance {
		pub(super) balance: U256
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct AssetMetadata {
		/// The user friendly name of this asset.
		pub(super) name: Vec<u8>,
		/// The ticker symbol for this asset.
		pub(super) symbol: Vec<u8>,
		/// The number of decimals this asset uses to represent one unit.
		pub(super) decimals: u8,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type AssetId: Member + Parameter + Default + Copy + HasCompact;

		type WeightInfo: WeightInfo;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AssetId = "AssetId", T::AccountId = "AccountId")]
	pub enum Event<T: Config>
	where
	{
		Created(T::AssetId),
		Issued(T::AssetId, T::AccountId, U256),
		Burned(T::AssetId, T::AccountId, U256),
		Transferred(T::AssetId, T::AccountId, T::AccountId, U256),
	}

	#[pallet::error]
	pub enum Error<T> {
		InUse,
		Unknown,
	}

	#[pallet::storage]
	pub(super) type Asset<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		AssetDetails,
		OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type Metadata<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		AssetMetadata,
		ValueQuery,
	>;

	#[pallet::storage]
	pub(super) type Account<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Blake2_128Concat,
		T::AccountId,
		AssetBalance,
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			id: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			amount: U256
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {

		pub(super) fn can_increase(id: T::AssetId, who: &T::AccountId, amount: U256) -> DepositConsequence {
			let details = match Asset::<T>::get(id) {
				Some(details) => details,
				None => return DepositConsequence::UnknownAsset,
			};

			if details.supply.checked_add(amount).is_none() {
				return DepositConsequence::Overflow
			}

			let account = Account::<T>::get(id, who);
			if account.balance.checked_add(amount).is_none() {
				return DepositConsequence::Overflow
			}

			DepositConsequence::Success
		}

		pub(super) fn can_decrease(
			id: T::AssetId,
			who: &T::AccountId,
			amount: U256,
		) -> WithdrawConsequence<u128> {
			let details = match Asset::<T>::get(id) {
				Some(details) => details,
				None => return WithdrawConsequence::UnknownAsset,
			};

			if details.supply.checked_sub(amount).is_none() {
				return WithdrawConsequence::Underflow
			}

			let account = Account::<T>::get(id, who);

			if let Some(rest) = account.balance.checked_sub(amount) {
				WithdrawConsequence::Success
			} else {
				WithdrawConsequence::NoFunds
			}
		}

		pub(super) fn do_deposit(id: T::AssetId, who: &T::AccountId, amount: U256) -> DispatchResult  {
			if amount.is_zero() {
				return Ok(())
			}
			Self::can_increase(id, who, amount).into_result()?;
			Asset::<T>::try_mutate(id, |maybe_details| -> DispatchResult {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;

				Account::<T>::try_mutate(id, who, |account| -> Result<(), DispatchError> {
					let new_balance = account.balance.saturating_add(amount);
					account.balance = new_balance;
					details.supply = details.supply.saturating_add(amount);
					Ok(())
				})
			})?;
			Self::deposit_event(Event::Issued(id, who.clone(), amount));
			Ok(())
		}

		pub(super) fn do_withdraw(id: T::AssetId, who: &T::AccountId, amount: U256) -> DispatchResult  {
			if amount.is_zero() {
				return Ok(())
			}
			Self::can_decrease(id, who, amount).into_result()?;
			Asset::<T>::try_mutate(id, |maybe_details| -> DispatchResult {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;

				Account::<T>::try_mutate(id, who, |account| -> Result<(), DispatchError> {
					let new_balance = account.balance.saturating_sub(amount);
					account.balance = new_balance;
					details.supply = details.supply.saturating_sub(amount);
					Ok(())
				})
			})?;
			Self::deposit_event(Event::Burned(id, who.clone(), amount));
			Ok(())
		}
	}

}

// pub trait FungibleAssets<AccountId, AssetId>
// {
// 	fn create(asset_id: AssetId) -> DispatchResult;

// 	fn set_metadata(
// 		asset_id: AssetId,
// 		name: Vec<u8>,
// 		symbol: Vec<u8>,
// 		decimals: u8
// 	) -> DispatchResult;

// 	fn supply(id: AssetId) -> U256;

// 	fn balance(id: AssetId, who: &AccountId) -> U256;

// 	fn transfer(
// 		id: AssetId,
// 		from: &AccountId,
// 		to: &AccountId,
// 		amount: U256
// 	) -> DispatchResult;

// 	fn withdraw(
// 		id: AssetId,
// 		who: &AccountId,
// 		amount: U256
// 	) -> DispatchResult;

// 	fn deposit(
// 		id: AssetId,
// 		who: &AccountId,
// 		amount: U256
// 	) -> DispatchResult;
// }

// impl<T: Config> FungibleAssets<T::AccountId, T::AssetId> for Pallet<T> {

// 	fn create(id: T::AssetId) -> Result<T::AssetId, DispatchError> {
// 		ensure!(!Asset::<T>::contains_key(id), Error::<T>::InUse);

// 		Asset::<T>::insert(id, AssetDetails {
// 			supply: U256::zero(),
// 		});

// 		Pallet::<T>::deposit_event(Event::Created(id));
// 	}

// 	fn supply(id: T::AssetId) -> U256 {
// 		Asset::<T>::get(id).map(|x| x.supply).unwrap_or_else(U256::zero())
// 	}

// 	fn balance(id: T::AssetId, who: &T::AccountId) -> U256 {
// 		<Account<T>>::get(id, who)
// 	}



// 	fn withdraw(asset_id: T::AssetId, who: &T::AccountId, amount: U256) -> DispatchResult  {
// 		if amount.is_zero() {
// 			return Ok(())
// 		}
// 		<Balances<T>>::try_mutate(asset_id, who, |balance| -> Result<(), DispatchError> {
// 			let current_total_issuance = Self::total_issuance(asset_id);
// 			let new_total_issuance = current_total_issuance.checked_sub(amount)
// 				.ok_or(Error::<T>::TotalIssuanceUnderflow)?;
// 			*balance = balance.checked_sub(amount)
// 				.ok_or(Error::<T>::InsufficientBalance)?;
// 			<TotalIssuance<T>>::insert(asset_id, new_total_issuance);
// 			Ok(())
// 		})
// 	}

// 	fn transfer(
// 		asset_id: T::AssetId,
// 		from: &T::AccountId,
// 		to: &T::AccountId,
// 		amount: U256)
// 	-> DispatchResult {
// 		if amount.is_zero() || from == to {
// 			return Ok(())
// 		}
// 		<Balances<T>>::try_mutate(asset_id, from, |from_balance| -> DispatchResult {
// 			<Balances<T>>::try_mutate(asset_id, to, |to_balance| -> DispatchResult {
// 				*from_balance = from_balance.checked_sub(amount).ok_or(Error::<T>::InsufficientBalance)?;
// 				*to_balance = to_balance.checked_add(amount).ok_or(Error::<T>::BalanceOverflow)?;
// 				Ok(())
// 			})
// 		})
// 	}
// }
