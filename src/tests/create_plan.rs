use crate::{Error, Event as SubscriptionEvent, PlanData, PlanId};

use super::mock::*;
use frame_benchmarking::frame_support::{assert_noop, BoundedVec};
use frame_support::assert_ok;

#[test]
fn ok() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let number_of_instalments = None;
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(ALICE()),
			amount,
			frequency,
			number_of_instalments,
			metadata.clone()
		));

		let plan_id: PlanId = 0.into();
		let expected_plan_data = PlanData {
			frequency,
			amount,
			number_of_instalments,
			beneficiary: ALICE(),
		};

		assert_eq!(
			PalletSubscription::subscription_plan(plan_id).unwrap(),
			expected_plan_data,
		);
		assert_eq!(
			PalletSubscription::plan_metadata(plan_id).unwrap(),
			metadata
		);
		assert_eq!(PalletSubscription::plan_nonce(), 1);

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::PlanCreated(plan_id, expected_plan_data));
		let received_event = &System::events()[0].event;
		assert_eq!(*received_event, expected_event);
	})
}

#[test]
fn invalid_amount() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 0;
		let frequency = 5;
		let number_of_instalments = None;
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_noop!(
			PalletSubscription::create_plan(
				Origin::signed(ALICE()),
				amount,
				frequency,
				number_of_instalments,
				metadata
			),
			Error::<TestRuntime>::InvalidAmount
		);
	})
}

#[test]
fn invalid_frequency() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 0;
		let number_of_instalments = None;
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_noop!(
			PalletSubscription::create_plan(
				Origin::signed(ALICE()),
				amount,
				frequency,
				number_of_instalments,
				metadata
			),
			Error::<TestRuntime>::InvalidFrequency
		);
	})
}

#[test]
fn invalid_number_of_instalments() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let number_of_instalments = Some(0);
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_noop!(
			PalletSubscription::create_plan(
				Origin::signed(ALICE()),
				amount,
				frequency,
				number_of_instalments,
				metadata
			),
			Error::<TestRuntime>::InvalidNumberOfInstalment
		);
	})
}
