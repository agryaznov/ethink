//! Mocked rutnime for tests

use crate::{self as pallet_ethink, Config};
use ep_eth::AccountId20;
use ep_eth::EthereumSignature;
use frame_support::{
    derive_impl,
    dispatch::DispatchClass,
    parameter_types,
    traits::{ConstBool, Everything},
    weights::{
        Weight,
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee,
    },
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureSigned,
};
use pallet_contracts::Schedule;
use pallet_transaction_payment::CurrencyAdapter;
use sp_core::ConstU128;
use sp_core::{ConstU32, ConstU64, ConstU8, H256, U256};
use sp_runtime::traits::AccountIdLookup;
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::Verify;
use sp_runtime::{DispatchError, Perbill};

// Well-known accounts taken from Moonbeam
pub const ALITH: AccountId20 = AccountId20([
    242, 79, 243, 169, 207, 4, 199, 29, 188, 148, 208, 181, 102, 247, 162, 123, 148, 86, 108, 172,
]);
pub const BALTATHAR: AccountId20 = AccountId20([
    60, 208, 167, 5, 162, 220, 101, 229, 177, 225, 32, 88, 150, 186, 162, 190, 138, 7, 198, 224,
]);

/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
const MAXIMUM_BLOCK_WEIGHT: Weight =
    Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);
/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
// Prints debug output of the `contracts` pallet to stdout if the node is
// started with `-lruntime::contracts=debug`.
const CONTRACTS_DEBUG_OUTPUT: pallet_contracts::DebugInfo =
    pallet_contracts::DebugInfo::UnsafeDebug;
const CONTRACTS_EVENTS: pallet_contracts::CollectEvents =
    pallet_contracts::CollectEvents::UnsafeCollect;
// Unit = the base number of indivisible units for balances
const MILLIUNIT: Balance = 1_000_000_000;
pub const ED: Balance = MILLIUNIT;

/// An index to a block.
pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = EthereumSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
/// This is derived to ep_crypto::AccountId20 because of the used EthereumSignature above.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// Balance of an account.
pub type Balance = u128;

type Block = frame_system::mocking::MockBlock<Test>;
type EventRecord = frame_system::EventRecord<
    <Test as frame_system::Config>::RuntimeEvent,
    <Test as frame_system::Config>::Hash,
>;

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

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type Nonce = u64;
    type Hash = H256;
    type RuntimeCall = RuntimeCall;
    type AccountId = AccountId;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Block = Block;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<ED>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
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
    type WeightToFee = IdentityFee<u128>;
    type LengthToFee = IdentityFee<u128>;
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
    type UploadOrigin = EnsureSigned<Self::AccountId>;
    type InstantiateOrigin = EnsureSigned<Self::AccountId>;
    type ApiVersion = ();
    type Xcm = ();
}

impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Contracts = Contracts;
    type Call = RuntimeCall;
}

parameter_types! {
    pub MySchedule: Schedule<Test> = {
        let schedule = <Schedule<Test>>::default();
        schedule
    };
    pub static DepositPerByte: u64 = 1;
    pub const DepositPerItem: u64 = 2;
    pub const DefaultDepositLimit: u64 = 10_000_000_000;
    pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(30);
}

// Implement ethink! executor for Contracts
pallet_ethink::impl_executor!(Test, Contracts);
