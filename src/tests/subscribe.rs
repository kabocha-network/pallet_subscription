use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn subscribe() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(1),
			2,
			4000,
			5,
			Some(4)
		));

		assert_noop!(
			PalletSubscription::subscribe(Origin::signed(1), 2, 0, 5, Some(4)),
			Error::<TestRuntime>::InvalidSubscription
		);

		assert_noop!(
			PalletSubscription::subscribe(Origin::signed(1), 2, 4000, 0, Some(4)),
			Error::<TestRuntime>::InvalidSubscription
		);

		assert_noop!(
			PalletSubscription::subscribe(Origin::signed(1), 2, 0, 0, Some(4)),
			Error::<TestRuntime>::InvalidSubscription
		);

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(2, 1, 4000, 5));

		assert_eq!(System::events()[0].event, expected_event);
	})
}

#[test]
fn subscribe_multiple_events() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(1),
			2,
			4000,
			5,
			Some(4)
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(2, 1, 4000, 5));
		assert_eq!(System::events()[0].event, expected_event);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(7),
			10,
			6000,
			7,
			Some(4)
		));
		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(10, 7, 6000, 7));
		assert_eq!(System::events()[1].event, expected_event);

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(8),
			11,
			6001,
			8,
			Some(4)
		));
		let expected_event =
			Event::PalletSubscription(crate::Event::Subscription(11, 8, 6001, 8));
		assert_eq!(System::events()[2].event, expected_event);
	})
}

#[test]
fn subscribe_frequency_zero() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			PalletSubscription::subscribe(Origin::signed(1), 2, 400, 0, Some(4)),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}

#[test]
fn subscribe_amount_zero() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			PalletSubscription::subscribe(Origin::signed(1), 2, 0, 5, Some(4)),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}

#[test]
fn subscribe_amount_frequency_zero() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			PalletSubscription::subscribe(Origin::signed(1), 2, 0, 0, Some(4)),
			Error::<TestRuntime>::InvalidSubscription
		);
	})
}
