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

const CONFIRMATION_OK: u32 = 1;
const CONFIRMATION_NOK_OVERESTIMATION: u32 = 2;
const CONFIRMATION_NOK_UNDERESTIMATION: u32 = 3;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub const FLEXIBILITY_SELLING_STATE_NOT_DECIDED: u32 = 1;
	pub  const FLEXIBILITY_SELLING_STATE_CONFIRMED: u32 = 2;
	pub const FLEXIBILITY_SELLING_STATE_REJECTED: u32 = 3;

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

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
	pub struct FlexibilitySellingData {
		// Sold power of the asset/flexibility
		pub sold_power: u32,
		// Change FCT/W related to a specific flexibility market
		pub change_fct_w: u32,
		// Selling state
		pub state: u32,
	}

	#[pallet::storage]
	#[pallet::getter(fn flexibility_market_ledger)]
	pub(super) type FlexibilityMarketLedger<T: Config> = StorageNMap<
		_,
		(
			// Seller
			NMapKey<Blake2_128Concat, T::AccountId>,
			// Buyer
			NMapKey<Blake2_128Concat, T::AccountId>,
			// Flexibility market identifier
			NMapKey<Twox64Concat, u32>,
			// Flexibility market timestamp
			NMapKey<Twox64Concat, u32>,
			// Asset/flexibility identifier
			NMapKey<Twox64Concat, u32>,
		),
		FlexibilitySellingData,
	    ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Events related to Payments StorageNMap
		GotFromPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			value: u32,
		},
		AddedToPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			value: u32,
		},
		RemovedFromPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
		},
		SetInPayments {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
		},
		InPayments(),
		NotInPayments(),

		// Events related to Confirmations StorageNMap
		AddedToConfirmations {
			key_confirmer: T::AccountId,
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
			status: u32,
		},
		RemovedFromConfirmations {
			key_sender: T::AccountId,
			key_receiver: T::AccountId,
			ts: u32,
		},
		ConfirmationOK(),
		ConfirmationOverEstimation(),
		ConfirmationUnderEstimation(),

		SuccessfullySoldFlexibility {
			seller: T::AccountId,
			buyer: T::AccountId,
			flexibility_market_identifier: u32,
			flexibility_market_timestamp: u32,
			asset_identifier: u32,
			sold_power: u32,
			change_fct_w: u32
		},
		AlreadySoldFlexibility {
			seller: T::AccountId,
			buyer: T::AccountId,
			flexibility_market_identifier: u32,
			flexibility_market_timestamp: u32,
			asset_identifier: u32,
		},
		FlexibilitySellingNotExisting {
			seller: T::AccountId,
			buyer: T::AccountId,
			flexibility_market_identifier: u32,
			flexibility_market_timestamp: u32,
			asset_identifier: u32,
		},
		FlexibilitySellingConfirmed {
			seller: T::AccountId,
			buyer: T::AccountId,
			flexibility_market_identifier: u32,
			flexibility_market_timestamp: u32,
			asset_identifier: u32,
		},
		FlexibilitySellingRejected {
			seller: T::AccountId,
			buyer: T::AccountId,
			flexibility_market_identifier: u32,
			flexibility_market_timestamp: u32,
			asset_identifier: u32,
		}
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
		FlexibilitySellingNotExisting,
		FlexibilitySellingAlreadyDecided,
		FlexibilitySellingRejected,
		FlexibilitySellingUnknownState,
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
				Self::deposit_event(Event::InPayments());
			}
			else {
				Self::deposit_event(Event::NotInPayments());
			}

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::get_payment_call())]
		pub fn get_payment_call(origin: OriginFor<T>, key_sender: T::AccountId, key_receiver: T::AccountId, ts: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let value = Payments::<T>::get((key_sender.clone(), key_receiver.clone(), ts));

			Self::deposit_event(Event::GotFromPayments { key_sender, key_receiver, ts, value });

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
							Self::deposit_event(Event::SetInPayments { key_sender: source, key_receiver, ts});
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
					Self::deposit_event(Event::AddedToPayments { key_sender: source, key_receiver, ts, value });
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
							Self::deposit_event(Event::RemovedFromPayments { key_sender: source, key_receiver, ts});
							Ok(())
						}
					}
				},
			}
		}

		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::add_confirmation())]
		pub fn add_confirmation(origin: OriginFor<T>, key_sender: T::AccountId, ts: u32, status: u32) -> DispatchResult {
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
							Confirmations::<T>::insert((key_sender.clone(), source.clone(), ts), status);

							Self::deposit_event(Event::AddedToConfirmations { key_confirmer: source.clone(), key_sender: key_sender, key_receiver: source.clone(), ts, status });

							if status == CONFIRMATION_OK {
								Self::deposit_event(Event::ConfirmationOK ());
							}
							else if status == CONFIRMATION_NOK_OVERESTIMATION {
								Self::deposit_event(Event::ConfirmationOverEstimation ());
							}
							else if status == CONFIRMATION_NOK_UNDERESTIMATION {
								Self::deposit_event(Event::ConfirmationUnderEstimation ());
							}

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
					Self::deposit_event(Event::RemovedFromConfirmations { key_sender: key_sender.clone(), key_receiver: source.clone(), ts});
					Ok(())
				}
			}
		}

		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::flexibility_selling())]
		pub fn flexibility_selling(origin: OriginFor<T>,
								   buyer: T::AccountId,
								   flexibility_market_identifier: u32,
								   flexibility_market_timestamp: u32,
								   asset_identifier: u32,
								   sold_power: u32,
								   change_fct_w: u32,
								   ) -> DispatchResult {
			let seller = ensure_signed(origin.clone())?;
			let flexibility_data = FlexibilitySellingData { sold_power, change_fct_w, state: FLEXIBILITY_SELLING_STATE_NOT_DECIDED};
			FlexibilityMarketLedger::<T>::insert((seller.clone(), buyer.clone(), flexibility_market_identifier, flexibility_market_timestamp, asset_identifier),
												 flexibility_data);

			if FlexibilityMarketLedger::<T>::contains_key((seller.clone(), buyer.clone(), flexibility_market_identifier, flexibility_market_timestamp, asset_identifier)) == true {
				Self::deposit_event(Event::AlreadySoldFlexibility {
					seller,
					buyer,
					flexibility_market_identifier,
					flexibility_market_timestamp,
					asset_identifier
				});
			}
			else {
				Self::deposit_event(Event::SuccessfullySoldFlexibility {
					seller,
					buyer,
					flexibility_market_identifier,
					flexibility_market_timestamp,
					asset_identifier,
					sold_power,
					change_fct_w
				});
			}

			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::flexibility_purchase())]
		pub fn flexibility_purchase_decision(origin: OriginFor<T>,
											seller: T::AccountId,
											flexibility_market_identifier: u32,
											flexibility_market_timestamp: u32,
											asset_identifier: u32,
											new_state: u32
		) -> DispatchResult {
			let buyer = ensure_signed(origin.clone())?;

			match <FlexibilityMarketLedger<T>>::contains_key((seller.clone(), buyer.clone(), flexibility_market_identifier, flexibility_market_timestamp, asset_identifier)) {
				false => {
					// Not existing entry
					Self::deposit_event(Event::FlexibilitySellingNotExisting {
						seller,
						buyer,
						flexibility_market_identifier,
						flexibility_market_timestamp,
						asset_identifier
					});
					return Err(Error::<T>::FlexibilitySellingNotExisting.into())
				},
				true => {
					let mut flexibility_data = FlexibilityMarketLedger::<T>::get((&seller, &buyer, &flexibility_market_identifier, &flexibility_market_timestamp, &asset_identifier));
					match flexibility_data.state {
						FLEXIBILITY_SELLING_STATE_NOT_DECIDED => {
							match new_state {
								// The selling is confirmed by the buyer
								FLEXIBILITY_SELLING_STATE_CONFIRMED => {
									// Market state confirmation
									flexibility_data.state = new_state;
									FlexibilityMarketLedger::<T>::set((seller.clone(), buyer.clone(), flexibility_market_identifier, flexibility_market_timestamp, asset_identifier),
																	  flexibility_data.clone());

									// Perform the payment
									let tkns_to_pay = flexibility_data.sold_power * flexibility_data.change_fct_w;
									Payments::<T>::insert((buyer.clone(), seller.clone(), flexibility_market_timestamp), tkns_to_pay);

									Self::deposit_event(Event::FlexibilitySellingConfirmed {
										seller,
										buyer,
										flexibility_market_identifier,
										flexibility_market_timestamp,
										asset_identifier
									});
									Ok(())
								}
								// The selling is rejected by the buyer
								FLEXIBILITY_SELLING_STATE_REJECTED => {
									// Market state rejection
									flexibility_data.state = new_state;
									FlexibilityMarketLedger::<T>::set((seller.clone(), buyer.clone(), flexibility_market_identifier, flexibility_market_timestamp, asset_identifier),
																	  flexibility_data.clone());

									Self::deposit_event(Event::FlexibilitySellingRejected {
										seller,
										buyer,
										flexibility_market_identifier,
										flexibility_market_timestamp,
										asset_identifier
									});
									Ok(())
								}
								_ => { return Err(Error::<T>::FlexibilitySellingUnknownState.into()) }
							}
						}
						_ => {
							return Err(Error::<T>::ConfirmationAlreadyExists.into())
						}
					}
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
		pub fn get_flexibility_selling(seller: T::AccountId,
									   buyer: T::AccountId,
									   flexibility_market_identifier: u32,
									   flexibility_market_timestamp: u32,
									   asset_identifier: u32) -> FlexibilitySellingData {
			return FlexibilityMarketLedger::<T>::get((&seller, &buyer, &flexibility_market_identifier, &flexibility_market_timestamp, &asset_identifier))
		}
	}
}
