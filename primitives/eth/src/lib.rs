#![cfg_attr(not(feature = "std"), no_std)]
// TODO: merge this crate with ep-crypto
// TODO: move ethereum-types re-exports here, and use from here in dep crates
#[cfg(any(feature = "std", test))]
mod input;

#[cfg(any(feature = "std", test))]
pub use input::{compose_and_sign_tx, ContractInput, EthTxInput};

pub use ep_crypto::{AccountId20, EthereumSignature};

pub use ethereum::{
    AccessListItem, BlockV2 as Block, EnvelopedEncodable, LegacyTransaction,
    LegacyTransactionMessage, Log, ReceiptV3 as Receipt, TransactionAction,
    TransactionV2 as EthTransaction, EnvelopedDecodable, TransactionSignature,
};
