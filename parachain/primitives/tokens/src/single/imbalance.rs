use frame_support::traits::{SameOrOther, TryDrop};

use sp_core::U256;
use sp_std::marker::PhantomData;

use super::*;

pub trait HandleImbalanceDrop
{
	fn handle(amount: U256);
}

pub struct Imbalance<OnDrop, OppositeOnDrop>
where
	OnDrop: HandleImbalanceDrop,
	OppositeOnDrop: HandleImbalanceDrop,
{
	amount: U256,
	_phantom: PhantomData<(OnDrop, OppositeOnDrop)>,
}

impl<
	OnDrop: HandleImbalanceDrop,
	OppositeOnDrop: HandleImbalanceDrop
> Drop for Imbalance<OnDrop, OppositeOnDrop> {
	fn drop(&mut self) {
		if !self.amount.is_zero() {
			OnDrop::handle(self.amount)
		}
	}
}

impl<
	OnDrop: HandleImbalanceDrop,
	OppositeOnDrop: HandleImbalanceDrop,
> TryDrop for Imbalance<OnDrop, OppositeOnDrop> {
	/// Drop an instance cleanly. Only works if its value represents "no-operation".
	fn try_drop(self) -> Result<(), Self> {
		if self.amount.is_zero() {
			sp_std::mem::forget(self);
			Ok(())
		} else {
			Err(self)
		}
	}
}

impl<OnDrop, OppositeOnDrop> Imbalance<OnDrop, OppositeOnDrop>
where
	OnDrop: HandleImbalanceDrop,
	OppositeOnDrop: HandleImbalanceDrop,
{
	pub(crate) fn new(amount: U256) -> Self {
		Imbalance {
			amount,
			_phantom: PhantomData
		}
	}

	pub fn zero() -> Self {
		Imbalance {
			amount: U256::zero(),
			_phantom: PhantomData
		}
	}

	pub fn split(self, amount: U256) -> (Self, Self) {
		let first = self.amount.min(amount);
		let second = self.amount - first;
		sp_std::mem::forget(self);
		(Imbalance::new(first), Imbalance::new(second))
	}

	pub fn offset(self, other: Imbalance<OppositeOnDrop, OnDrop>) -> Result<
		SameOrOther<Self, Imbalance<OppositeOnDrop, OnDrop>>,
		(Self, Imbalance<OppositeOnDrop, OnDrop>),
	> {
        let (a, b) = (self.amount, other.amount);
        sp_std::mem::forget((self, other));

        if a == b {
            Ok(SameOrOther::None)
        } else if a > b {
            Ok(SameOrOther::Same(Imbalance::new(a - b)))
        } else {
            Ok(SameOrOther::Other(Imbalance::<OppositeOnDrop, OnDrop>::new(b - a)))
        }
	}

	pub fn peek(&self) -> U256 {
		self.amount
	}
}

pub type CreditOf<AccountId, B> = Imbalance<
	<B as Balanced<AccountId>>::OnDropCredit,
	<B as Balanced<AccountId>>::OnDropDebt,
>;

pub type DebtOf<AccountId, B> = Imbalance<
	<B as Balanced<AccountId>>::OnDropDebt,
	<B as Balanced<AccountId>>::OnDropCredit,
>;
