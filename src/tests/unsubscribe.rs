use super::mock::*;
use crate::{Error, InstalmentData};
use frame_support::{assert_noop, assert_ok};

#[test]
fn unsubscribe() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Starting subscription
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

		// ALICE has been subscribed to BOB.

		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;
		let index: u32 = 0;
		let instalment_data = InstalmentData {
			subscription_id: plan_id.into(),
			remaining_payments: number_of_instalments,
			payer: ALICE(),
		};

		assert!(PalletSubscription::active_subscriptions(when).contains(&instalment_data));

		assert_ok!(PalletSubscription::unsubscribe(
			Origin::signed(ALICE()),
			when,
			index
		));

		assert!(!PalletSubscription::active_subscriptions(when).contains(&instalment_data));
		let expected_event =
			Event::PalletSubscription(crate::Event::Unsubscription(ALICE(), plan_id.into()));
		let received_event = &System::events()[2].event;
		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn no_subscription_planned_at_block() {
	ExternalityBuilder::default().build().execute_with(|| {
		let origin = Origin::signed(ALICE());
		let when = 1000;
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(origin, when, index),
			Error::<TestRuntime>::NoSubscriptionPlannedAtBlock
		);
	})
}

#[test]
fn index_out_of_bounds() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Starting subscription
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			4000,
			5,
			Some(4),
			vec![].try_into().unwrap(),
		));

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			0.into(),
		));

		// ALICE has been subscribed to BOB.

		let index: u32 = 1000;
		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;

		assert_noop!(
			PalletSubscription::unsubscribe(Origin::signed(ALICE()), when, index),
			Error::<TestRuntime>::IndexOutOfBounds
		);
	})
}

#[test]
fn callet_is_not_payer() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Starting subscription
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			4000,
			5,
			Some(4),
			vec![].try_into().unwrap(),
		));

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			0.into(),
		));

		let wrong_origin = Origin::signed(CHARLIE());
		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(wrong_origin, when, index),
			Error::<TestRuntime>::CallerIsNotPayer
		);
	})
}
