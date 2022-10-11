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
	pub type PlanNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn subscription_to_user_nonce)]
	pub type SubscriptionToUserNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn subscription_plan)]
	pub type SubscriptionPlan<T: Config> = StorageMap<
		_,
		Twox64Concat,
		PlanId,
		PlanData<T::BlockNumber, BalanceOf<T>, T::AccountId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn subscription_to_user)]
	pub type SubscriptionToUser<T: Config> = StorageMap<
		_,
		Twox64Concat,
		SubscriptionToUserId,
		PlanData<T::BlockNumber, BalanceOf<T>, T::AccountId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn plan_metadata)]
	pub type PlanMetadata<T: Config> =
		StorageMap<_, Twox64Concat, PlanId, BoundedVec<u8, T::MaxMetadataLength>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn are_subscriptions_closed)]
	pub type AreSubscriptionsClosed<T: Config> =
		StorageMap<_, Twox64Concat, PlanId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn active_subscriptions)]
	pub type ActiveSubscriptions<T: Config> =
		StorageMap<_, Twox64Concat, T::BlockNumber, Vec<InstalmentData<T::AccountId>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PlanCreated(PlanId, PlanData<T::BlockNumber, BalanceOf<T>, T::AccountId>),
		PlanDeleted(PlanId),
		PlanClosed(PlanId),
		PlanOpened(PlanId),
		Subscription(T::AccountId, SubscriptionId),
		SubscriptionToUser(
			T::AccountId,
			PlanData<T::BlockNumber, BalanceOf<T>, T::AccountId>,
			SubscriptionToUserId,
		),
		Unsubscription(T::AccountId, SubscriptionId),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidAmount,
		InvalidFrequency,
		InvalidNumberOfInstalment,
		CannotSubscribeToSelf,
		IndexOutOfBounds,
		NoSubscriptionPlannedAtBlock,
		CallerIsNotPayer,
		PlanDoesNotExist,
		PlanIdMustBeSome,
		MustBeOwner,
		SubscriptionsAreClosed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let limit = T::MaximumWeight::get();
			let mut total_weight: Weight = 0;

			let mut scheduled_subscriptions = <ActiveSubscriptions<T>>::take(block_number);
			total_weight += T::DbWeight::get().reads_writes(1 as Weight, 1 as Weight);

			while total_weight < limit {
				let sub_info = match scheduled_subscriptions.pop() {
					Some(data) => data,
					None => return total_weight,
				};

				total_weight += T::DbWeight::get().reads(1 as Weight);
				let plan_data = match Self::get_plan_form_id(sub_info.subscription_id) {
					Ok(data) => data,
					Err(_) => continue,
				};

				let res_transfer = T::Currency::transfer(
					&sub_info.payer,
					&plan_data.beneficiary,
					plan_data.amount,
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
							block_number + plan_data.frequency,
							&[InstalmentData {
								remaining_payments: Some(remaining_payments - 1),
								..sub_info
							}],
						);
					},
					None => {
						Self::schedule_subscriptions(
							block_number + plan_data.frequency,
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
		pub fn create_plan(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			frequency: T::BlockNumber,
			number_of_instalments: Option<u32>,
			metadata: BoundedVec<u8, T::MaxMetadataLength>,
		) -> DispatchResult {
			let beneficiary = ensure_signed(origin)?;

			ensure!(!frequency.is_zero(), Error::<T>::InvalidFrequency);
			ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
			ensure!(
				match number_of_instalments {
					Some(x) => x >= 1,
					None => true,
				},
				Error::<T>::InvalidNumberOfInstalment
			);

			let nonce = Self::plan_nonce();
			<PlanNonce<T>>::set(nonce + 1);

			let id: PlanId = nonce.into();
			let plan_data = PlanData {
				frequency,
				amount,
				number_of_instalments,
				beneficiary,
			};

			<SubscriptionPlan<T>>::insert(id, plan_data.clone());
			<PlanMetadata<T>>::insert(id, metadata);

			Self::deposit_event(Event::PlanCreated(id, plan_data));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn delete_plan(origin: OriginFor<T>, plan_id: PlanId) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let plan = Self::subscription_plan(plan_id).ok_or(Error::<T>::PlanDoesNotExist)?;

			ensure!(plan.beneficiary == sender, Error::<T>::MustBeOwner);

			Self::remove_plan_from_storage(plan_id);

			Self::deposit_event(Event::PlanDeleted(plan_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn close_plan(origin: OriginFor<T>, plan_id: PlanId) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let plan = Self::subscription_plan(plan_id).ok_or(Error::<T>::PlanDoesNotExist)?;

			ensure!(plan.beneficiary == sender, Error::<T>::MustBeOwner);

			<AreSubscriptionsClosed<T>>::insert(plan_id, true);

			Self::deposit_event(Event::PlanClosed(plan_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn open_plan(origin: OriginFor<T>, plan_id: PlanId) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let plan = Self::subscription_plan(plan_id).ok_or(Error::<T>::PlanDoesNotExist)?;

			ensure!(plan.beneficiary == sender, Error::<T>::MustBeOwner);

			<AreSubscriptionsClosed<T>>::insert(plan_id, false);

			Self::deposit_event(Event::PlanOpened(plan_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn subscribe_to_plan(origin: OriginFor<T>, plan_id: PlanId) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let plan = Self::subscription_plan(plan_id).ok_or(Error::<T>::PlanDoesNotExist)?;
			ensure!(plan.beneficiary != from, Error::<T>::CannotSubscribeToSelf);

			let subscription_id = SubscriptionId::Plan(plan_id);

			let next_block_number = <frame_system::Pallet<T>>::block_number() + 1u32.into();

			Self::schedule_subscriptions(
				next_block_number,
				&[InstalmentData {
					subscription_id,
					remaining_payments: plan.number_of_instalments,
					payer: from.clone(),
				}],
			);

			Self::deposit_event(Event::Subscription(from, subscription_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn subscribe_to_account(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			frequency: T::BlockNumber,
			number_of_instalments: Option<u32>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			ensure!(!frequency.is_zero(), Error::<T>::InvalidFrequency);
			ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
			ensure!(
				match number_of_instalments {
					Some(x) => x >= 1,
					None => true,
				},
				Error::<T>::InvalidNumberOfInstalment
			);
			ensure!(from != to, Error::<T>::CannotSubscribeToSelf);

			let nonce = Self::subscription_to_user_nonce();
			<SubscriptionToUserNonce<T>>::set(nonce + 1);

			let id: SubscriptionToUserId = nonce.into();
			let plan_data = PlanData {
				frequency,
				amount,
				number_of_instalments,
				beneficiary: to,
			};

			<SubscriptionToUser<T>>::insert(id, plan_data.clone());

			let next_block_number = <frame_system::Pallet<T>>::block_number() + 1u32.into();

			Self::schedule_subscriptions(
				next_block_number,
				&[InstalmentData {
					subscription_id: id.into(),
					remaining_payments: number_of_instalments,
					payer: from.clone(),
				}],
			);

			Self::deposit_event(Event::SubscriptionToUser(from, plan_data, id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn unsubscribe(
			origin: OriginFor<T>,
			when: T::BlockNumber,
			index: u32,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let mut instalments = Self::active_subscriptions(when);
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
				Error::<T>::CallerIsNotPayer,
			);

			// Those two lines are safe because we checked index < length
			// Doing this rather than calling remove reduce complexity to 0(1)
			instalments.swap(index, length - 1);
			let subscription_data = instalments.pop().unwrap();

			<ActiveSubscriptions<T>>::insert(when, instalments);
			if let SubscriptionId::User(id) = subscription_data.subscription_id {
				<SubscriptionToUser<T>>::remove(id);
			}

			Self::deposit_event(Event::Unsubscription(
				from,
				subscription_data.subscription_id,
			));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn schedule_subscriptions(
			when: T::BlockNumber,
			new_subscription: &[InstalmentData<T::AccountId>],
		) {
			<ActiveSubscriptions<T>>::mutate(when, |current_subscriptions| {
				current_subscriptions.extend_from_slice(new_subscription);
			});
		}

		fn remove_plan_from_storage(plan_id: PlanId) {
			<SubscriptionPlan<T>>::remove(plan_id);
			<PlanMetadata<T>>::remove(plan_id);
			<AreSubscriptionsClosed<T>>::remove(plan_id);
		}

		fn get_plan_form_id(
			plan_id: SubscriptionId,
		) -> Result<PlanData<T::BlockNumber, BalanceOf<T>, T::AccountId>, Error<T>> {
			match plan_id {
				SubscriptionId::None => Err(Error::<T>::PlanIdMustBeSome)?,
				SubscriptionId::Plan(id) => Self::subscription_plan(id),
				SubscriptionId::User(id) => Self::subscription_to_user(id),
			}
			.ok_or(Error::<T>::PlanDoesNotExist)
		}
	}
}
