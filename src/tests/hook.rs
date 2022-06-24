use super::mock::*;
use frame_support::traits::OnInitialize;

#[test]
fn working_subscription() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let _x = 10;
		let _bob_balance = Balances::free_balance(BOB());
		let _alice_balance = Balances::free_balance(ALICE());

		//TODO: make a subscription of x amount, that recur every 1 block, from ALICE() to BOB()
        //check subscription is stored in key BlockNumber 1

		<Subscription as OnInitialize<u64>>::on_initialize(1);

        //check subscription is not stored in key BlockNumber 1
        //check subscription is stored in key BlockNumber 2

		//assert_eq!(Balances::free_balance(BOB()), bob_balance + _x);
		//assert_eq!(Balances::free_balance(ALICE()), alice_balance - _x);
		//TODO: check if the subscription is still running (should be OK)
	})
}

#[test]
fn sub_last_occurence() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let _x = 10;
		let _bob_balance = Balances::free_balance(BOB());
		let _alice_balance = Balances::free_balance(ALICE());

		//TODO: make a subscription of x amount, that recur every 1 block, from ALICE() to BOB().
		// with remaining_payments = 1, this should only be executed once.

        //check subscription is not stored in key BlockNumber 1
        //check subscription is stored in key BlockNumber 2
		<Subscription as OnInitialize<u64>>::on_initialize(1);

        //check subscription is not stored in key BlockNumber 1
        //check subscription is stored in key BlockNumber 2

		//assert_eq!(Balances::free_balance(BOB()), bob_balance + _x);
		//assert_eq!(Balances::free_balance(ALICE()), alice_balance - _x);

		//TODO: check if the subscription is still running (should be OK)

		<Subscription as OnInitialize<u64>>::on_initialize(2);

        //check subscription is not stored in key BlockNumber 2
        //check subscription is not stored in key BlockNumber 3

		//assert_eq!(Balances::free_balance(BOB()), bob_balance + _x);
		//assert_eq!(Balances::free_balance(ALICE()), alice_balance - _x);

		//TODO: check if the subscription is still running (should be KO)
	})
}
