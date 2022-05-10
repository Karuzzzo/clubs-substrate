#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    
    // TODO move to Config trait
    type ClubsInfo = u32;
    type ClubId = u32;
	
    /// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
    
    #[pallet::type_value]
    pub fn DefaultTotalClubs<T: Config>() -> u8 { 0u8 }

    #[pallet::storage]
	#[pallet::getter(fn total_clubs)]
    pub type TotalClubs<T> = StorageValue<_, u8, ValueQuery, DefaultTotalClubs<T>>;

	// List of users and their privileges
    #[pallet::storage]
	#[pallet::getter(fn users)]
    pub type Users<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ClubsInfo, ValueQuery>;
	
    // This storage is unnecessary and can be removed
    // Can be used in case we need to store some stuff in
    #[pallet::storage]
    #[pallet::getter(fn clubs)]
    pub type Clubs<T:Config> = StorageMap<_, Blake2_128Concat, ClubId, u8, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
        UserAddedTo(T::AccountId, ClubId),
        UserRemovedFrom(T::AccountId, ClubId),
        NewClub(ClubId)

    }

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
        InvalidOrigin,
        InvalidClub,
        ClubAlreadySet,
        IndexOutOfBounds
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn add_club(origin: OriginFor<T>, identifier: u8) -> DispatchResult {
            ensure_root(origin)?;
            let clubs_total: u8 = Self::total_clubs();
            ensure!(clubs_total <= 32, Error::<T>::IndexOutOfBounds);
            // Bitwise operation
            let club_id: ClubId = Self::number_to_id(clubs_total);
            ensure!(!Clubs::<T>::contains_key(club_id), Error::<T>::InvalidClub);
            
            // TODO This storage not necessary rn
            Clubs::<T>::insert(&club_id, identifier);
            TotalClubs::<T>::put(clubs_total + 1);
            Self::deposit_event(Event::NewClub(club_id));
            Ok(())
        }

        // Club id here should be bitmask 000100000 or something alike 
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn assign_to_club(origin: OriginFor<T>, user: T::AccountId, club_number: u8) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(club_number <= 32, Error::<T>::IndexOutOfBounds);

            let club_id: ClubId = Self::number_to_id(club_number);
            ensure!(Clubs::<T>::contains_key(club_id), Error::<T>::InvalidClub);
            let user_info: u32 = <Users<T>>::get(user.clone());
            ensure!(user_info & club_id == 0, Error::<T>::ClubAlreadySet);
            
            <Users<T>>::insert(user.clone(), user_info | club_id);
            Self::deposit_event(Event::UserAddedTo(user, club_id));
            Ok(())
        }

        // Club id here should be bitmask 000100000 or something alike 
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn remove_from_club(origin: OriginFor<T>, user: T::AccountId, club_number: u8) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(club_number <= 32, Error::<T>::IndexOutOfBounds);

            let club_id: ClubId = Self::number_to_id(club_number);
            ensure!(Clubs::<T>::contains_key(club_id), Error::<T>::InvalidClub);
            let user_info: u32 = Users::<T>::get(user.clone());
            ensure!(user_info & club_id != 0, Error::<T>::ClubAlreadySet);
            
            <Users<T>>::insert(user.clone(), user_info & !club_id);
            Self::deposit_event(Event::UserRemovedFrom(user, club_id));
            Ok(())
        }
	}

    impl<T: Config> Pallet<T> {
		pub fn number_to_id(number: u8) -> ClubId {
                2_u32.pow(number.into())
        }
    }
}
