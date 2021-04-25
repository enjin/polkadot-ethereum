use sp_runtime::TokenError;
use sp_core::U256;

/// One of a number of consequences of withdrawing a fungible from an account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum WithdrawConsequence {
        /// Withdraw could not happen since the amount to be withdrawn is less than the total funds in
        /// the account.
        NoFunds,
        /// The withdraw would mean the account dying when it needs to exist (usually because it is a
        /// provider and there are consumer references on it).
        WouldDie,
        /// The asset is unknown. Usually because an `AssetId` has been presented which doesn't exist
        /// on the system.
        UnknownAsset,
        /// There has been an underflow in the system. This is indicative of a corrupt state and
        /// likely unrecoverable.
        Underflow,
        /// There has been an overflow in the system. This is indicative of a corrupt state and
        /// likely unrecoverable.
        Overflow,
        /// Not enough of the funds in the account are unavailable for withdrawal.
        Frozen,
        /// Account balance would reduce to zero, potentially destroying it. The parameter is the
        /// amount of balance which is destroyed.
        ReducedToZero(U256),
        /// Account continued in existence.
        Success,
}

impl WithdrawConsequence {
    /// Convert the type into a `Result` with `TokenError` as the error or the additional `Balance`
    /// by which the account will be reduced.
    pub fn into_result(self) -> Result<U256, TokenError> {
            use WithdrawConsequence::*;
            match self {
                    NoFunds => Err(TokenError::NoFunds),
                    WouldDie => Err(TokenError::WouldDie),
                    UnknownAsset => Err(TokenError::UnknownAsset),
                    Underflow => Err(TokenError::Underflow),
                    Overflow => Err(TokenError::Overflow),
                    Frozen => Err(TokenError::Frozen),
                    ReducedToZero(result) => Ok(result),
                    Success => Ok(U256::zero()),
            }
    }
}

/// One of a number of consequences of withdrawing a fungible from an account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DepositConsequence {
        /// Deposit couldn't happen due to the amount being too low. This is usually because the
        /// account doesn't yet exist and the deposit wouldn't bring it to at least the minimum needed
        /// for existance.
        BelowMinimum,
        /// Deposit cannot happen since the account cannot be created (usually because it's a consumer
        /// and there exists no provider reference).
        CannotCreate,
        /// The asset is unknown. Usually because an `AssetId` has been presented which doesn't exist
        /// on the system.
        UnknownAsset,
        /// An overflow would occur. This is practically unexpected, but could happen in test systems
        /// with extremely small balance types or balances that approach the max value of the balance
        /// type.
        Overflow,
        /// Account continued in existence.
        Success,
}

impl DepositConsequence {
    pub fn into_result(self) -> Result<(), TokenError> {
        use DepositConsequence::*;
        Err(match self {
                BelowMinimum => TokenError::BelowMinimum,
                CannotCreate => TokenError::CannotCreate,
                UnknownAsset => TokenError::UnknownAsset,
                Overflow => TokenError::Overflow,
                Success => return Ok(()),
        })
    }
}
