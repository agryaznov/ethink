// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
// This file was derived from Frontier (fc-rpc-core),
// and modified to become part of Ethink.
//
// Copyright (c) (Frontier): 2020-2022 Parity Technologies (UK) Ltd.
// Copyright (c) (Ethink):   2023-2024 Alexander Gryaznov.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! RPC types

mod block;
mod block_number;
mod bytes;
mod call_request;
mod fee;
mod index;
mod log;
mod receipt;
mod sync;
mod transaction;
mod transaction_request;
mod work;

use serde::{de::Error, Deserialize, Deserializer};

pub use self::{
    block::{Block, BlockTransactions, Header, Rich, RichBlock, RichHeader},
    block_number::BlockNumber,
    bytes::Bytes,
    call_request::{CallRequest, CallStateOverride},
    fee::{FeeHistory, FeeHistoryCache, FeeHistoryCacheItem, FeeHistoryCacheLimit},
    index::Index,
    log::Log,
    receipt::Receipt,
    sync::{
        ChainStatus, EthProtocolInfo, PeerCount, PeerInfo, PeerNetworkInfo, PeerProtocolsInfo,
        Peers, PipProtocolInfo, SyncInfo, SyncStatus, TransactionStats,
    },
    transaction::{LocalTransactionStatus, RichRawTransaction, Transaction},
    transaction_request::{TransactionRequest, TxMessage},
    work::Work,
};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
pub(crate) struct CallOrInputData {
    data: Option<Bytes>,
    input: Option<Bytes>,
}

/// Function to deserialize `data` and `input`  within `TransactionRequest` and `CallRequest`.
/// It verifies that if both `data` and `input` are provided, they must be identical.
pub(crate) fn deserialize_data_or_input<'d, D: Deserializer<'d>>(
    d: D,
) -> Result<Option<Bytes>, D::Error> {
    let CallOrInputData { data, input } = CallOrInputData::deserialize(d)?;
    match (&data, &input) {
        (Some(data), Some(input)) => {
            if data == input {
                Ok(Some(data.clone()))
            } else {
                Err(D::Error::custom(
                    "Ambiguous value for `data` and `input`".to_string(),
                ))
            }
        }
        (_, _) => Ok(data.or(input)),
    }
}
