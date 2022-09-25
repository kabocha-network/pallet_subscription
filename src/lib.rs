#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

pub use pallet::*;

pub mod types;

pub use types::*;

#[cfg(test)]
mod tests;

pub use frame_support::{
	storage::IterableStorageMap,
	traits::tokens::{
		currency::{Currency, ReservableCurrency},
		ExistenceRequirement,
	},
	ReversibleStorageHasher,
};

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
		/// Weight information for extrinsics in this pallet.
		// type WeightInfo: WeightInfo;
		/// The maximum amount of metadata
		#[pallet::constant]
		type MaxMetadataLength: Get<u32>;
		/// The maximum weight that may be scheduled per block for any dispatchables of less
		/// priority than `schedule::HARD_DEADLINE`.
		#[pallet::constant]
		type MaximumWeight: Get<Weight>;
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
		Twox64Concat,
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
		Twox64Concat,
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
		Subscription(
			T::AccountId,
			T::AccountId,
			BalanceOf<T>,
			T::BlockNumber,
			Option<u32>,
		),
		Unsubscription(
			Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>,
			T::AccountId,
		),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidSubscription,
		IndexOutOfBounds,
		NoSubscriptionPlannedAtBlock,
		CallerIsNotSubscriber,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let limit = T::MaximumWeight::get();
			let mut total_weight: Weight = 0;

			let mut scheduled_subscriptions = match <Subscriptions<T>>::take(block_number) {
				Some(s) => s,
				None => return total_weight,
			};
			total_weight += T::DbWeight::get().reads_writes(1 as Weight, 1 as Weight);

			while total_weight < limit {
				let (sub_info, from) = match scheduled_subscriptions.pop() {
					Some(data) => data,
					None => break,
				};

				let _ = T::Currency::transfer(
					&from,
					&sub_info.beneficiary,
					sub_info.amount,
					ExistenceRequirement::KeepAlive,
				);
				// TODO: benchmark what cost a call to transfer and add it to total_weight
				// For now let's use this
				total_weight += T::DbWeight::get().reads_writes(1 as Weight, 1 as Weight);

				if let Some(1) = sub_info.remaining_payments {
					continue
				}

				match sub_info.remaining_payments {
					Some(remaining_payments) => {
						Self::schedule_subscriptions(
							block_number + sub_info.frequency,
							&[(
								Subscription {
									remaining_payments: Some(remaining_payments - 1),
									..sub_info
								},
								from,
							)],
						);
					},
					None => {
						Self::schedule_subscriptions(
							block_number + sub_info.frequency,
							&[(sub_info, from)],
						);
					},
				}

				total_weight += T::DbWeight::get().writes(1 as Weight);
			}

			if !scheduled_subscriptions.is_empty() {
				Self::schedule_subscriptions(
					block_number + T::BlockNumber::from(1u32),
					&scheduled_subscriptions,
				);
			}

			total_weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn subscribe(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			frequency: T::BlockNumber,
			number_of_installment: Option<u32>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			ensure!(
				!frequency.is_zero()
					&& !amount.is_zero() && match number_of_installment {
					Some(x) => x >= 1,
					None => true,
				} && to != from,
				Error::<T>::InvalidSubscription
			);

			let subscription = Subscription {
				frequency,
				amount,
				remaining_payments: number_of_installment,
				beneficiary: to.clone(),
			};

			let next_block_number = <frame_system::Pallet<T>>::block_number() + 1u32.into();

			Self::schedule_subscriptions(next_block_number, &[(subscription, from.clone())]);

			Self::deposit_event(Event::Subscription(
				from,
				to,
				amount,
				frequency,
				number_of_installment,
			));
			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn unsubscribe(
			origin: OriginFor<T>,
			when: T::BlockNumber,
			index: u32,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			<Subscriptions<T>>::mutate(when, |wrapped_current_subscriptions| {
				if let Some(current_subscriptions) = wrapped_current_subscriptions {
					let index = index as usize;

					if index >= current_subscriptions.len() {
						return Err(Error::<T>::IndexOutOfBounds)
					}

					let desired_subscription = &(current_subscriptions[index]);

					if desired_subscription.1 != from {
						return Err(Error::<T>::CallerIsNotSubscriber)
					}

					let subscription = desired_subscription.0.clone();

					current_subscriptions.remove(index);
					Self::deposit_event(Event::Unsubscription(subscription, from));
					Ok(())
				} else {
					Err(Error::<T>::NoSubscriptionPlannedAtBlock)
				}
			})?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn schedule_subscriptions(
			when: T::BlockNumber,
			new_subscription: &[(
				Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>,
				T::AccountId,
			)],
		) {
			<Subscriptions<T>>::mutate(when, |wrapped_current_subscriptions| {
				if let Some(current_subscriptions) = wrapped_current_subscriptions {
					current_subscriptions.extend_from_slice(new_subscription);
				} else {
					*wrapped_current_subscriptions = Some(new_subscription.to_vec());
				}
			});
		}
	}
}
