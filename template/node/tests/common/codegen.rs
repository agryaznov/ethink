use alloy::sol;
use serde::{Deserialize, Serialize};
// Codegen from ABI file to interact with the contract.
// Basic ERC20 interface
sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    IERC20,
    "tests/abi/IERC20Minimal.json"
);
// Codegen from ABI file to interact with the contract.
// Flipper.ink contract expressed in Solidity
sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    IFlipper,
    "tests/abi/IFlipper.json"
);
