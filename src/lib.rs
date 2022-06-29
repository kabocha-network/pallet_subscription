#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;

pub use types::*;
use codec::EncodeLike;

#[cfg(test)]
mod tests;

pub use frame_support::{
	ReversibleStorageHasher,
	storage::IterableStorageMap,
	traits::tokens::{
		currency::{Currency, ReservableCurrency},
		ExistenceRequirement,
	},
};


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
	use frame_support::sp_runtime::traits::Zero;

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
	pub type Subscriptions<T: Config> =
	StorageMap<_, Blake2_256, u32, Vec<(Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>, T::AccountId)>, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Subscription has been created
		SubscriptionCreated(T::AccountId, T::AccountId, BalanceOf<T>, T::BlockNumber),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid subscription
		InvalidSubscription
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
		pub fn subscribe(origin: OriginFor<T>, to: T::AccountId,
					 amount: BalanceOf<T>, frequency: T::BlockNumber, number_of_installment: Option<u32>) -> DispatchResult {

			let from = ensure_signed(origin)?;

			// check if subscription is valid
			ensure!(!frequency.is_zero() && !amount.is_zero(), Error::<T>::InvalidSubscription);

			let sub = Subscription {
				frequency: frequency.clone(),
				amount: amount.clone(),
				remaining_payments: number_of_installment,
				beneficiary: to.clone(),
			};

			let subscriptions: Vec<(Subscription<T::BlockNumber, BalanceOf<T>, T::AccountId>, T::AccountId)>
				= vec![(sub, from.clone())];

			let key = TryInto::<u32>::try_into(<frame_system::Pallet<T>>::block_number()).ok();
			let key_to_u32 =  match key {
				Some(key) => key + 1,
				_ => 0,
			};

			<Subscriptions<T>>::mutate(key_to_u32,|val| {
				*val = Option::from(subscriptions);
			});

			Self::deposit_event(Event::SubscriptionCreated(to, from,amount, frequency));
			Ok(())
		}

	}
}
