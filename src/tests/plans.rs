use super::mock::*;
use crate::{Error, SubscriptionPart};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_plan() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let subscription: SubscriptionPart<<Test as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500,
				remaining_payments: None,
				beneficiary: ALICE(),
			};

		assert_ok!(Subscription::create_subscription_plan(
			Origin::signed(ALICE()),
			subscription.clone()
		));
		assert_eq!(Subscription::plan_nonce(), 1);
		assert_eq!(Subscription::plans(0), Some(subscription));
	});
}

#[test]
fn create_plan_for_someone_else() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let subscription: SubscriptionPart<<Test as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500,
				remaining_payments: None,
				beneficiary: BOB(),
			};

		assert_noop!(
			Subscription::create_subscription_plan(Origin::signed(ALICE()), subscription.clone()),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn create_invalid_plan() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let subscription: SubscriptionPart<<Test as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 0u32.into(),
				amount: 500,
				remaining_payments: None,
				beneficiary: ALICE(),
			};

		assert_noop!(
			Subscription::create_subscription_plan(Origin::signed(ALICE()), subscription),
			Error::<Test>::BadFrequency
		);
	});
}

#[test]
fn delete_plan() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let subscription: SubscriptionPart<<Test as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500,
				remaining_payments: None,
				beneficiary: ALICE(),
			};

		assert_ok!(Subscription::create_subscription_plan(
			Origin::signed(ALICE()),
			subscription.clone()
		));
		assert_eq!(Subscription::plan_nonce(), 1);
		assert_eq!(Subscription::plans(0), Some(subscription));
		assert_ok!(Subscription::delete_subscription_plan(
			Origin::signed(ALICE()),
			0
		));
		assert_eq!(Subscription::plan_nonce(), 1);
		assert_eq!(Subscription::plans(0), None);
	});
}

#[test]
fn delete_unknown_plan() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let subscription: SubscriptionPart<<Test as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500,
				remaining_payments: None,
				beneficiary: ALICE(),
			};

		assert_ok!(Subscription::create_subscription_plan(
			Origin::signed(ALICE()),
			subscription.clone()
		));
		assert_noop!(
			Subscription::delete_subscription_plan(Origin::signed(ALICE()), 1),
			Error::<Test>::UnknownPlan
		);
	});
}

#[test]
fn subscribe_to_plan() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let subscription: SubscriptionPart<<Test as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500,
				remaining_payments: None,
				beneficiary: ALICE(),
			};

		assert_ok!(Subscription::create_subscription_plan(
			Origin::signed(ALICE()),
			subscription.clone()
		));
		assert_ok!(Subscription::subscribe_to_plan(Origin::signed(BOB()), 0,));
		// assert that a subscription is running
	});
}

#[test]
fn subscribe_to_unknown_plan() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		let subscription: SubscriptionPart<<Test as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500,
				remaining_payments: None,
				beneficiary: ALICE(),
			};

		assert_ok!(Subscription::create_subscription_plan(
			Origin::signed(ALICE()),
			subscription.clone()
		));
		assert_noop!(
			Subscription::subscribe_to_plan(Origin::signed(BOB()), 1),
			Error::<Test>::UnknownPlan
		);
	});
}
