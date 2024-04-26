# End-to-End Tests

_ethink!_ comes with e2e testing framework. It allows to write self-describing e2e tests, each completing the same set of actions as happens upon a user interaction with _ethink!_: building and deploying contract to the node, then interacting with it via exposed Ethereum RPC endpoint, and validating the resulting state changes and the values returned.

The following integration test suites are ready to run on the _ethink!_ template node:

+ [flipper](https://github.com/agryaznov/ethink/blob/master/template/node/tests/flipper.rs): basic tests for the RPC methods;  
+ [erc20](https://github.com/agryaznov/ethink/blob/master/template/node/tests/erc20.rs): ERC20 contract tests.
+ _(more to be added later)_

Use this command to run the integration tests (at the project root): 

```bash
cargo test --test *
```
