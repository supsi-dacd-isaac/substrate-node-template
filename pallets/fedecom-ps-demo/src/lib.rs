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
	#[pallet::getter(fn payments)]
	pub(super) type Payments<T: Config> = StorageNMap<
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
	#[pallet::getter(fn confirmations)]
	pub(super) type Confirmations<T: Config> = StorageNMap<
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
		// Events related to Payments StorageNMap
		ElementGotFromPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			value: u32,
		},
		ElementAddedToPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			value: u32,
		},
		ElementRemovedFromPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
		},
		ElementSetInPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
		},
		ElementInPayments(),
		ElementNotInPayments(),

		// Events related to Confirmations StorageNMap
		ElementAddedToConfirmations {
			key_confirmer: T::AccountId,
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			value: u32,
		},
		ElementRemovedFromConfirmations {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		PaymentAlreadyExists,
		PaymentNotExists,
		ConfirmationAlreadyExists,
		ConfirmationNotExists,
	}

	// Calls
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Calls with null weights
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::check_payment_call())]
		pub fn check_payment_call(origin: OriginFor<T>, key_sender: T::AccountId, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			if Payments::<T>::contains_key((key_sender, key_receiver, ts)) == true {
				Self::deposit_event(Event::ElementInPayments());
			}
			else {
				Self::deposit_event(Event::ElementNotInPayments());
			}

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::check_payment_call())]
		pub fn get_payment_call(origin: OriginFor<T>, key_sender: T::AccountId, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let value = Payments::<T>::get((key_sender.clone(), key_receiver.clone(), ts));

			Self::deposit_event(Event::ElementGotFromPayments { key_sender, key_receiver, ts, value });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::modify_payment())]
		pub fn modify_payment(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32, value: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			match <Payments<T>>::contains_key((source.clone(), key_receiver.clone(), ts)) {
				false => return Err(Error::<T>::PaymentNotExists.into()),
				true => {
					// Check if a confirmation with the triple (sender, receiver, timestamp) has already been stored
					match <Confirmations<T>>::contains_key((source.clone(), key_receiver.clone(), ts)) {
						true => return Err(Error::<T>::ConfirmationAlreadyExists.into()),
						false => {
							// Modify the payment
							Payments::<T>::set((source.clone(), key_receiver.clone(), ts), value);
							Self::deposit_event(Event::ElementSetInPayments { key_sender: source, key_receiver, ts});
							Ok(())
						}
					}
				},
			}
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::add_payment())]
		pub fn add_payment(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32, value: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			match <Payments<T>>::contains_key((source.clone(), key_receiver.clone(), ts)) {
				true => return Err(Error::<T>::PaymentAlreadyExists.into()),
				false => {
					// Insert the new payment
					Payments::<T>::insert((source.clone(), key_receiver.clone(), ts), value);
					Self::deposit_event(Event::ElementAddedToPayments { key_sender: source, key_receiver, ts, value });
					Ok(())
				},
			}
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::remove_payment())]
		pub fn remove_payment(origin: OriginFor<T>, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			match <Payments<T>>::contains_key((source.clone(), key_receiver.clone(), ts)) {
				false => return Err(Error::<T>::PaymentNotExists.into()),
				true => {
					// Check if a confirmation with the triple (sender, receiver, timestamp) has already been stored
					match <Confirmations<T>>::contains_key((source.clone(), key_receiver.clone(), ts)) {
						true => return Err(Error::<T>::ConfirmationAlreadyExists.into()),
						false => {
							// Remove the payment
							Payments::<T>::remove((source.clone(), key_receiver.clone(), ts));
							Self::deposit_event(Event::ElementRemovedFromPayments { key_sender: source, key_receiver, ts});
							Ok(())
						}
					}
				},
			}
		}

		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::add_confirmation())]
		pub fn add_confirmation(origin: OriginFor<T>, key_sender: T::AccountId, ts: u32, value: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			// Check if a payment with the triple (sender, receiver, timestamp) has already been stored
			match <Payments<T>>::contains_key((key_sender.clone(), source.clone(), ts)) {
				false => return Err(Error::<T>::PaymentNotExists.into()),
				true => {
					// Check if a confirmation with the triple (sender, receiver, timestamp) has already been stored
					match <Confirmations<T>>::contains_key((key_sender.clone(), source.clone(), ts)) {
						true => return Err(Error::<T>::ConfirmationAlreadyExists.into()),
						false => {
							// Insert the new confirmation
							Confirmations::<T>::insert((key_sender.clone(), source.clone(), ts), 1);
							Self::deposit_event(Event::ElementAddedToConfirmations { key_confirmer: source.clone(), key_sender: key_sender, key_receiver: source.clone(), ts, value });
							Ok(())
						}
					}
				},
			}
		}

		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::remove_confirmation())]
		pub fn remove_confirmation(origin: OriginFor<T>, key_sender: T::AccountId, ts: u32) -> DispatchResult {
			let source = ensure_signed(origin.clone())?;

			// Check if a confirmation with the triple (sender, receiver, timestamp) has already been stored
			match <Confirmations<T>>::contains_key((key_sender.clone(), source.clone(), ts)) {
				false => return Err(Error::<T>::ConfirmationNotExists.into()),
				true => {
					// Remove the confirmation
					Confirmations::<T>::remove((key_sender.clone(), source.clone(), ts));
					Self::deposit_event(Event::ElementRemovedFromConfirmations { key_sender: key_sender.clone(), key_receiver: source.clone(), ts});
					Ok(())
				}
			}
		}
	}

	// Queries
	impl<T: Config> Pallet<T> {
		pub fn get_payment(key_sender: T::AccountId, key_receiver: T::AccountId, timestamp: u32) -> u32 {
			return Payments::<T>::get((&key_sender, &key_receiver, &timestamp))
		}

		pub fn check_payment(key_sender: T::AccountId, key_receiver: T::AccountId, timestamp: u32) -> bool {
			return Payments::<T>::contains_key((&key_sender, &key_receiver, &timestamp))
		}

		pub fn get_confirmation(key_sender: T::AccountId, key_receiver: T::AccountId, timestamp: u32) -> u32 {
			return Payments::<T>::get((&key_sender, &key_receiver, &timestamp))
		}

		pub fn check_confirmation(key_sender: T::AccountId, key_receiver: T::AccountId, timestamp: u32) -> bool {
			return Confirmations::<T>::contains_key((&key_sender, &key_receiver, &timestamp))
		}
	}
}
