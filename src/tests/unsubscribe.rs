use super::mock::*;
use crate::{Error, Subscription};
use frame_support::{assert_noop, assert_ok};

#[test]
fn unsubscribe() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Starting subscription
		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			ALICE(),
			BOB(),
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
			beneficiary: BOB(),
		};
		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;
		let index: u32 = 0;

		assert_ok!(PalletSubscription::unsubscribe(
			Origin::signed(ALICE()),
			when,
			index
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Unsubscription(subscription, ALICE()));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn unsubscribe_no_subscriptions_found() {
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
fn unsubscribe_invalid_when() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Starting subscription

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			ALICE(),
			BOB(),
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
			PalletSubscription::unsubscribe(Origin::signed(ALICE()), when, index),
			Error::<TestRuntime>::NoSubscriptionPlannedAtBlock
		);
	})
}

#[test]
fn unsubscribe_index_out_of_bounds() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Starting subscription

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			ALICE(),
			BOB(),
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
			PalletSubscription::unsubscribe(Origin::signed(ALICE()), when, index),
			Error::<TestRuntime>::IndexOutOfBounds
		);
	})
}

#[test]
fn unsubscribe_wrong_subscriber() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Starting subscription

		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			ALICE(),
			BOB(),
			amount,
			frequency,
			number_of_installment,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		let wrong_origin = Origin::signed(CHARLIE());

		let when = <frame_system::Pallet<TestRuntime>>::block_number() + 1;
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(wrong_origin, when, index),
			Error::<TestRuntime>::CallerIsNotSubscriber
		);
	})
}
