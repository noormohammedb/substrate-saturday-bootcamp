#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
		pallet_prelude::{ValueQuery, *},
		sp_runtime::traits::Hash,
		traits::{tokens::Balance, Currency, ExistenceRequirement, Randomness, StorageMapShim},
		BoundedVec, Twox64Concat,
	};
	// use frame_system::{pallet_prelude::*, Config};
	use frame_system::pallet_prelude::{OriginFor, *};
	use sp_core::H256;
	use sp_io::hashing::blake2_128;

	use log;

	// TODO Part II: Struct for holding Kitty information.

	// TODO Part II: Enum and implementation to handle Gender type in Kitty struct.

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	// #[pallet::without_storage_info]
	// pub struct Pallet<T>(PhantomData<T>);
	pub struct Pallet<T>(_);

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 16],
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: AccountOf<T>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the Kitties pallet.
		type Currency: Currency<Self::AccountId>;

		// TODO Part II: Specify the custom types for our runtime.

		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;

		// For Random Number Generator
		type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		// TODO Part III
		KittyCountOverflow,
		ExceedMaxKittyOwned,
	}

	#[pallet::event]
	// #[pallet::metadata(T::AccountId = "AccountId")] // <----------------------- error
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO Part III
		Created(T::AccountId, T::Hash),
	}

	// ACTION: Storage item to keep a count of all existing Kitties.
	#[pallet::storage]
	#[pallet::getter(fn kitty_count)]
	pub(super) type kittyCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Kitty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxKittyOwned>, ValueQuery>;

	// TODO Part II: Remaining storage items.

	// TODO Part III: Our pallet's genesis configuration.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO Part III: create_kitty
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_kitty(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let kitty_id = Self::mint(&sender, None, None);

			log::info!("A kitty is born with ID: {:?}.", kitty_id);

			Self::deposit_event(Event::<T>::Created(sender, kitty_id.unwrap()));

			Ok(())
		}

		// TODO Part III: set_price

		// TODO Part III: transfer

		// TODO Part III: buy_kitty

		// TODO Part III: breed_kitty
	}

	// TODO Parts II: helper function for Kitty struct

	impl<T: Config> Pallet<T> {
		// TODO Part III: helper functions for dispatchable functions
		pub fn mint(
			owner: &T::AccountId,
			dna: Option<[u8; 16]>,
			gender: Option<Gender>,
		) -> Result<T::Hash, Error<T>> {
			let kitty = Kitty::<T> {
				dna: dna.unwrap_or_else(Self::gen_dna),
				price: None,
				gender: gender.unwrap_or_else(Self::gen_gender),
				owner: owner.clone(),
			};

			let kitty_id = T::Hashing::hash_of(&kitty);

			let new_count = Self::kitty_count().checked_add(1).ok_or(<Error<T>>::KittyCountOverflow)?;

			<KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| kitty_vec.try_push(kitty_id))
				.map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

			<Kitties<T>>::insert(kitty_id, kitty);
			<kittyCount<T>>::put(new_count);

			Ok(kitty_id)
		}

		fn gen_dna() -> [u8; 16] {
			let payload =
				(T::KittyRandomness::random(&b"dna"[..]).0, <frame_system::Pallet<T>>::block_number());

			payload.using_encoded(blake2_128)
		}

		fn gen_gender() -> Gender {
			let random = T::KittyRandomness::random(&b"gender"[..]).0;

			match random.as_ref()[0] % 2 {
				0 => Gender::Male,
				_ => Gender::Female,
			}
		}

		// TODO: increment_nonce, random_hash, mint, transfer_from
	}
}
