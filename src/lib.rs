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
		/// The maximum amount of metadata
		#[pallet::constant]
		type MaxMetadataLength: Get<u32>;
		/////!TODO: add a u32 type that represent the maximum weigth the hook is allowed to take
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
		fn on_initialize(n: T::BlockNumber) -> Weight {
			//!TODO: déjà la première chose, c'est qu'on soit sur que la weight de la hook prenne
			// pas plus que prévu. du coup on introduit un type (l 38) qui représente ca, et on
			// l'utilise plus tard
			let current_weight: u64 = 0; //u64 psk on veut retourner ca, et Weight c'est un u64
			//
			// Nous, on va utiliser le storage Subscription. c'est une map, donc une seule clef, et
			// cette clef, c'est un blocknumber, donc parfait, on en a un qui est envoyé par la
			// hook: n
			//
			// donc qqch du genre:
		let subscriptions = Subscriptions<T>.take(n); // ptet pas la bonne syntaxe, regarde plus bas
													  // comment c'est utilisé
			// si c'est None, tu return 0, sinon tu continue. tu peux faire ca dans un if let c plus
			// simple
			//
			if let Some(subs) = subscriptions {
				// la subs c'est un vec de tuple, donc tu peux déconstruire:
				for (sub, src) in subs {
					// la plusieurs step:
					// faire un balance transfer de `sub.amount` de `src` vers `sub.benef`
					// si y'a un remaining payment:
					// // si c'est 1, tu fais rien et passe a la subscription suivante (donc tu drop
					// // celle ci)
					// // si c'est > 1, tu decrease de 1
					//
				//	et tu reschedule le prochain payment au prochain blocknumber, donc faut que tu
					//	fasse un Subscription<T>.mutate(n + frequency, |smth| {et tu ajoute la
					//	subscription a smth})
				}
				//ce qui serait bien, c'est que tu trouve un moyen de chopper la weight d'un read et
				//d'un write, et que a chaque fois que tu fais une des deux op, tu ajoute la weight
				//a `current_weight`, comme ca la hook est pixel. jpense tu peux regarder supersig
				//dans weight.rs
			}

			current_weight
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

			let new_subscription = (subscription, from.clone());

			let next_block_number = <frame_system::Pallet<T>>::block_number() + 1u32.into();

			<Subscriptions<T>>::mutate(next_block_number, |wrapped_current_subscriptions| {
				if let Some(current_subscriptions) = wrapped_current_subscriptions {
					current_subscriptions.push(new_subscription);
				} else {
					*wrapped_current_subscriptions = Some(vec![new_subscription]);
				}
			});

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
}
