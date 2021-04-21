
use frame_support::weights::Weight;


/// Weight functions needed for this pallet.
pub trait WeightInfo {
	fn transfer() -> Weight;
}

impl WeightInfo for () {
	fn transfer() -> Weight { 0 }
}
