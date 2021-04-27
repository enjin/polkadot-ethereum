use frame_support::traits::{SameOrOther, TryDrop};

use sp_core::U256;
use sp_std::marker::PhantomData;

use super::*;
pub trait Balanced<AccountId>: Inspect<AccountId> {

	/// The type for managing what happens when an instance of `Debt` is dropped without being used.
	type OnDropDebt: HandleImbalanceDrop;
	/// The type for managing what happens when an instance of `Credit` is dropped without being
	/// used.
	type OnDropCredit: HandleImbalanceDrop;

	fn deposit(
		who: &AccountId,
		amount: U256
	) -> Result<DebtOf<AccountId, Self>, DispatchError>;

	fn withdraw(
		who: &AccountId,
		amount: U256
	) -> Result<CreditOf<AccountId, Self>, DispatchError>;

	/// The balance of `who` is increased in order to counter `credit`. If the whole of `credit`
	/// cannot be countered, then nothing is changed and the original `credit` is returned in an
	/// `Err`.
	///
	fn resolve(
		who: &AccountId,
		credit: CreditOf<AccountId, Self>,
	) -> Result<(), CreditOf<AccountId, Self>> {
		let v = credit.peek();
		let debt = match Self::deposit(who, v) {
			Err(_) => return Err(credit),
			Ok(d) => d,
		};
		if let Ok(result) = credit.offset(debt) {
			let result = result.try_drop();
			debug_assert!(result.is_ok(), "ok deposit return must be equal to credit value; qed");
		} else {
			debug_assert!(false, "debt.asset is credit.asset; qed");
		}
		Ok(())
	}

	/// The balance of `who` is decreased in order to counter `debt`. If the whole of `debt`
	/// cannot be countered, then nothing is changed and the original `debt` is returned in an
	/// `Err`.
	fn settle(
		who: &AccountId,
		debt: DebtOf<AccountId, Self>,
	) -> Result<CreditOf<AccountId, Self>, DebtOf<AccountId, Self>> {
		let amount = debt.peek();
		let credit = match Self::withdraw(who, amount) {
			Err(_) => return Err(debt),
			Ok(d) => d,
		};
		match credit.offset(debt) {
			Ok(SameOrOther::None) => Ok(CreditOf::<AccountId, Self>::zero()),
			Ok(SameOrOther::Same(dust)) => Ok(dust),
			Ok(SameOrOther::Other(rest)) => {
				debug_assert!(false, "ok withdraw return must be at least debt value; qed");
				Err(rest)
			}
			Err(_) => {
				debug_assert!(false, "debt.asset is credit.asset; qed");
				Ok(CreditOf::<AccountId, Self>::zero())
			}
		}
	}
}
pub trait Unbalanced<AccountId>: Inspect<AccountId> {
	/// Set the total issuance of `asset` to `amount`.
	fn set_total_issuance(amount: U256);

	/// Reduce the balance of `who` by `amount`. If it cannot be reduced by that amount for
	/// some reason, return `Err` and don't reduce it at all. If Ok, return the imbalance.
	///
	/// Minimum balance will be respected and the returned imbalance may be up to
	/// `Self::minimum_balance() - 1` greater than `amount`.
	fn decrease_balance(who: &AccountId, amount: U256) -> Result<U256, DispatchError>;

	/// Increase the balance of `who` by `amount`. If it cannot be increased by that amount
	/// for some reason, return `Err` and don't increase it at all. If Ok, return the imbalance.
	///
	/// Minimum balance will be respected and an error will be returned if
	/// `amount < Self::minimum_balance()` when the account of `who` is zero.
	fn increase_balance(who: &AccountId, amount: U256) -> Result<U256, DispatchError>;

}

pub struct DecreaseIssuance<AccountId, U>(PhantomData<(AccountId, U)>);
impl<AccountId, U: Unbalanced<AccountId>> HandleImbalanceDrop
	for DecreaseIssuance<AccountId, U>
{
	fn handle(amount: U256) {
		U::set_total_issuance(U::total_issuance().saturating_sub(amount))
	}
}

pub struct IncreaseIssuance<AccountId, U>(PhantomData<(AccountId, U)>);
impl<AccountId, U: Unbalanced<AccountId>> HandleImbalanceDrop
	for IncreaseIssuance<AccountId, U>
{
	fn handle(amount: U256) {
		U::set_total_issuance(U::total_issuance().saturating_add(amount))
	}
}
