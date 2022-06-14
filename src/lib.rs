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
	#[pallet::getter(fn subscription_plans)]
	pub type SubscriptionPlans<T: Config> =
		StorageMap<_, Blake2_256, Nonce, SubscriptionPlan<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn plan_nonce)]
	pub type PlanNonce<T> = StorageValue<_, Nonce, ValueQuery>;

    // actually the best solution rn because we dont need that much key ? idk
	#[pallet::storage]
	#[pallet::getter(fn subscriptions)]
    pub type Subscriptions<T: Config> =
        StorageMap<_, Blake2_256, Nonce, Subscription<T>, OptionQuery>;


    //// OPTION WITH A LOT OF STORAGE
    /// way better for the execution, but some constraints
    ///
    /// - check if a subscription is running = 4 reads, not cool // oooor maybe 2 read, get the
    /// recurence in the plans, and get in the right storage ?
    /// - we can have ONLY 4 types of recurence (or we need another storage for each recurence)

    // pub type DailySubscriptions<T: Config> =
    //     StorageMap<_, Blake2_256, Nonce, Subscription<T>, OptionQuery>;
    // pub type WeeklySubscriptions<T: Config> =
    //     StorageMap<_, Blake2_256, Nonce, Subscription<T>, OptionQuery>;
    // pub type MonthlySubscriptions<T: Config> =
    //     StorageMap<_, Blake2_256, Nonce, Subscription<T>, OptionQuery>;
    // pub type AnnualySubscriptions<T: Config> =
    //     StorageMap<_, Blake2_256, Nonce, Subscription<T>, OptionQuery>;
    //

	// might be a good way too but more complicated idk
    // first solution simpler atm
    //
	// #[pallet::storage]
	// #[pallet::getter(fn subscriptions)]
 //    pub type Subscriptions<T: Config> =
 //        StorageDoubleMap<
 //        _,
 //        Blake2_256,
 //        AccountId,
 //        Blake2_256,
 //        Nonce,
 //        Subscription<T>,
 //        OptionQuery,
 //    >;
 //
 //    easier for execution ?
 //
 //    pub type Subscriptions<T: Config> =
 //        StorageDoubleMap<
 //        _,
 //        Blake2_256,
 //        AccountId,
 //        Blake2_256,
 //        Nonce,
 //        Recurence, // or block_number or idk
 //        OptionQuery,
 //    >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
	}

    // this example comes from treasury
    //
	// #[pallet::hooks]
	// impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
	// 	/// # <weight>
	// 	/// - Complexity: `O(A)` where `A` is the number of approvals
	// 	/// - Db reads and writes: `Approvals`, `pot account data`
	// 	/// - Db reads and writes per approval: `Proposals`, `proposer account data`, `beneficiary
	// 	///   account data`
	// 	/// - The weight is overestimated if some approvals got missed.
	// 	/// # </weight>
	// 	fn on_initialize(n: T::BlockNumber) -> Weight {
	// 		// Check to see if we should spend some funds!
	// 		if (n % T::SpendPeriod::get()).is_zero() {
	// 			Self::spend_funds()
	// 		} else {
	// 			0
	// 		}
	// 	}
	// }
    //
    // this is OUR hook
    // #[pallet::hooks]
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	//     fn on_initialize(n: T::BlockNumber) -> Weight {
    //         //// prop 1:
    //         - loop over all subscriptions (.iter_values())
    //         - check if the subscription shoud be taken
    //         - take it
    //
    //         //// prop 2: (2nd key: )
    //
    // 	   }
    // }

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1)]
		pub fn create_subsciption_plan(origin: OriginFor<T>, plan: SubscriptionPlan<T>) -> DispatchResult {
            // maybe make a new function for Subscription, so `plan` is already valid 100%
            //
            //
            // todo:
            // push the plan to the storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn another_create_subsciption_plan(origin: OriginFor<T>, /* data of the struct so we can verify */) -> DispatchResult {
            // better ? 5 parameters but we can check, better option if no plan.new() function
            //
            //
            // todo:
            // push the plan to the storage
			Ok(())
		}

		#[pallet::weight(1)]
		pub fn delete_subsciption_plan(origin: OriginFor<T>, plan_id: Nonce) -> DispatchResult {
            // better ? 5 parameters but we can check, better option if no plan.new() function
            //
            //
            // todo:
            // push the plan to the storage
			Ok(())
		}

        #[pallet::weight(1)]
        pub fn subscribe(origin: OriginFor<T>, plan_id: Nonce) -> DispatchResult {
            // todo:
            // check if plan exist
            // if yes, push the nonce to the subscription storage
        }

        #[pallet::weight(1)]
        pub fn unsubscribe(origin: OriginFor<T>, plan_id: Nonce) -> DispatchResult {
            //kinda obv
        }
	}
}
