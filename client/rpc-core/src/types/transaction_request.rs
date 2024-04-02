// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2015-2022 Parity Technologies (UK) Ltd.
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

//! `TransactionRequest` type

use ethereum::{AccessListItem, LegacyTransactionMessage};
use ethereum_types::{H160, U256};
use serde::{Deserialize, Serialize};

use crate::types::{deserialize_data_or_input, Bytes};

// TODO we don't really need to support all of these,
// because it's not a real eth tx in our use case, just something that looks like one
pub struct TxMessage(pub LegacyTransactionMessage);

/// Transaction request coming from RPC
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    /// Sender
    pub from: Option<H160>,
    /// Recipient
    pub to: Option<H160>,
    /// Gas Price, legacy.
    #[serde(default)]
    pub gas_price: Option<U256>,
    /// Max BaseFeePerGas the user is willing to pay.
    #[serde(default)]
    pub max_fee_per_gas: Option<U256>,
    /// The miner's tip.
    #[serde(default)]
    pub max_priority_fee_per_gas: Option<U256>,
    /// Gas
    pub gas: Option<U256>,
    /// Value of transaction in wei
    pub value: Option<U256>,
    /// Additional data sent with transaction
    #[serde(deserialize_with = "deserialize_data_or_input", flatten)]
    pub data: Option<Bytes>,
    /// Transaction's nonce
    pub nonce: Option<U256>,
    /// Pre-pay to warm storage access.
    #[serde(default)]
    pub access_list: Option<Vec<AccessListItem>>,
    /// EIP-2718 type
    #[serde(rename = "type")]
    pub transaction_type: Option<U256>,
}

impl From<TransactionRequest> for TxMessage {
    fn from(req: TransactionRequest) -> Self {
        TxMessage(LegacyTransactionMessage {
            nonce: req.nonce.unwrap_or_default(), // This doesn't count anyways (TODO though should it?)
            gas_price: req.gas_price.unwrap_or_default(), // This doesn't count anyways
            gas_limit: req.gas.unwrap_or_default(), // No gas_limit defaults 0 (TODO could be changed to MAX (no limit))
            value: req.value.unwrap_or_default(),   // No value defaults to 0
            input: req.data.map(|s| s.into_vec()).unwrap_or_default(), // No data defaults to vec![]
            action: match req.to {
                Some(to) => ethereum::TransactionAction::Call(to),
                None => ethereum::TransactionAction::Create,
            },
            chain_id: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_with_only_input() {
        let data = json!({
            "from": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b",
            "to": "0x13fe2d1d3665660d22ff9624b7be0551ee1ac91b",
            "gasPrice": "0x10",
            "maxFeePerGas": "0x20",
            "maxPriorityFeePerGas": "0x30",
            "gas": "0x40",
            "value": "0x50",
            "input": "0x123abc",
            "nonce": "0x60",
            "accessList": [{"address": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b", "storageKeys": []}],
            "type": "0x70"
        });

        let request: Result<TransactionRequest, _> = serde_json::from_value(data);
        assert!(request.is_ok());

        let request = request.unwrap();
        assert_eq!(request.data, Some(Bytes::from(vec![0x12, 0x3a, 0xbc])));
    }

    #[test]
    fn test_deserialize_with_only_data() {
        let data = json!({
            "from": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b",
            "to": "0x13fe2d1d3665660d22ff9624b7be0551ee1ac91b",
            "gasPrice": "0x10",
            "maxFeePerGas": "0x20",
            "maxPriorityFeePerGas": "0x30",
            "gas": "0x40",
            "value": "0x50",
            "data": "0x123abc",
            "nonce": "0x60",
            "accessList": [{"address": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b", "storageKeys": []}],
            "type": "0x70"
        });

        let request: Result<TransactionRequest, _> = serde_json::from_value(data);
        assert!(request.is_ok());

        let request = request.unwrap();
        assert_eq!(request.data, Some(Bytes::from(vec![0x12, 0x3a, 0xbc])));
    }

    #[test]
    fn test_deserialize_with_data_and_input_mismatch() {
        let data = json!({
            "from": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b",
            "to": "0x13fe2d1d3665660d22ff9624b7be0551ee1ac91b",
            "gasPrice": "0x10",
            "maxFeePerGas": "0x20",
            "maxPriorityFeePerGas": "0x30",
            "gas": "0x40",
            "value": "0x50",
            "data": "0x123abc",
            "input": "0x456def",
            "nonce": "0x60",
            "accessList": [{"address": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b", "storageKeys": []}],
            "type": "0x70"
        });

        let request: Result<TransactionRequest, _> = serde_json::from_value(data);
        assert!(request.is_err());
    }

    #[test]
    fn test_deserialize_with_data_and_input_equal() {
        let data = json!({
            "from": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b",
            "to": "0x13fe2d1d3665660d22ff9624b7be0551ee1ac91b",
            "gasPrice": "0x10",
            "maxFeePerGas": "0x20",
            "maxPriorityFeePerGas": "0x30",
            "gas": "0x40",
            "value": "0x50",
            "data": "0x123abc",
            "input": "0x123abc",
            "nonce": "0x60",
            "accessList": [{"address": "0x60be2d1d3665660d22ff9624b7be0551ee1ac91b", "storageKeys": []}],
            "type": "0x70"
        });

        let request: Result<TransactionRequest, _> = serde_json::from_value(data);
        assert!(request.is_ok());

        let request = request.unwrap();
        assert_eq!(request.data, Some(Bytes::from(vec![0x12, 0x3a, 0xbc])));
    }
}
