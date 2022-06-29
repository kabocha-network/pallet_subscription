use crate::{self as pallet_subscription, Config };
use frame_support::{assert_noop, assert_ok, construct_runtime, parameter_types,
                    traits::Everything, traits::Currency};
use frame_system::ensure_signed;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		PalletSubscription: pallet_subscription::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl Config for TestRuntime {
    type Event = Event;
    type Currency = ();
    type MaxMetadataLength = ();
}

struct ExternalityBuilder;

impl ExternalityBuilder {
    pub fn build() -> TestExternalities {
        let storage = frame_system::GenesisConfig::default()
            .build_storage::<TestRuntime>()
            .unwrap();
        let mut ext = TestExternalities::from(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}


 #[test]
fn subscribe() {
     ExternalityBuilder::build().execute_with(|| {
         assert_ok!(PalletSubscription::subscribe(Origin::signed(1), 2, 4000, 5, Some(4)));

         assert_noop!(PalletSubscription::subscribe(Origin::signed(1), 2, 0, 5, Some(4)), pallet_subscription::Error::<TestRuntime>::InvalidSubscription);

         assert_noop!(PalletSubscription::subscribe(Origin::signed(1), 2, 4000, 0 ,Some(4)), pallet_subscription::Error::<TestRuntime>::InvalidSubscription);

         assert_noop!(PalletSubscription::subscribe(Origin::signed(1), 2 ,0, 0, Some(4)),  pallet_subscription::Error::<TestRuntime>::InvalidSubscription);

         let expected_event = Event::PalletSubscription(pallet_subscription::Event::SubscriptionCreated(2, 1,4000, 5));

         assert_eq!(System::events()[0].event, expected_event);
     })
}

 #[test]
fn subscribe_multiple_events(){
    ExternalityBuilder::build().execute_with(|| {

        assert_ok!(PalletSubscription::subscribe(Origin::signed(1),2,4000,5, Some(4)));

        let expected_event = Event::PalletSubscription(pallet_subscription::Event::SubscriptionCreated(2,1, 4000, 5));
        assert_eq!(System::events()[0].event, expected_event);

        assert_ok!(PalletSubscription::subscribe(Origin::signed(7),10,6000,7, Some(4)));
        let expected_event = Event::PalletSubscription(pallet_subscription::Event::SubscriptionCreated (10, 7,6000, 7));
        assert_eq!(System::events()[1].event, expected_event);

        assert_ok!(PalletSubscription::subscribe(Origin::signed(8),11,6001,8, Some(4)));
        let expected_event = Event::PalletSubscription(pallet_subscription::Event::SubscriptionCreated (11, 8,6001, 8));
        assert_eq!(System::events()[2].event, expected_event);

    })
}


#[test]
fn subscribe_frequency_zero(){
    ExternalityBuilder::build().execute_with(|| {

        assert_noop!(PalletSubscription::subscribe(Origin::signed(1),2,400,0,Some(4)), pallet_subscription::Error::<TestRuntime>::InvalidSubscription);

    })
}

#[test]
fn subscribe_amount_zero(){
    ExternalityBuilder::build().execute_with(|| {

        assert_noop!(PalletSubscription::subscribe(Origin::signed(1),2,0,5,Some(4)), pallet_subscription::Error::<TestRuntime>::InvalidSubscription);

    })
}

#[test]
fn subscribe_amount_frequency_zero(){
    ExternalityBuilder::build().execute_with(|| {

        assert_noop!(PalletSubscription::subscribe(Origin::signed(1),2,0,0,Some(4)), pallet_subscription::Error::<TestRuntime>::InvalidSubscription);

    })
}
