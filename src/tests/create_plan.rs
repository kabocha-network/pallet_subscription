use crate::{Error, PlanData};

use super::mock::*;
use frame_benchmarking::frame_support::{assert_noop, BoundedVec};
use frame_support::assert_ok;

#[test]
fn ok() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let recurence = None;
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_ok!(PalletSubscription::create_plan(
			Origin::signed(ALICE()),
			amount,
			frequency,
			recurence,
			metadata.clone()
		));

		assert_eq!(
			PalletSubscription::plans(0).unwrap(),
			(
				PlanData {
					frequency,
					amount,
					remaining_payments: recurence,
					beneficiary: ALICE(),
				},
				metadata
			)
		);
		assert_eq!(PalletSubscription::plan_nonce(), 1);
	})
}

#[test]
fn invalid_amount() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 0;
		let frequency = 5;
		let recurence = None;
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_noop!(
			PalletSubscription::create_plan(
				Origin::signed(ALICE()),
				amount,
				frequency,
				recurence,
				metadata.clone()
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
		let recurence = None;
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_noop!(
			PalletSubscription::create_plan(
				Origin::signed(ALICE()),
				amount,
				frequency,
				recurence,
				metadata.clone()
			),
			Error::<TestRuntime>::InvalidFrequency
		);
	})
}

#[test]
fn invalid_number_of_instalment() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 4000;
		let frequency = 5;
		let recurence = Some(0);
		let metadata: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_noop!(
			PalletSubscription::create_plan(
				Origin::signed(ALICE()),
				amount,
				frequency,
				recurence,
				metadata.clone()
			),
			Error::<TestRuntime>::InvalidNumberOfInstalment
		);
	})
}
