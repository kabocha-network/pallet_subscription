use super::mock::*;
use crate::{Error, Subscription};
use frame_support::{assert_noop, assert_ok};

#[test]
fn trigger_hook_once_transfer_funds() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let beneficiary = BOB();
		let recurence = None;

		let alice_balance_before = Balances::free_balance(&ALICE());
		let bob_balance_before = Balances::free_balance(&BOB());

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			beneficiary.clone(),
			amount,
			frequency,
			recurence,
		));

		// Will be executed for the first time at block 2
		run_to_block(2);
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount
		);
		assert_eq!(Balances::free_balance(&BOB()), bob_balance_before + amount);
	})
}

#[test]
fn infinite_subscription() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let beneficiary = BOB();
		let recurence = None;

		let alice_balance_before = Balances::free_balance(&ALICE());
		let bob_balance_before = Balances::free_balance(&BOB());

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			beneficiary.clone(),
			amount,
			frequency,
			recurence,
		));

		run_to_block(2 + frequency * 100);
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount * 101
		);
		assert_eq!(
			Balances::free_balance(&BOB()),
			bob_balance_before + amount * 101
		);
		let subscriptions_to_come = PalletSubscription::subscriptions(2 + frequency * 101);
		assert!(
			subscriptions_to_come
				.expect("Should contain the reinserted subscription")
				.contains(&(
					Subscription {
						frequency,
						amount,
						remaining_payments: recurence,
						beneficiary,
					},
					ALICE()
				))
		)
	})
}

#[test]
fn caped_instalment_subscription() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let beneficiary = BOB();
		let recurence = Some(10);

		let alice_balance_before = Balances::free_balance(&ALICE());
		let bob_balance_before = Balances::free_balance(&BOB());

		assert_ok!(PalletSubscription::subscribe(
			Origin::signed(ALICE()),
			beneficiary.clone(),
			amount,
			frequency,
			recurence,
		));

		run_to_block(2);
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount
		);
		assert_eq!(Balances::free_balance(&BOB()), bob_balance_before + amount);
		let subscriptions_to_come = PalletSubscription::subscriptions(2 + frequency);
		assert!(
			subscriptions_to_come
				.expect("Should contain the reinserted subscription")
				.contains(&(
					Subscription {
						frequency,
						amount,
						remaining_payments: Some(recurence.unwrap() - 1),
						beneficiary,
					},
					ALICE()
				))
		);

		run_to_block(2 + frequency * (recurence.unwrap() as u64 - 1));
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount * 10
		);
		assert_eq!(
			Balances::free_balance(&BOB()),
			bob_balance_before + amount * 10
		);
		let subscriptions_to_come = PalletSubscription::subscriptions(2 + frequency * 10);
		assert!(subscriptions_to_come.is_none())
	})
}
