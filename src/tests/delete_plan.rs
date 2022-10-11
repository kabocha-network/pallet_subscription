use crate::{Error, Event as SubscriptionEvent, PlanId};

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

		let plan_id: PlanId = 0.into();

		assert_ok!(PalletSubscription::delete_plan(
			Origin::signed(ALICE()),
			plan_id
		));
		assert_eq!(PalletSubscription::subscription_plan(plan_id), None);

		let expected_event = Event::PalletSubscription(SubscriptionEvent::PlanDeleted(plan_id));
		let received_event = &System::events()[1].event;
		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn plan_does_not_exist() {
	ExternalityBuilder::default().build().execute_with(|| {
		assert_noop!(
			PalletSubscription::delete_plan(Origin::signed(ALICE()), 0.into()),
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
			PalletSubscription::delete_plan(Origin::signed(BOB()), 0.into()),
			Error::<TestRuntime>::MustBeOwner
		);
	})
}
