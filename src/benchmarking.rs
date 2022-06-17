//! Benchmarking setup for pallet-benchmark
#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet;
use frame_benchmarking::{account as benchmark_account, benchmarks};
use frame_support::{assert_ok, storage::bounded_vec::*, traits::Get};
use frame_system::RawOrigin;
// use sp_std::vec;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

benchmarks! {
	create_subscription_plan {
		let alice: T::AccountId = get_account::<T>("ALICE");

		let subscription: SubscriptionPart<<T as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500u32.into(),
				remaining_payments: None,
				beneficiary: alice.clone(),
			};
	}: _(RawOrigin::Signed(alice), subscription.clone())
	verify {
		assert_eq!(Pallet::<T>::plan_nonce(), 1);
		assert_eq!(Pallet::<T>::plans(0), Some(subscription));
	}
	delete_subscription_plan {
		let alice: T::AccountId = get_account::<T>("ALICE");
		let subscription: SubscriptionPart<<T as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500u32.into(),
				remaining_payments: None,
				beneficiary: alice.clone(),
			};
		assert_ok!(Pallet::<T>::create_subscription_plan(
			RawOrigin::Signed(alice.clone()).into(),
			subscription.clone()
		));
	}: _(RawOrigin::Signed(alice), 0)
	verify {
		assert_eq!(Pallet::<T>::plan_nonce(), 1);
		assert_eq!(Pallet::<T>::plans(0), None);
	}
	subscribe_to_plan {
		let alice: T::AccountId = get_account::<T>("ALICE");
		let bob: T::AccountId = get_account::<T>("BOB");
		let subscription: SubscriptionPart<<T as frame_system::Config>::BlockNumber, _, _> =
			SubscriptionPart {
				frequency: 1u32.into(),
				amount: 500u32.into(),
				remaining_payments: None,
				beneficiary: alice.clone(),
			};
		assert_ok!(Pallet::<T>::create_subscription_plan(
			RawOrigin::Signed(alice.clone()).into(),
			subscription.clone()
		));
	}: _(RawOrigin::Signed(bob), 0)
}
