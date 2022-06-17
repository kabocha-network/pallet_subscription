#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

pub use pallet::*;

pub mod types;
pub use types::*;

pub use frame_support::{
	storage::IterableStorageMap,
	traits::tokens::{
		currency::{Currency, ReservableCurrency},
		ExistenceRequirement,
	},
	ReversibleStorageHasher,
};

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The trait to manage funds
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		/// The maximum amount of metadata
		#[pallet::constant]
		type MaxMetadataLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn plan_nonce)]
	pub type PlanNonce<T: Config> = StorageValue<_, PlanId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn subscription_plans)]
	pub type Plans<T: Config> = StorageMap<
		_,
		Blake2_256,
		PlanId,
		Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn subscription_nonce)]
	pub type SubscriptionNonce<T: Config> = StorageValue<_, SubscriptionId, ValueQuery>;

	// the vector should ALWAYS be sorted from biggest to smallest block_number
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn subscriptions)]
	pub type Subscriptions<T: Config> = StorageValue<
		_,
		Vec<(
			T::BlockNumber,
			(
				Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>,
				T::AccountId,
			),
		)>,
	>;

	#[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			let _weight = 0;

			let mut subs = unwrap_or_return!(Subscriptions::<T>::get(), 0); // 1 read
			let tmp = subs.clone();
			let (exec_block, (_, _)) = unwrap_or_return!(tmp.last(), 0); // 1 unwrap
			if n != *exec_block {
				return 0
			}
			loop {
				if let Some((b, (mut sub, account))) = subs.pop() {
					if b == *exec_block {
						if let Some(val) = sub.remaining_payments {
							if val == 0 {
								subs.pop();
								continue
							}
						}
						if T::Currency::transfer(
							&account,
							&sub.beneficiary,
							sub.amount,
							ExistenceRequirement::AllowDeath,
						)
						.is_err()
						{
							continue
						}
						sub.remaining_payments = sub.remaining_payments.map(|amount| amount - 1);
						let new_block = n + sub.frequency;
						let new_index =
							subs.binary_search_by(|(b, _)| b.cmp(&new_block)).unwrap_or_else(|e| e);
						subs.insert(new_index, (new_block, (sub.clone(), account.clone())));
					}
				}
				if (unwrap_or_return!(subs.last(), 0)).0 == *exec_block {
					Subscriptions::<T>::set(Some(subs));
					return 0 // WEIGHT
				}
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1)]
		pub fn create_subsciption_plan(
			_origin: OriginFor<T>,
			_plan: Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>,
		) -> DispatchResult {
			// maybe make a new function for Subscription, so `plan` is already valid 100%
			//
			//
			// todo:
			// signed ?
			// valid plan ?
			//
			// -> push the plan to the storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn delete_subsciption_plan(_origin: OriginFor<T>, _plan_id: Nonce) -> DispatchResult {
			// better ? 5 parameters but we can check, better option if no plan.new() function
			//
			//
			// todo:
			// signed ?
			// plan exist ?
			//
			// -> remove the plan to the storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn subscribe_to_plan(_origin: OriginFor<T>, _plan_id: Nonce) -> DispatchResult {
			// todo:
			// signed ?
			// plan exist ?
			//
			// -> create entry in storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn subscribe(
			_origin: OriginFor<T>,
			_plan: Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>,
		) -> DispatchResult {
			// todo:
			// signed ?
			// valid subscription ?
			//
			// -> create entry in storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn unsubscribe(_origin: OriginFor<T>, _other: T::AccountId) -> DispatchResult {
			// signed by the subscriber ?
			// yes -> subscriber is subscribed to other ?
			//        delete from storage
			// no -> signed by beneficiary ?
			//       other is subscribed to beneficiary ?
			//       delete from storage
			Ok(())
		}
	}
}
