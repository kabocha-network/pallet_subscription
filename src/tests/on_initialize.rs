use super::mock::*;
use crate::InstalmentData;
use frame_support::assert_ok;
//
// TODO: add tests for triggering subscription to user

#[test]
fn trigger_hook_once_transfer_funds() {
	ExternalityBuilder::default().build().execute_with(|| {
		let alice_balance_before = Balances::free_balance(&ALICE());
		let bob_balance_before = Balances::free_balance(&BOB());

		let amount = 4000;

		// Starting subscription
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			amount,
			5,
			Some(4),
			vec![].try_into().unwrap(),
		));

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			0.into(),
		));

		// ALICE has been subscribed to BOB.

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
		let alice_balance_before = Balances::free_balance(&ALICE());
		let bob_balance_before = Balances::free_balance(&BOB());

		let amount = 4000;
		let frequency = 5;
		let number_of_instalments = None;

		// Starting subscription
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			amount,
			frequency,
			number_of_instalments,
			vec![].try_into().unwrap(),
		));

		let plan_id = 0.into();

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			plan_id,
		));

		// ALICE has been subscribed to BOB.

		run_to_block(2 + frequency * 100);
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount * 101
		);
		assert_eq!(
			Balances::free_balance(&BOB()),
			bob_balance_before + amount * 101
		);
		let subscriptions_to_come = PalletSubscription::active_subscriptions(2 + frequency * 101);
		assert!(subscriptions_to_come.contains(&InstalmentData {
			subscription_id: plan_id.into(),
			remaining_payments: number_of_instalments,
			payer: ALICE(),
		},))
	})
}

#[test]
fn caped_instalment_subscription() {
	ExternalityBuilder::default().build().execute_with(|| {
		let alice_balance_before = Balances::free_balance(&ALICE());
		let bob_balance_before = Balances::free_balance(&BOB());

		let amount = 4000;
		let frequency = 5;
		let number_of_instalments = Some(10);

		// Starting subscription
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			amount,
			frequency,
			number_of_instalments,
			vec![].try_into().unwrap(),
		));

		let plan_id = 0.into();

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			plan_id,
		));

		// ALICE has been subscribed to BOB.

		run_to_block(2);
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount
		);
		assert_eq!(Balances::free_balance(&BOB()), bob_balance_before + amount);
		let subscriptions_to_come = PalletSubscription::active_subscriptions(2 + frequency);
		assert!(subscriptions_to_come.contains(&InstalmentData {
			subscription_id: plan_id.into(),
			remaining_payments: Some(number_of_instalments.unwrap() - 1),
			payer: ALICE(),
		},));

		run_to_block(2 + frequency * (number_of_instalments.unwrap() as u64 - 1));
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount * 10
		);
		assert_eq!(
			Balances::free_balance(&BOB()),
			bob_balance_before + amount * 10
		);
		let subscriptions_to_come = PalletSubscription::active_subscriptions(
			2 + frequency * number_of_instalments.unwrap() as u64,
		);
		assert!(subscriptions_to_come.is_empty())
	})
}

#[test]
fn transfer_failed() {
	ExternalityBuilder::default().build().execute_with(|| {
		let alice_balance_before = Balances::free_balance(&ALICE());
		let bob_balance_before = Balances::free_balance(&BOB());

		let amount = alice_balance_before / 2 + 1;
		let frequency = 5;

		// Starting subscription
		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(BOB()),
			amount,
			frequency,
			None,
			vec![].try_into().unwrap(),
		));

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			0.into(),
		));

		// ALICE has been subscribed to BOB.

		// First execution work like a charm
		run_to_block(2);
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount
		);
		assert_eq!(Balances::free_balance(&BOB()), bob_balance_before + amount);

		// Balance did not change
		run_to_block(2 + frequency);
		assert_eq!(
			Balances::free_balance(&ALICE()),
			alice_balance_before - amount
		);
		assert_eq!(Balances::free_balance(&BOB()), bob_balance_before + amount);

		// Won't be executed anymore
		let subscriptions_to_come = PalletSubscription::active_subscriptions(2 + frequency * 10);
		assert!(subscriptions_to_come.is_empty())
	})
}
