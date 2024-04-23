#![cfg_attr(not(feature = "std"), no_std)]

mod account;
mod signing;

// TODO: move ethereum-types re-exports here, and use from here in dep crates
#[cfg(any(feature = "std", test))]
mod input;

#[cfg(any(feature = "std", test))]
pub use input::{compose_and_sign_tx, ContractInput, EthTxInput};

pub use account::AccountId20;
pub use signing::EthereumSignature;

pub use ethereum::{
    AccessListItem, BlockV2 as Block, EnvelopedDecodable, EnvelopedEncodable, LegacyTransaction,
    LegacyTransactionMessage, Log, ReceiptV3 as Receipt, TransactionAction, TransactionSignature,
    TransactionV2 as EthTransaction,
};

pub use ethereum_types::{H160, H256, H64, U256, U64};
