use super::mock::*;
use crate::{Error, Subscription};
use frame_support::{assert_noop, assert_ok};

#[test]
fn unsubscribe() {
	ExternalityBuilder::build().execute_with(|| {
		// Starting subscription

		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let from = ALICE;
		let to = BOB;

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			from,
			to,
			amount,
			frequency,
			number_of_installment,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		let remaining_payments = number_of_installment;

		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary: to,
		};
		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;
		let index: u32 = 0;

		assert_ok!(PalletSubscription::unsubscribe(origin, when, index));

		let expected_event =
			Event::PalletSubscription(crate::Event::Unsubscription(subscription, from));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn unsubscribe_no_subscriptions_found() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);

		let when = 1000;
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(origin, when, index),
			Error::<TestRuntime>::NoSubscriptionPlannedAtBlock
		);
	})
}

#[test]
fn unsubscribe_invalid_when() {
	ExternalityBuilder::build().execute_with(|| {
		// Starting subscription

		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let from = ALICE;
		let to = BOB;

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			from,
			to,
			amount,
			frequency,
			number_of_installment,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		let when = 1000;
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(origin, when, index),
			Error::<TestRuntime>::NoSubscriptionPlannedAtBlock
		);
	})
}

#[test]
fn unsubscribe_index_out_of_bounds() {
	ExternalityBuilder::build().execute_with(|| {
		// Starting subscription

		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let from = ALICE;
		let to = BOB;

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			from,
			to,
			amount,
			frequency,
			number_of_installment,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		let index: u32 = 1000;
		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;

		assert_noop!(
			PalletSubscription::unsubscribe(origin, when, index),
			Error::<TestRuntime>::IndexOutOfBounds
		);
	})
}

#[test]
fn unsubscribe_wrong_subscription_at_index() {
	ExternalityBuilder::build().execute_with(|| {
		// Starting subscription n1

		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let from = ALICE;
		let to = BOB;

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			from,
			to,
			amount,
			frequency,
			number_of_installment,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		// Starting subscription n2

		const ALEX: u64 = 3;

		let to = ALEX;

		let amount = 5000;
		let frequency = 2;
		let number_of_installment = Some(3);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			from,
			to,
			amount,
			frequency,
			number_of_installment,
		));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to ALEX.

		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;
		// 'index' value is purposely wrong here.
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(origin, when, index),
			Error::<TestRuntime>::SubscriptionAtIndexDoesNotMatch
		);
	})
}

#[test]
fn unsubscribe_wrong_subscriber() {
	ExternalityBuilder::build().execute_with(|| {
		// Starting subscription

		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let from = ALICE;
		let to = BOB;

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin,
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			from,
			to,
			amount,
			frequency,
			number_of_installment,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		const THOMAS: u64 = 100;

		let wrong_origin = Origin::signed(THOMAS);

		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(wrong_origin, when, index),
			Error::<TestRuntime>::CallerIsNotSubscriber
		);
	})
}
