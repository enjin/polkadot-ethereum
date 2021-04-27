use frame_support::dispatch::{DispatchResult, DispatchError};
use frame_support::traits::Get;

use sp_core::U256;

use crate::multi;

mod balanced;
mod imbalance;

pub use balanced::{Balanced, Unbalanced};
pub use imbalance::{Imbalance, HandleImbalanceDrop, CreditOf, DebtOf};

pub use crate::consequence::{DepositConsequence, WithdrawConsequence};

pub trait Inspect<AccountId> {
	fn total_issuance() -> U256;

	fn balance(who: &AccountId) -> U256;

	fn can_deposit(
        who: &AccountId,
        amount: U256,
    ) -> DepositConsequence;

	fn can_withdraw(
		who: &AccountId,
		amount: U256,
	) -> WithdrawConsequence;
}

pub trait Mutate<AccountId>: Inspect<AccountId> {
	fn mint(who: &AccountId, amount: U256) -> DispatchResult;

	fn burn(who: &AccountId, amount: U256) -> DispatchResult;

    fn transfer(
		source: &AccountId,
		dest: &AccountId,
		amount: U256
	) -> DispatchResult;
}


pub struct ItemOf<
	F: multi::Inspect<AccountId>,
	A: Get<F::AssetId>,
	AccountId,
>(
	sp_std::marker::PhantomData<(F, A, AccountId)>
);

impl<
	F: multi::Inspect<AccountId>,
	A: Get<F::AssetId>,
	AccountId,
> Inspect<AccountId> for ItemOf<F, A, AccountId> {
	fn total_issuance() -> U256 {
		F::total_issuance(A::get())
	}
	fn balance(who: &AccountId) -> U256 {
		F::balance(A::get(), who)
	}
	fn can_deposit(who: &AccountId, amount: U256) -> DepositConsequence {
		F::can_deposit(A::get(), who, amount)
	}
	fn can_withdraw(who: &AccountId, amount: U256) -> WithdrawConsequence {
		F::can_withdraw(A::get(), who, amount)
	}
}

impl<
	F: multi::Mutate<AccountId>,
	A: Get<F::AssetId>,
	AccountId,
> Mutate<AccountId> for ItemOf<F, A, AccountId> {
	fn mint(who: &AccountId, amount: U256) -> DispatchResult {
		F::mint(A::get(), who, amount)
	}
	fn burn(who: &AccountId, amount: U256) -> DispatchResult {
		F::burn(A::get(), who, amount)
	}

    fn transfer(
		source: &AccountId,
		dest: &AccountId,
		amount: U256
	) -> DispatchResult {
        F::transfer(A::get(), source, dest, amount)
    }
}

impl<
	F: multi::Unbalanced<AccountId>,
	A: Get<F::AssetId>,
	AccountId,
> Unbalanced<AccountId> for ItemOf<F, A, AccountId> {
	fn set_balance(who: &AccountId, amount: U256) -> DispatchResult {
		F::set_balance(A::get(), who, amount)
	}
	fn set_total_issuance(amount: U256) -> () {
		F::set_total_issuance(A::get(), amount)
	}
	fn decrease_balance(who: &AccountId, amount: U256) -> Result<U256, DispatchError> {
		F::decrease_balance(A::get(), who, amount)
	}
	fn increase_balance(who: &AccountId, amount: U256) -> Result<U256, DispatchError> {
		F::increase_balance(A::get(), who, amount)
	}
}
