#![cfg_attr(not(feature = "std"), no_std)]
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

use frame_support::{dispatch::DispatchResult};
use frame_system::ensure_signed;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

pub type KeyLedger = str;
pub type ValueLedger = u32;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn payments_ledger)]
	pub(super) type PaymentsLedger<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Twox64Concat, u32>,
		),
		u32,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn confirmations_ledger)]
	pub(super) type ConfirmationsLedger<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Twox64Concat, u32>,
		),
		u32,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		ElementGotFromPaymentsLedger {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			value: u32,
		},
		ElementAddedToPaymentsLedger {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			value: u32,
		},
		ElementRemovedFromPaymentsLedger {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
		},
		ElementInPaymentsLedger(),
		ElementNotInPaymentsLedger(),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		ElementAlreadyExists,
		ElementNotExists,
	}

	// Calls
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::add_element())]
		pub fn add_element(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32, value: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			// The signer must be the sender in the key of the element to be inserted
			PaymentsLedger::<T>::insert((source.clone(), key_receiver.clone(), ts), value);

			Self::deposit_event(Event::ElementAddedToPaymentsLedger { key_sender: source, key_receiver, ts, value });

			Ok(())
		}

		// Calls with null weights
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::exist_call())]
		pub fn exist_call(origin: OriginFor<T>, key_sender: T::AccountId, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			if PaymentsLedger::<T>::contains_key((key_sender, key_receiver, ts)) == true {
				Self::deposit_event(Event::ElementInPaymentsLedger());
			}
			else {
				Self::deposit_event(Event::ElementNotInPaymentsLedger());
			}

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::get_element_call())]
		pub fn get_element_call(origin: OriginFor<T>, key_sender: T::AccountId, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let value = PaymentsLedger::<T>::get((key_sender.clone(), key_receiver.clone(), ts));

			Self::deposit_event(Event::ElementGotFromPaymentsLedger { key_sender, key_receiver, ts, value });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::remove_element())]
		pub fn remove_element(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			// The signer must be the sender in the key of the element to be removed
			PaymentsLedger::<T>::remove((source.clone(), key_receiver.clone(), ts));

			Self::deposit_event(Event::ElementRemovedFromPaymentsLedger { key_sender: source, key_receiver, ts});

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::set_element())]
		pub fn set_element(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32, value: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			// The signer must be the sender in the key of the element to be removed
			PaymentsLedger::<T>::set((source.clone(), key_receiver.clone(), ts), value);

			Self::deposit_event(Event::ElementRemovedFromPaymentsLedger { key_sender: source, key_receiver, ts});

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::add_element_with_error())]
		pub fn add_element_with_error(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32, value: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			match <PaymentsLedger<T>>::contains_key((source.clone(), key_receiver.clone(), ts)) {
				true => return Err(Error::<T>::ElementAlreadyExists.into()),
				false => {
					// Insert the new element
					PaymentsLedger::<T>::insert((source.clone(), key_receiver.clone(), ts), value);
					Self::deposit_event(Event::ElementAddedToPaymentsLedger { key_sender: source, key_receiver, ts, value });
					Ok(())
				},
			}
		}

		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::remove_element_with_error())]
		pub fn remove_element_with_error(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			match <PaymentsLedger<T>>::contains_key((source.clone(), key_receiver.clone(), ts)) {
				false => return Err(Error::<T>::ElementNotExists.into()),
				true => {
					// Remove the element
					PaymentsLedger::<T>::remove((source.clone(), key_receiver.clone(), ts));
					Self::deposit_event(Event::ElementRemovedFromPaymentsLedger { key_sender: source, key_receiver, ts});
					Ok(())
				},
			}
		}
	}

	// Queries
	impl<T: Config> Pallet<T> {
		pub fn get_element(key_sender: T::AccountId, key_receiver: T::AccountId, timestamp: u32) -> u32 {
			return PaymentsLedger::<T>::get((&key_sender, &key_receiver, &timestamp))
		}

		pub fn exist(key_sender: T::AccountId, key_receiver: T::AccountId, timestamp: u32) -> bool {
			return PaymentsLedger::<T>::contains_key((&key_sender, &key_receiver, &timestamp))
		}
	}
}