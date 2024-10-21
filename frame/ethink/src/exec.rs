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

#[macro_export]
macro_rules! impl_ethink_executor {
    ($conf:ident,$contr:ident) => {
        impl pallet_ethink::Executor<$conf> for $contr {
            type ExecResult = ContractExecResult<
                BalanceOf<$conf>,
                frame_system::EventRecord<
                    <$conf as frame_system::Config>::RuntimeEvent,
                    <$conf as frame_system::Config>::Hash,
                >,
            >;

            fn is_contract(who: &<$conf as frame_system::Config>::AccountId) -> bool {
                Self::code_hash(who).is_some()
            }

            /// Estimate gas
            fn gas_estimate(
                from: <$conf as frame_system::Config>::AccountId,
                to: <$conf as frame_system::Config>::AccountId,
                data: Vec<u8>,
                value: BalanceOf<$conf>,
                gas_limit: Weight,
            ) -> Result<U256, DispatchError> {
                if Self::is_contract(&to) {
                    let res = <Self as Executor<$conf>>::call(from, to, data, value, gas_limit);
                    // ensure successful execution
                    let _ = res.result?;
                    // get consumed gas
                    let gas_consumed = res.gas_consumed.ref_time();
                    Ok(gas_consumed.into())
                } else {
                    // Standard base fee
                    // TODO put to ethink constants
                    Ok(U256::from(21000u32))
                }
            }

            fn build_call(
                to: <$conf as frame_system::Config>::AccountId,
                value: U256,
                data: Vec<u8>,
                gas_limit: U256,
            ) -> Option<<$conf as frame_system::Config>::RuntimeCall> {
                let dest = sp_runtime::MultiAddress::Id(to.into());
                // TODO proper ERR on conversion failures
                let value = value.try_into().ok()?;
                let gas_limit = gas_limit.try_into().ok()?;
                // TODO this logic to be encapsulated in ep_ crate,
                // and re-used from there here and in the rpc
                let gas_limit = Weight::from_parts(gas_limit, u64::MAX);

                Some(if Self::is_contract(&to) {
                    pallet_contracts::Call::<$conf>::call {
                        dest,
                        value,
                        data,
                        gas_limit,
                        storage_deposit_limit: None,
                    }
                    .into()
                } else {
                    // NOTE basically pallet-contracts can do this for us, as its call() extrinsic
                    // handles the call made to user account in a similar fashion.
                    // However, we keep this logic here not to rely on particular executor pallet too much.
                    pallet_balances::Call::<$conf>::transfer_allow_death { dest, value }.into()
                })
            }

            fn call(
                from: <$conf as frame_system::Config>::AccountId,
                to: <$conf as frame_system::Config>::AccountId,
                data: Vec<u8>,
                value: BalanceOf<$conf>,
                gas_limit: Weight,
            ) -> Self::ExecResult {
                Self::bare_call(
                    from,
                    to,
                    value,
                    gas_limit,
                    None,
                    data,
                    CONTRACTS_DEBUG_OUTPUT,
                    CONTRACTS_EVENTS,
                    pallet_contracts::Determinism::Enforced,
                )
            }
        }
    };
}
