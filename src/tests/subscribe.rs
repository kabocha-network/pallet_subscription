use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn subscribe() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;
		let first_amount = 4000;
		let first_frequency = 5;
		let first_number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE),
			BOB,
			first_amount,
			first_frequency,
			first_number_of_installment
		));

		let amount = 0;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE),
				BOB,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);

		let amount = 4000;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE),
				BOB,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);

		let amount = 0;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE),
				BOB,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);

		let expected_event = Event::PalletSubscription(crate::Event::Subscription(
			BOB,
			ALICE,
			first_amount,
			first_frequency,
		));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn subscribe_multiple_events() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;
		let amount = 4000;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE),
			BOB,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(BOB, ALICE, amount, frequency));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		const PAUL: u64 = 7;
		const JANE: u64 = 10;
		let amount = 6000;
		let frequency = 7;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(PAUL),
			JANE,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(JANE, PAUL, amount, frequency));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);

		const MIKE: u64 = 8;
		const ASHLEY: u64 = 11;
		let amount = 6001;
		let frequency = 8;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(MIKE),
			ASHLEY,
			amount,
			frequency,
			number_of_installment
		));
		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(ASHLEY, MIKE, amount, frequency));
		let received_event = &System::events()[2].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn subscribe_frequency_zero() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;
		let amount = 400;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE),
				BOB,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}

#[test]
fn subscribe_amount_zero() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;
		let amount = 0;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE),
				BOB,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}

#[test]
fn subscribe_amount_frequency_zero() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;
		let amount = 0;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				Origin::signed(ALICE),
				BOB,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}
