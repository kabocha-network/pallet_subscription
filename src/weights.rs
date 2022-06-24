#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn do_execute_subscriptions(itts: u32) -> Weight;
}

impl WeightInfo for () {
    fn do_execute_subscriptions(itts: u32) -> Weight {
        (itts as Weight)
    }
}

/// Weight functions for `pallet_supersig`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn do_execute_subscriptions(itts: u32) -> Weight {
		(0u64)
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
			.saturating_add((0u64).saturating_mul(itts as Weight))
	}
}
