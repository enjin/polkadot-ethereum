use frame_support::traits::{SameOrOther, TryDrop};

use sp_core::U256;
use sp_std::marker::PhantomData;

use super::*;
pub trait HandleImbalanceDrop<AssetId>
{
	fn handle(asset: AssetId, amount: U256);
}

pub struct Imbalance<A, OnDrop, OppositeOnDrop>
where
	A: AssetId,
	OnDrop: HandleImbalanceDrop<A>,
	OppositeOnDrop: HandleImbalanceDrop<A>,
{
	asset: A,
	amount: U256,
	_phantom: PhantomData<(OnDrop, OppositeOnDrop)>,
}

impl<
	A: AssetId,
	OnDrop: HandleImbalanceDrop<A>,
	OppositeOnDrop: HandleImbalanceDrop<A>
> Drop for Imbalance<A, OnDrop, OppositeOnDrop> {
	fn drop(&mut self) {
		if !self.amount.is_zero() {
			OnDrop::handle(self.asset, self.amount)
		}
	}
}

impl<
	A: AssetId,
	OnDrop: HandleImbalanceDrop<A>,
	OppositeOnDrop: HandleImbalanceDrop<A>,
> TryDrop for Imbalance<A, OnDrop, OppositeOnDrop> {
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

impl<A, OnDrop, OppositeOnDrop> Imbalance<A, OnDrop, OppositeOnDrop>
where
	A: AssetId,
	OnDrop: HandleImbalanceDrop<A>,
	OppositeOnDrop: HandleImbalanceDrop<A>,
{
	pub(crate) fn new(asset: A, amount: U256) -> Self {
		Imbalance {
			asset,
			amount,
			_phantom: PhantomData
		}
	}

	pub fn zero(asset: A) -> Self {
		Imbalance {
			asset,
			amount: U256::zero(),
			_phantom: PhantomData
		}
	}

	pub fn split(self, amount: U256) -> (Self, Self) {
		let first = self.amount.min(amount);
		let second = self.amount - first;
		let asset = self.asset;
		sp_std::mem::forget(self);
		(Imbalance::new(asset, first), Imbalance::new(asset, second))
	}

	pub fn offset(self, other: Imbalance<A, OppositeOnDrop, OnDrop>) -> Result<
		SameOrOther<Self, Imbalance<A, OppositeOnDrop, OnDrop>>,
		(Self, Imbalance<A, OppositeOnDrop, OnDrop>),
	> {
		if self.asset == other.asset {
			let (a, b) = (self.amount, other.amount);
			let asset = self.asset;
			sp_std::mem::forget((self, other));

			if a == b {
				Ok(SameOrOther::None)
			} else if a > b {
				Ok(SameOrOther::Same(Imbalance::new(asset, a - b)))
			} else {
				Ok(SameOrOther::Other(Imbalance::<A, OppositeOnDrop, OnDrop>::new(asset, b - a)))
			}
		} else {
			Err((self, other))
		}
	}

	pub fn peek(&self) -> U256 {
		self.amount
	}

	pub fn asset(&self) -> A {
		self.asset
	}
}

pub type CreditOf<AccountId, B> = Imbalance<
	<B as Inspect<AccountId>>::AssetId,
	<B as Balanced<AccountId>>::OnDropCredit,
	<B as Balanced<AccountId>>::OnDropDebt,
>;


pub type DebtOf<AccountId, B> = Imbalance<
	<B as Inspect<AccountId>>::AssetId,
	<B as Balanced<AccountId>>::OnDropDebt,
	<B as Balanced<AccountId>>::OnDropCredit,
>;
