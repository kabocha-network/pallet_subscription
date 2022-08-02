use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn subscribe() {
	ExternalityBuilder::build().execute_with(|| {
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

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(to, from, amount, frequency));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// Wrong subscribe (amount == 0)

		let amount = 0;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				origin.clone(),
				to,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);

		// Wrong subscribe (frequency == 0)

		let amount = 4000;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(
				origin.clone(),
				to,
				amount,
				frequency,
				number_of_installment
			),
			Error::<TestRuntime>::InvalidSubscription
		);

		// Wrong subscribe (amount == 0 & frequency == 0)

		let amount = 0;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(origin, to, amount, frequency, number_of_installment),
			Error::<TestRuntime>::InvalidSubscription
		);

		// Checking whether no new events were added.
		assert_eq!(System::events().len(), 1);
	})
}

#[test]
fn subscribe_multiple_events() {
	ExternalityBuilder::build().execute_with(|| {
		// Subscription n1 - ALICE to BOB

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

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(to, from, amount, frequency));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);

		// Subscription n2 - PAUL to JANE

		const PAUL: u64 = 3;
		const JANE: u64 = 4;

		let origin = Origin::signed(PAUL);
		let from = PAUL;
		let to = JANE;

		let amount = 6000;
		let frequency = 7;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin,
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(to, from, amount, frequency));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);

		// Subscription n3 - MIKE to ASHLEY

		const MIKE: u64 = 5;
		const ASHLEY: u64 = 6;

		let origin = Origin::signed(MIKE);
		let from = MIKE;
		let to = ASHLEY;

		let amount = 6001;
		let frequency = 8;
		let number_of_installment = Some(4);

		assert_ok!(PalletSubscription::subscribe(
			origin,
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(to, from, amount, frequency));
		let received_event = &System::events()[2].event;

		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn subscribe_frequency_zero() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let to = BOB;

		let amount = 400;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(origin, to, amount, frequency, number_of_installment),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}

#[test]
fn subscribe_amount_zero() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let to = BOB;

		let amount = 0;
		let frequency = 5;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(origin, to, amount, frequency, number_of_installment),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}

#[test]
fn subscribe_amount_frequency_zero() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let to = BOB;

		let amount = 0;
		let frequency = 0;
		let number_of_installment = Some(4);

		assert_noop!(
			PalletSubscription::subscribe(origin, to, amount, frequency, number_of_installment),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}

#[test]
fn subscribe_number_of_installment_none() {
	ExternalityBuilder::build().execute_with(|| {
		const ALICE: u64 = 1;
		const BOB: u64 = 2;

		let origin = Origin::signed(ALICE);
		let from = ALICE;
		let to = BOB;

		let amount = 2000;
		let frequency = 4;
		let number_of_installment = None;

		assert_ok!(PalletSubscription::subscribe(
			origin,
			to,
			amount,
			frequency,
			number_of_installment
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(to, from, amount, frequency));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);
	})
}
