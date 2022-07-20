#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![feature(is_some_with)]

pub use pallet::*;

pub mod types;

pub use types::*;

mod mock;
#[cfg(test)]
mod subscribe_call;

pub use frame_support::{
	storage::IterableStorageMap,
	traits::tokens::{
		currency::{Currency, ReservableCurrency},
		ExistenceRequirement,
	},
	ReversibleStorageHasher,
};

use sp_runtime::traits::Saturating;

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
	use frame_support::{pallet_prelude::*, sp_runtime::traits::Zero};
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

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn subscriptions)]
	pub type Subscriptions<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::BlockNumber,
		Vec<(
			Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>,
			T::AccountId,
		)>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Subscription has been created
		SubscriptionCreated(T::AccountId, T::AccountId, BalanceOf<T>, T::BlockNumber),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid subscription
		InvalidSubscription,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
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
		pub fn subscribe(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			frequency: T::BlockNumber,
			number_of_installment: Option<u32>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// check if subscription is valid
			ensure!(
				!frequency.is_zero()
					&& !amount.is_zero() && number_of_installment.is_some_and(|&x| x > 1)
					&& to != from,
				Error::<T>::InvalidSubscription
			);

			let subscription = Subscription {
				frequency,
				amount,
				remaining_payments: number_of_installment,
				beneficiary: to.clone(),
			};

			let new_subscription = (subscription, from.clone());

			let mut next_block_number = <frame_system::Pallet<T>>::block_number();
			next_block_number.saturating_inc();

			<Subscriptions<T>>::mutate(next_block_number, |wrapped_current_subscriptions| {
				if let Some(current_subscription) = wrapped_current_subscriptions {
					current_subscription.push(new_subscription);
				} else {
					*wrapped_current_subscriptions = Option::from(vec![new_subscription]);
				}
			});

			Self::deposit_event(Event::SubscriptionCreated(to, from, amount, frequency));
			Ok(())
		}
	}
}
