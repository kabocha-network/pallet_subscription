use crate::*;
use scale_info::TypeInfo;
use codec::{Decode, Encode};
// use frame_support::{
//     // storage::bounded_vec::BoundedVec,
//     pallet_prelude::MaxEncodedLen,
// };

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type Nonce = u64;
pub type PlanId = u64;
pub type SubscriptionId = u64;

// #[derive(Clone, Encode, Decode, TypeInfo, Debug, MaxEncodedLen)]
// pub enum Frequency {
//     Daily,
//     Weekly,
//     Monthly,
//     Annualy,
// }

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
// #[codec(mel_bound())]
pub struct Subscription<T: Config> {
    pub frequency: u32, // block number
    pub amount: BalanceOf<T>,
    pub remaining_payments: Option<u32>,
    pub beneficiary: T::AccountId,
    // for later
    // pub metadata: BoundedVec<u8, T::MaxMetadataLength>
}
