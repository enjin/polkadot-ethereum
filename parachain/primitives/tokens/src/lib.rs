#![cfg_attr(not(feature = "std"), no_std)]
pub mod consequence;
pub mod single;
pub mod multi;

pub use consequence::{DepositConsequence, WithdrawConsequence};
