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
		let beneficiary = BOB;
		let subscriber = ALICE;
		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			beneficiary,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			beneficiary,
			subscriber,
			amount,
			frequency,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		let remaining_payments = number_of_installment;

		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary,
		};
		let index: u32 = 0;
		let when = <frame_system::Pallet<TestRuntime>>::block_number().saturating_add(1u32.into());

		assert_ok!(PalletSubscription::unsubscribe(
			origin,
			subscriber,
			subscription.clone(),
			when,
			index
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Unsubscription(subscription, subscriber));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn unsubscribe_no_subscriptions_found() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let frequency = 5;
		let amount = 4000;
		let remaining_payments = Some(4);

		let origin = Origin::signed(ALICE);
		let subscriber = ALICE;
		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary: BOB,
		};
		let when = 1000;
		let index: u32 = 0;

		assert_noop!(
			PalletSubscription::unsubscribe(origin, subscriber, subscription, when, index),
			Error::<TestRuntime>::UnknownUnsubscription
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
		let beneficiary = BOB;
		let subscriber = ALICE;
		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			beneficiary,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			beneficiary,
			subscriber,
			amount,
			frequency,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		let remaining_payments = number_of_installment;

		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary,
		};
		let index: u32 = 0;
		let when = 1000;

		assert_noop!(
			PalletSubscription::unsubscribe(origin, subscriber, subscription, when, index),
			Error::<TestRuntime>::UnknownUnsubscription
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
		let beneficiary = BOB;
		let subscriber = ALICE;
		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			beneficiary,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			beneficiary,
			subscriber,
			amount,
			frequency,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		let remaining_payments = number_of_installment;

		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary,
		};
		let index: u32 = 1000;
		let when = <frame_system::Pallet<TestRuntime>>::block_number().saturating_add(1u32.into());

		assert_noop!(
			PalletSubscription::unsubscribe(origin, subscriber, subscription, when, index),
			Error::<TestRuntime>::InvalidUnsubscription
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
		let beneficiary = BOB;
		let subscriber = ALICE;
		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			beneficiary,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			beneficiary,
			subscriber,
			amount,
			frequency,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		// Starting subscription n2

		const ALEX: u64 = 3;

		let second_beneficiary = ALEX;
		let second_amount = 5000;
		let second_frequency = 2;
		let second_number_of_installment = Some(3);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			second_beneficiary,
			second_amount,
			second_frequency,
			second_number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			second_beneficiary,
			subscriber,
			second_amount,
			second_frequency,
		));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to ALEX.

		let remaining_payments = number_of_installment;

		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary,
		};
		let index: u32 = 1;
		let when = <frame_system::Pallet<TestRuntime>>::block_number().saturating_add(1u32.into());

		// 'index' value is purposely wrong here.

		assert_noop!(
			PalletSubscription::unsubscribe(origin, subscriber, subscription, when, index),
			Error::<TestRuntime>::InvalidUnsubscription
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
		let beneficiary = BOB;
		let subscriber = ALICE;
		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin.clone(),
			beneficiary,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			beneficiary,
			subscriber,
			amount,
			frequency,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// ALICE has been subscribed to BOB.

		const THOMAS: u64 = 100;
		let wrong_subscriber = THOMAS;

		let remaining_payments = number_of_installment;

		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary,
		};
		let index: u32 = 0;
		let when = <frame_system::Pallet<TestRuntime>>::block_number().saturating_add(1u32.into());

		assert_noop!(
			PalletSubscription::unsubscribe(origin, wrong_subscriber, subscription, when, index),
			Error::<TestRuntime>::InvalidUnsubscription
		);
	})
}
