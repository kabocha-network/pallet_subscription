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
	use frame_support::{pallet_prelude::*, sp_runtime::traits::Zero, BoundedVec};
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
	#[pallet::getter(fn plans)]
	pub type Plans<T: Config> = StorageMap<
		_,
		Twox64Concat,
		PlanId,
		(
			PlanData<T::BlockNumber, BalanceOf<T>, T::AccountId>,
			BoundedVec<u8, T::MaxMetadataLength>,
		),
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn subscriptions)]
	pub type Subscriptions<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::BlockNumber,
		Vec<InstalmentData<T::BlockNumber, BalanceOf<T>, T::AccountId>>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Subscription(InstalmentData<T::BlockNumber, BalanceOf<T>, T::AccountId>),
		Unsubscription(InstalmentData<T::BlockNumber, BalanceOf<T>, T::AccountId>),
		PlanCreated(PlanId, PlanData<T::BlockNumber, BalanceOf<T>, T::AccountId>),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidAmount,
		InvalidFrequency,
		InvalidNumberOfInstalment,
		CannotSubscribeToSelf,
		IndexOutOfBounds,
		NoSubscriptionPlannedAtBlock,
		CallerIsNotSubscriber,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let limit = T::MaximumWeight::get();
			let mut total_weight: Weight = 0;

			let mut scheduled_subscriptions = <Subscriptions<T>>::take(block_number);
			total_weight += T::DbWeight::get().reads_writes(1 as Weight, 1 as Weight);

			while total_weight < limit {
				let sub_info = match scheduled_subscriptions.pop() {
					Some(data) => data,
					None => return total_weight,
				};

				let res_transfer = T::Currency::transfer(
					&sub_info.payer,
					&sub_info.beneficiary,
					sub_info.amount,
					ExistenceRequirement::KeepAlive,
				);
				// TODO: benchmark what costs a call to transfer and add it to total_weight
				// For now let's use this
				total_weight += T::DbWeight::get().reads_writes(1 as Weight, 1 as Weight);

				// Cases where we don't want to execute another instalment of this subscription
				if res_transfer.is_err() {
					continue
				}
				if let Some(1) = sub_info.remaining_payments {
					continue
				}

				match sub_info.remaining_payments {
					Some(remaining_payments) => {
						Self::schedule_subscriptions(
							block_number + sub_info.frequency,
							&[InstalmentData {
								remaining_payments: Some(remaining_payments - 1),
								..sub_info
							}],
						);
					},
					None => {
						Self::schedule_subscriptions(
							block_number + sub_info.frequency,
							&[sub_info],
						);
					},
				}
				total_weight += T::DbWeight::get().reads_writes(1 as Weight, 1 as Weight);
			}

			if !scheduled_subscriptions.is_empty() {
				Self::schedule_subscriptions(
					block_number + T::BlockNumber::from(1u32),
					&scheduled_subscriptions,
				);
				total_weight += T::DbWeight::get().reads_writes(1 as Weight, 1 as Weight);
			}

			total_weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn subscribe(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			frequency: T::BlockNumber,
			number_of_instalment: Option<u32>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			ensure!(!frequency.is_zero(), Error::<T>::InvalidFrequency);
			ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
			ensure!(
				match number_of_instalment {
					Some(x) => x >= 1,
					None => true,
				},
				Error::<T>::InvalidNumberOfInstalment
			);
			ensure!(to != from, Error::<T>::CannotSubscribeToSelf);

			let subscription = InstalmentData {
				frequency,
				amount,
				remaining_payments: number_of_instalment,
				beneficiary: to,
				payer: from,
			};

			let next_block_number = <frame_system::Pallet<T>>::block_number() + 1u32.into();

			Self::schedule_subscriptions(next_block_number, &[subscription.clone()]);

			Self::deposit_event(Event::Subscription(subscription));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn unsubscribe(
			origin: OriginFor<T>,
			when: T::BlockNumber,
			index: u32,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let mut instalments = Self::subscriptions(when);
			ensure!(
				!instalments.is_empty(),
				Error::<T>::NoSubscriptionPlannedAtBlock,
			);

			let index = index as usize;
			let length = instalments.len();

			ensure!(index < length, Error::<T>::IndexOutOfBounds,);

			let desired_subscription = &instalments[index];

			ensure!(
				desired_subscription.payer == from,
				Error::<T>::CallerIsNotSubscriber,
			);

			// Those two lines are safe because we checked index < length
			// Doing this rather than calling remove reduce complexity to 0(1)
			instalments.swap(index, length - 1);
			let subscription_data = instalments.pop().unwrap();

			<Subscriptions<T>>::insert(when, instalments);

			Self::deposit_event(Event::Unsubscription(subscription_data));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn create_plan(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			frequency: T::BlockNumber,
			number_of_instalment: Option<u32>,
			metadata: BoundedVec<u8, T::MaxMetadataLength>,
		) -> DispatchResult {
			let beneficiary = ensure_signed(origin)?;

			ensure!(!frequency.is_zero(), Error::<T>::InvalidFrequency);
			ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
			ensure!(
				match number_of_instalment {
					Some(x) => x >= 1,
					None => true,
				},
				Error::<T>::InvalidNumberOfInstalment
			);

			let id = Self::plan_nonce();

			<Plans<T>>::insert(
				id,
				(
					PlanData {
						frequency,
						amount,
						remaining_payments: number_of_instalment,
						beneficiary,
					},
					metadata,
				),
			);

			<PlanNonce<T>>::set(id + 1);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn schedule_subscriptions(
			when: T::BlockNumber,
			new_subscription: &[InstalmentData<T::BlockNumber, BalanceOf<T>, T::AccountId>],
		) {
			<Subscriptions<T>>::mutate(when, |current_subscriptions| {
				current_subscriptions.extend_from_slice(new_subscription);
			});
		}
	}
}
