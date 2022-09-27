use crate::Error;

use super::mock::*;
use frame_benchmarking::frame_support::assert_noop;
use frame_support::assert_ok;

#[test]
fn ok() {
	ExternalityBuilder::default().build().execute_with(|| {
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(ALICE()),
			4000,
			5,
			None,
			vec![].try_into().unwrap(),
		));

		assert_ok!(PalletSubscription::delete_plan(Origin::signed(ALICE()), 0));
		assert_eq!(PalletSubscription::plans(0), None);
	})
}

#[test]
fn plan_does_not_exist() {
	ExternalityBuilder::default().build().execute_with(|| {
		assert_noop!(
			PalletSubscription::delete_plan(Origin::signed(ALICE()), 0),
			Error::<TestRuntime>::PlanDoesNotExist
		);
	})
}

#[test]
fn must_be_owner() {
	ExternalityBuilder::default().build().execute_with(|| {
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(ALICE()),
			4000,
			5,
			None,
			vec![].try_into().unwrap(),
		));

		assert_noop!(
			PalletSubscription::delete_plan(Origin::signed(BOB()), 0),
			Error::<TestRuntime>::MustBeOwner
		);
	})
}
