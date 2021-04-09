use super::*;
use crate as pallet_webb_tokens;
use frame_benchmarking::whitelisted_caller;
use frame_support::{construct_runtime, parameter_types, traits::GenesisBuild, weights::Weight};
use frame_system::mocking::{MockBlock, MockUncheckedExtrinsic};
use merkle::weights::Weights as MerkleWeights;
use orml_currencies::BasicCurrencyAdapter;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	ModuleId, Perbill,
};
use weights::Weights;

pub(crate) type Balance = u64;
pub type Amount = i128;
pub type CurrencyId = u64;
pub type AccountId = u64;
pub type BlockNumber = u64;

// Configure a mock runtime to test the pallet.
type UncheckedExtrinsic = MockUncheckedExtrinsic<Test>;
type Block = MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
		MerkleTrees: merkle::{Module, Call, Storage, Event<T>},
		WebbTokens: pallet_webb_tokens::{Module, Call, Storage, Event<T>},
		Currencies: orml_currencies::{Module, Storage, Event<T>},
		Tokens: orml_tokens::{Module, Storage, Event<T>, Config<T>},
	}
);

parameter_types! {
	pub Prefix: u8 = 100;
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Config for Test {
	type AccountData = balances::AccountData<u64>;
	type AccountId = AccountId;
	type BaseCallFilter = ();
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = BlockNumber;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = Prefix;
	type SystemWeightInfo = ();
	type Version = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
	pub const MaxLocks: u32 = 50;
	pub const MaxTreeDepth: u8 = 32;
	pub const CacheBlockLength: u64 = 5;
	// Minimum deposit length is 1 month w/ 6 second blocks
	pub const MinimumDepositLength: u64 = 10 * 60 * 24 * 28;
}

impl balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type WeightInfo = ();
}

parameter_type_with_key! {
	pub ExistentialDepositMap: |k: CurrencyId| -> Balance {
		match k {
			_ => 2,
		}
	};
}

parameter_types! {
	pub const NativeCurrencyId: CurrencyId = 0;
}

impl orml_tokens::Config for Test {
	type Amount = Amount;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type Event = Event;
	type ExistentialDeposits = ExistentialDepositMap;
	type OnDust = ();
	type WeightInfo = ();
}

impl orml_currencies::Config for Test {
	type Event = Event;
	type GetNativeCurrencyId = NativeCurrencyId;
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;
	type WeightInfo = ();
}

parameter_types! {
	pub const CurrencyDeposit: u64 = 1;
	pub const ApprovalDeposit: u64 = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: u64 = 1;
	pub const MetadataDepositPerByte: u64 = 1;
}

impl Config for Test {
	type Event = Event;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type Currency = Currencies;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type CurrencyDeposit = CurrencyDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type WeightInfo = ();
	type Extra = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	use balances::GenesisConfig as BalancesConfig;
	use orml_tokens::GenesisConfig as TokensConfig;
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	BalancesConfig::<Test> {
		// Total issuance will be 200 with treasury account initialized at ED.
		balances: vec![
			(0, 1_000_000_000),
			(1, 1_000_000_000),
			(2, 1_000_000_000),
			(whitelisted_caller(), 1_000_000_000),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let token_currency_id: CurrencyId = 1;
	TokensConfig::<Test> {
		endowed_accounts: vec![(0, token_currency_id, 1_000_000_000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}