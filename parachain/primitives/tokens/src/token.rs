use frame_support::dispatch::{Parameter, DispatchResult};
use sp_core::U256;
use sp_runtime::{TokenError, traits::Member};

use super::{DepositConsequence, WithdrawConsequence};

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
