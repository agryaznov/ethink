// SPDX-License-Identifier: Apache-2.0
// This file was derived from Frontier (fp-rpc),
// and modified to become part of Ethink.
//
// Copyright (c) (Frontier): 2020-2022 Parity Technologies (UK) Ltd.
// Copyright (c) (Ethink):   2023-2024 Alexander Gryaznov.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![deny(unused_crate_dependencies)]

pub use ethereum::TransactionV2 as EthTransaction;
// Substrate
use sp_core::{H160, U256};
use sp_runtime::traits::Block as BlockT;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    /// Runtime-exposed API necessary for ETH-compatibility layer.
    pub trait ETHRuntimeRPC {
        /// Return runtime-defined CHAIN_ID.
        fn chain_id() -> u64;

        /// Return account balance.
        fn account_free_balance(address: H160) -> U256;

        /// Return account nonce.
        fn nonce(address: H160) -> U256;

        /// Call contract (without extrinsic submission)
        fn call(
            from: H160,
            to: H160,
            data: Vec<u8>,
            value: U256,
            gas_limit: U256,
        ) -> Result<Vec<u8>, sp_runtime::DispatchError>;

        /// Estimate gas needed for a contract call
        fn gas_estimate(
            from: H160,
            to: H160,
            data: Vec<u8>,
            value: U256,
            gas_limit: U256,
        ) -> Result<U256, sp_runtime::DispatchError>;

        /// Wrap Ethereum transaction into an extrinsic
        fn build_extrinsic(from: ethereum::TransactionV2) -> <Block as BlockT>::Extrinsic;
    }
}
