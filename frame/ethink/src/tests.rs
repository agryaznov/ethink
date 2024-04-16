type Block = frame_system::mocking::MockBlock<Test>;

use crate::{self as pallet_ethink, Config, Error, Pallet};
use ep_crypto::{AccountId20, EthereumSignature};
use ep_mapping::{SubstrateWeight, Weight};
use frame_support::{
    assert_err, assert_ok,
    dispatch::DispatchClass,
    parameter_types,
    traits::{ConstBool, Everything},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee,
    },
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    RawOrigin,
};
use pallet_contracts::Schedule;
use pallet_transaction_payment::CurrencyAdapter;
use sp_core::{ecdsa, ConstU32, ConstU64, ConstU8, Pair, H160, H256, U256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    ArithmeticError, BuildStorage, DispatchError, Perbill,
};

pub const EXISTENTIAL_DEPOSIT: u64 = 1_000;

pub type BlockNumber = u32;

/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
const MAXIMUM_BLOCK_WEIGHT: Weight =
    Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment,
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Randomness: pallet_insecure_randomness_collective_flip::{Pallet, Storage},
        Utility: pallet_utility::{Pallet, Call, Storage, Event},
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>, HoldReason},
        Ethink: pallet_ethink,
    }
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;

    // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
    //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
    // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
    // the lazy contract deletion.
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();

    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type Nonce = u64;
    type Hash = H256;
    type RuntimeCall = RuntimeCall;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId20;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}
impl pallet_insecure_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type MaxHolds = ConstU32<1>;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<1>;
    type WeightInfo = ();
}

impl pallet_utility::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = ();
}

impl pallet_transaction_payment::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<u64>;
    type LengthToFee = IdentityFee<u64>;
    type FeeMultiplierUpdate = ();
}

impl pallet_contracts::Config for Test {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallFilter = Everything;
    type DepositPerItem = DepositPerItem;
    type DepositPerByte = DepositPerByte;
    type CallStack = [pallet_contracts::Frame<Self>; 5];
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    type ChainExtension = ();
    type Schedule = MySchedule;
    type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
    type MaxCodeLen = ConstU32<{ 64 * 1024 }>;
    type DefaultDepositLimit = DefaultDepositLimit;
    type MaxStorageKeyLen = ConstU32<128>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
    type UnsafeUnstableInterface = ConstBool<true>;
    type RuntimeHoldReason = RuntimeHoldReason;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type Migrations = ();
    #[cfg(feature = "runtime-benchmarks")]
    type Migrations = pallet_contracts::migration::codegen::BenchMigrations;
    type MaxDelegateDependencies = ConstU32<32>;
    type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
    type Debug = ();
    type Environment = ();
}

impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Contracts = ContractsExecutor;
    type Call = RuntimeCall;
}

parameter_types! {
    pub MySchedule: Schedule<Test> = {
        let schedule = <Schedule<Test>>::default();
        schedule
    };
    pub static DepositPerByte: u64 = 1;
    pub const DepositPerItem: u64 = 2;
    pub const DefaultDepositLimit: u64 = 10_000_000;
    pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(30);
}

// Prints debug output of the `contracts` pallet to stdout if the node is
// started with `-lruntime::contracts=debug`.
const CONTRACTS_DEBUG_OUTPUT: pallet_contracts::DebugInfo =
    pallet_contracts::DebugInfo::UnsafeDebug;
const CONTRACTS_EVENTS: pallet_contracts::CollectEvents =
    pallet_contracts::CollectEvents::UnsafeCollect;

type EventRecord = frame_system::EventRecord<
    <Test as frame_system::Config>::RuntimeEvent,
    <Test as frame_system::Config>::Hash,
>;

// TODO this was jsut copied from runtime
pub struct ContractsExecutor;

use pallet_contracts_primitives::ContractExecResult;

impl pallet_ethink::Executor<RuntimeCall> for ContractsExecutor {
    type ExecResult = ContractExecResult<u64, EventRecord>;

    fn is_contract(who: H160) -> bool {
        // TODO This could possibly be optimized later with another method which uses
        // StorageMap::contains_key() instead of StorageMap::get() under the hood.
        Contracts::code_hash(&who.into()).is_some()
    }

    fn build_call(to: H160, value: U256, data: Vec<u8>, gas_limit: U256) -> Option<RuntimeCall> {
        let dest = to.into();
        let value = value.try_into().ok()?;
        let gas_limit = SubstrateWeight::from(gas_limit).into();

        Some(if Self::is_contract(to) {
            pallet_contracts::Call::<Test>::call {
                dest,
                value,
                data,
                gas_limit,
                storage_deposit_limit: None,
            }
            .into()
        } else {
            pallet_balances::Call::<Test>::transfer_allow_death { dest, value }.into()
        })
    }

    fn call(
        from: H160,
        to: H160,
        data: Vec<u8>,
        value: U256,
        gas_limit: U256,
    ) -> Result<Self::ExecResult, DispatchError> {
        let from = AccountId20::from(from);
        let to = AccountId20::from(to);
        // TODO this is not really a Dispatch error
        // TODO maybe it worth adding specific error types on arg. types conversion failures
        // Here we try to convert provided U256 into runtime Balance (which is usually u128 in Substrate)
        let value = value
            .try_into()
            .map_err(|_| DispatchError::Arithmetic(ArithmeticError::Overflow))?;

        let gas_limit = SubstrateWeight::from(gas_limit).into();

        Ok(Contracts::bare_call(
            from,
            to,
            value,
            gas_limit,
            None,
            data,
            CONTRACTS_DEBUG_OUTPUT,
            CONTRACTS_EVENTS,
            pallet_contracts::Determinism::Enforced,
        ))
    }
}

// Well-known accounts taken from Moonbeam
pub const ALITH: AccountId20 = AccountId20([
    242, 79, 243, 169, 207, 4, 199, 29, 188, 148, 208, 181, 102, 247, 162, 123, 148, 86, 108, 172,
]);
pub const ALITH_KEY: &'static str = env!("ALITH_KEY");
pub const BALTATHAR: AccountId20 = AccountId20([
    60, 208, 167, 5, 162, 220, 101, 229, 177, 225, 32, 88, 150, 186, 162, 190, 138, 7, 198, 224,
]);
pub const BALTATHAR_KEY: &'static str = env!("BALTATHAR_KEY");

pub mod test_utils {
    use crate::{
        tests::{AccountId20, Test},
        Config,
    };
    use frame_support::traits::fungible::Mutate;

    pub fn set_balance(who: &AccountId20, amount: u64) {
        let _ = <Test as Config>::Currency::set_balance(who, amount);
    }
    pub fn get_balance(who: &AccountId20) -> u64 {
        <Test as Config>::Currency::free_balance(who)
    }
}

#[derive(Default)]
pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut s = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut s)
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(s);
        ext.execute_with(|| {
            use frame_support::traits::OnGenesis;

            Pallet::<Test>::on_genesis();

            System::set_block_number(1);
        });

        ext
    }
}

// TODO put these to a seaprate crate to DRY with e2e tests
#[derive(Clone)]
pub struct ContractInput(Vec<u8>);

impl From<Vec<u8>> for ContractInput {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl Into<Vec<u8>> for ContractInput {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Clone)]
/// Ethereum transaction input, use for transaciton building in tests
pub struct EthTxInput {
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: SubstrateWeight,
    pub action: ethereum::TransactionAction,
    pub value: u64,
    pub data: ContractInput,
    pub chain_id: Option<u64>,
    pub signer: ecdsa::Pair,
}

impl Default for EthTxInput {
    fn default() -> Self {
        Self {
            nonce: 1u64,
            gas_price: 0u64,
            gas_limit: SubstrateWeight::from(Weight::MAX),
            action: ethereum::TransactionAction::Call(Default::default()),
            value: 0u64,
            data: vec![0].into(),
            chain_id: None,
            signer: ecdsa::Pair::generate().0,
        }
    }
}
use crate::{EthTransaction, LegacyTransactionMessage};
use ethereum::LegacyTransaction;

impl From<EthTxInput> for LegacyTransactionMessage {
    fn from(v: EthTxInput) -> Self {
        let nonce = v.nonce.into();
        let gas_price = v.gas_price.into();
        let gas_limit: U256 = v.gas_limit.into();
        let value = v.value.into();

        Self {
            nonce,
            gas_price,
            gas_limit,
            action: v.action,
            value,
            input: v.data.into(),
            chain_id: v.chain_id,
        }
    }
}

/// Build Eth tx message, sign it and build an Eth transaction
pub fn compose_and_sign_tx(i: EthTxInput) -> EthTransaction {
    let msg: LegacyTransactionMessage = i.clone().into();
    let sig = EthereumSignature::new(i.signer.sign_prehashed(&msg.hash().into()));
    let sig: Option<ethereum::TransactionSignature> = sig.into();
    let signature = sig.expect("signer generated no signature");

    EthTransaction::Legacy(LegacyTransaction {
        nonce: msg.nonce,
        gas_price: msg.gas_price,
        gas_limit: msg.gas_limit,
        action: msg.action,
        value: msg.value,
        input: msg.input,
        signature,
    })
}

#[test]
fn calling_user_account_transfers_balance() {
    ExtBuilder::default().build().execute_with(|| {
        let init_balance = 100_000_000;
        let transfer_balance = 20_000_000;

        let _ = test_utils::set_balance(&ALITH, init_balance);

        let input = EthTxInput {
            signer: ecdsa::Pair::from_string(ALITH_KEY, None).unwrap(),
            action: ethereum::TransactionAction::Call(BALTATHAR.into()),
            data: vec![].into(),
            value: transfer_balance,
            ..Default::default()
        };
        let eth_tx = compose_and_sign_tx(input);

        let origin = RuntimeOrigin::from(pallet_ethink::RawOrigin::EthTransaction(ALITH.into()));
        assert_ok!(Ethink::transact(origin, eth_tx));

        let alith_balance = test_utils::get_balance(&ALITH);
        let baltathar_balance = test_utils::get_balance(&BALTATHAR);

        assert_eq!(alith_balance, init_balance - transfer_balance);
        assert_eq!(baltathar_balance, transfer_balance);
    });
}
