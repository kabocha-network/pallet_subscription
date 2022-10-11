use crate::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type Nonce = u64;

#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, Debug, MaxEncodedLen)]
pub struct InstalmentData<AccountId> {
	pub subscription_id: SubscriptionId,
	pub remaining_payments: Option<u32>,
	pub payer: AccountId,
}

#[derive(Debug, Clone, TypeInfo, Encode, Decode, MaxEncodedLen, PartialEq, Eq)]
pub struct PlanData<BlockNumber, Balance, AccountId> {
	pub beneficiary: AccountId,
	pub amount: Balance,
	pub frequency: BlockNumber,
	pub number_of_instalments: Option<u32>,
}

#[derive(Copy, Clone, Encode, Decode, TypeInfo, PartialEq, Eq, Debug, MaxEncodedLen, Default)]
pub struct PlanId(pub u64);

impl From<u64> for PlanId {
	fn from(id: u64) -> Self {
		Self(id)
	}
}

#[derive(Copy, Clone, Encode, Decode, TypeInfo, PartialEq, Eq, Debug, MaxEncodedLen, Default)]
pub struct SubscriptionToUserId(pub u64);

impl From<u64> for SubscriptionToUserId {
	fn from(id: u64) -> Self {
		Self(id)
	}
}

#[derive(Copy, Clone, Encode, Decode, TypeInfo, PartialEq, Eq, Debug, MaxEncodedLen, Default)]
pub enum SubscriptionId {
	#[default]
	None,
	Plan(PlanId),
	User(SubscriptionToUserId),
}

impl From<PlanId> for SubscriptionId {
	fn from(id: PlanId) -> Self {
		Self::Plan(id)
	}
}

impl From<SubscriptionToUserId> for SubscriptionId {
	fn from(id: SubscriptionToUserId) -> Self {
		Self::User(id)
	}
}
