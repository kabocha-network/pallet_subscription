#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;
pub use types::*;

pub use frame_support::traits::tokens::currency::{Currency, ReservableCurrency};

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
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
	pub type PlanNonce<T> = StorageValue<_, PlanId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn subscription_plans)]
	pub type Plans<T: Config> = StorageMap<_, Blake2_256, PlanId, Subscription<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn subscription_nonce)]
	pub type SubscriptionNonce<T> = StorageValue<_, SubscriptionId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn subscriptions)]
	pub type Subscriptions<T: Config> =
		StorageMap<_, Blake2_256, SubscriptionId, Subscription<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
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
			//// prop 1:
			// - loop over all subscriptions (.iter_values())
			// - check if the subscription shoud be taken
			// - take it

			//// prop 2: (2nd key: )
			0
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1)]
		pub fn create_subsciption_plan(
			origin: OriginFor<T>,
			plan: Subscription<T>,
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
		pub fn delete_subsciption_plan(origin: OriginFor<T>, plan_id: Nonce) -> DispatchResult {
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
		pub fn subscribe_to_plan(origin: OriginFor<T>, plan_id: Nonce) -> DispatchResult {
			// todo:
			// signed ?
            // plan exist ?
            //
			// -> create entry in storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn subscribe(origin: OriginFor<T>, plan: Subscription) -> DispatchResult {
			// todo:
			// signed ?
            // valid subscription ?
            //
			// -> create entry in storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn unsubscribe(origin: OriginFor<T>, other: T::AccountId) -> DispatchResult {
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
