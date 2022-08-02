use super::mock::*;
use crate::Subscription;
use frame_support::assert_ok;

#[test]
fn unsubscribe() {
	ExternalityBuilder::build().execute_with(|| {
		// Starting subscription

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

		// ALICE has been subscribed to BOB.

		let remaining_payments = number_of_installment;
		let subscription = Subscription {
			frequency,
			amount,
			remaining_payments,
			beneficiary: BOB,
		};
		let index: u32 = 0;
		let next_block_number =
			<frame_system::Pallet<TestRuntime>>::block_number().saturating_add(1u32.into());

		assert_ok!(PalletSubscription::unsubscribe(
			Origin::signed(ALICE),
			ALICE,
			subscription.clone(),
			next_block_number,
			index
		));

		let expected_event =
			Event::PalletSubscription(crate::Event::Unsubscription(subscription.clone(), ALICE));
		let received_event = &System::events()[1].event;

		assert_eq!(*received_event, expected_event);
	})
}
