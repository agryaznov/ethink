use super::*;

/// Provider of the contracts functionality
/// Currently this is pallet_contracts, though might be changed in the future.
pub trait Executor<T: pallet::Config> {
    type ExecResult;

    /// Check if AccountId is owned by a contract
    fn is_contract(who: &T::AccountId) -> bool;
    /// Construct proper runtime call for the input provided
    fn build_call(
        to: T::AccountId,
        value: U256,
        data: Vec<u8>,
        gas_limit: U256,
    ) -> Option<T::RuntimeCall>;
    /// Call contract
    fn call(
        from: T::AccountId,
        to: T::AccountId,
        data: Vec<u8>,
        value: BalanceOf<T>,
        gas_limit: Weight,
    ) -> Self::ExecResult;
    /// Estimate gas
    fn gas_estimate(
        from: T::AccountId,
        to: T::AccountId,
        data: Vec<u8>,
        value: BalanceOf<T>,
        gas_limit: Weight,
    ) -> Result<U256, DispatchError>;
}
