use super::mock::*;
use crate::{Error, Event as SubscriptionEvent, InstalmentData};
use frame_support::{assert_noop, assert_ok};

#[test]
fn subscribe() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let number_of_instalment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_instalment
		));

		let expected_instalment = InstalmentData {
			frequency,
			amount,
			remaining_payments: number_of_instalment,
			beneficiary: BOB(),
			payer: ALICE(),
		};
		assert!(PalletSubscription::subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::Subscription(expected_instalment));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn subscribe_multiple_events() {
	ExternalityBuilder::default().build().execute_with(|| {
		// Subscription n1 - ALICE() BOB() BOB()

		let amount = 4000;
		let frequency = 5;
		let number_of_instalment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_instalment
		));

		let expected_instalment = InstalmentData {
			frequency,
			amount,
			remaining_payments: number_of_instalment,
			beneficiary: BOB(),
			payer: ALICE(),
		};
		assert!(PalletSubscription::subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::Subscription(expected_instalment));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// Subscription n2 - CHARLIE BOB() PAUL

		let amount = 6000;
		let frequency = 7;
		let number_of_instalment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(CHARLIE()),
			PAUL(),
			amount,
			frequency,
			number_of_instalment
		));

		let expected_instalment = InstalmentData {
			frequency,
			amount,
			remaining_payments: number_of_instalment,
			beneficiary: PAUL(),
			payer: CHARLIE(),
		};
		assert!(PalletSubscription::subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::Subscription(expected_instalment));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn subscribe_frequency_zero() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 400;
		let frequency = 0;
		let number_of_instalment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE()),
				BOB(),
				amount,
				frequency,
				number_of_instalment
			),
			Error::<TestRuntime>::InvalidFrequency
		);
	})
}

#[test]
fn subscribe_amount_zero() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 0;
		let frequency = 5;
		let number_of_instalment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE()),
				BOB(),
				amount,
				frequency,
				number_of_instalment
			),
			Error::<TestRuntime>::InvalidAmount
		);
	})
}

#[test]
fn subscribe_instalments_zero() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 400;
		let frequency = 10;
		let number_of_instalment = Some(0);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE()),
				BOB(),
				amount,
				frequency,
				number_of_instalment
			),
			Error::<TestRuntime>::InvalidNumberOfInstalment
		);
	})
}

#[test]
fn subscribe_number_of_installment_none() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 2000;
		let frequency = 4;
		let number_of_instalment = None;

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_instalment
		));

		let expected_instalment = InstalmentData {
			frequency,
			amount,
			remaining_payments: number_of_instalment,
			beneficiary: BOB(),
			payer: ALICE(),
		};
		assert!(PalletSubscription::subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::Subscription(expected_instalment));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);
	})
}
