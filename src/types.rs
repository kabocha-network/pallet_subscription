use crate::*;
use scale_info::TypeInfo;
use codec::{Decode, Encode};
use frame_support::{
    storage::bounded_vec::BoundedVec,
    pallet_prelude::MaxEncodedLen,
};

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type Nonce = u64;

#[derive(Clone, Encode, Decode, TypeInfo, Debug, MaxEncodedLen)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Annualy,
}

#[derive(Clone, Encode, Decode, TypeInfo, Debug, MaxEncodedLen)]
#[codec(mel_bound())]
pub struct SubscriptionPlan<T: Config> {
    pub frequency: Frequency,
    pub amount: BalanceOf<T>,
    pub beneficiary: T::AccountId,
    pub metadata: BoundedVec<u8, T::MaxMetadataLength>
}
