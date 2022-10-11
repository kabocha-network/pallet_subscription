use crate::{Error, Event as SubscriptionEvent};

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

		let plan_id = 0.into();

		let expected_event_opened =
			Event::PalletSubscription(SubscriptionEvent::PlanOpened(plan_id));
		let expected_event_closed =
			Event::PalletSubscription(SubscriptionEvent::PlanClosed(plan_id));

		assert!(!PalletSubscription::are_subscriptions_closed(plan_id));
		assert_ok!(PalletSubscription::open_plan(
			Origin::signed(ALICE()),
			plan_id
		));
		assert!(!PalletSubscription::are_subscriptions_closed(plan_id));
		let received_event = &System::events()[1].event;
		assert_eq!(*received_event, expected_event_opened);

		assert_ok!(PalletSubscription::close_plan(
			Origin::signed(ALICE()),
			plan_id
		));
		assert!(PalletSubscription::are_subscriptions_closed(plan_id));
		let received_event = &System::events()[2].event;
		assert_eq!(*received_event, expected_event_closed);

		assert_ok!(PalletSubscription::close_plan(
			Origin::signed(ALICE()),
			plan_id
		));
		assert!(PalletSubscription::are_subscriptions_closed(plan_id));
		let received_event = &System::events()[3].event;
		assert_eq!(*received_event, expected_event_closed);

		assert_ok!(PalletSubscription::open_plan(
			Origin::signed(ALICE()),
			plan_id
		));
		assert!(!PalletSubscription::are_subscriptions_closed(plan_id));
		let received_event = &System::events()[4].event;
		assert_eq!(*received_event, expected_event_opened);
	})
}

#[test]
fn plan_does_not_exist() {
	ExternalityBuilder::default().build().execute_with(|| {
		let plan_id = 0.into();

		assert_noop!(
			PalletSubscription::open_plan(Origin::signed(ALICE()), plan_id),
			Error::<TestRuntime>::PlanDoesNotExist
		);
		assert_noop!(
			PalletSubscription::close_plan(Origin::signed(ALICE()), plan_id),
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

		let plan_id = 0.into();

		assert_noop!(
			PalletSubscription::open_plan(Origin::signed(BOB()), plan_id),
			Error::<TestRuntime>::MustBeOwner
		);

		assert_noop!(
			PalletSubscription::close_plan(Origin::signed(BOB()), plan_id),
			Error::<TestRuntime>::MustBeOwner
		);
	})
}
