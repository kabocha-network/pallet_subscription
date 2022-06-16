use crate::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type Nonce = u64;
pub type PlanId = u64;
pub type SubscriptionId = u64;

#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, Debug, MaxEncodedLen)]
pub struct Subscription<BlockNumber, Balance, AccountId> {
	pub frequency: BlockNumber,
	pub amount: Balance,
	pub remaining_payments: Option<u32>,
	pub beneficiary: AccountId,
}

#[macro_export]
macro_rules! unwrap_or_return {
	( $e:expr, $f:expr ) => {
		match $e {
			Some(x) => x,
			None => return $f,
		}
	};
}
