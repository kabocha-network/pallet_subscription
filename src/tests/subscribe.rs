use super::mock::*;
use crate::{Error, Event as SubscriptionEvent, InstalmentData};
use frame_support::{assert_noop, assert_ok};

#[test]
fn subscribe() {
	ExternalityBuilder::default().build().execute_with(|| {
		let number_of_instalments = Some(4);

		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			4000,
			5,
			number_of_instalments,
			vec![].try_into().unwrap(),
		));

		let plan_id = 0.into();

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			plan_id,
		));

		let expected_instalment = InstalmentData {
			subscription_id: plan_id.into(),
			remaining_payments: number_of_instalments,
			payer: ALICE(),
		};
		assert!(PalletSubscription::active_subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::Subscription(ALICE(), plan_id.into()));
		let received_event = &System::events()[1].event;
		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn subscribe_multiple_events_ok() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Subscription n1 - ALICE() BOB() BOB()

		let number_of_instalments = Some(4);

		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			4000,
			5,
			number_of_instalments,
			vec![].try_into().unwrap(),
		));

		let plan_id = 0.into();

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			plan_id,
		));

		let expected_instalment = InstalmentData {
			subscription_id: plan_id.into(),
			remaining_payments: number_of_instalments,
			payer: ALICE(),
		};
		assert!(PalletSubscription::active_subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::Subscription(ALICE(), plan_id.into()));
		let received_event = &System::events()[1].event;
		assert_eq!(*received_event, expected_event);

		// Subscription n2 - CHARLIE BOB() PAUL

		let number_of_instalments = Some(4);

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(CHARLIE()),
			plan_id,
		));

		let expected_instalment = InstalmentData {
			subscription_id: plan_id.into(),
			remaining_payments: number_of_instalments,
			payer: CHARLIE(),
		};
		assert!(PalletSubscription::active_subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::Subscription(CHARLIE(), plan_id.into()));
		let received_event = &System::events()[2].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn plan_does_not_exist() {
	ExternalityBuilder::default().build().execute_with(|| {
		assert_noop!(
			PalletSubscription::subscribe_to_plan(Origin::signed(ALICE()), 0.into(),),
			Error::<TestRuntime>::PlanDoesNotExist
		);
	})
}

#[test]
fn cannot_subscribe_to_self() {
	ExternalityBuilder::default().build().execute_with(|| {
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(ALICE()),
			4000,
			5,
			Some(4),
			vec![].try_into().unwrap(),
		));

		assert_noop!(
			PalletSubscription::subscribe_to_plan(Origin::signed(ALICE()), 0.into(),),
			Error::<TestRuntime>::CannotSubscribeToSelf,
		);
	})
}
