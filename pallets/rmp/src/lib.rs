//! # Rights Management Pallet

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use core::result::Result;

// use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, 
// 	sp_std::prelude::*, traits::EnsureOrigin};

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, 
	sp_std::prelude::*};
use frame_system::ensure_signed;
pub use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// General constraints to limit data size
pub const SONG_ID_MAX_LENGTH: usize = 36;
pub const SONG_NAME_MAX_LENGTH: usize = 20;
pub const ARTIST_NAME_MAX_LENGTH: usize = 10;
pub const COMPOSER_MAX_LENGTH: usize = 10;
pub const LYRICIST_MAX_LENGTH: usize = 10;
pub const YOR_MAX_LENGTH: usize = 4;
pub const SONG_MAX_PROPS: usize = 6;

pub trait Config: frame_system::Config + timestamp::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	// type CreateRoleOrigin: EnsureOrigin<Self::Origin>;
}

// Custom types
pub type SongId = Vec<u8>;
pub type SongName = Vec<u8>;
pub type AlbumTitle = Vec<u8>;
pub type ArtistName = Vec<u8>;
pub type Composer = Vec<u8>;
pub type Lyricist = Vec<u8>;
pub type YOR = Vec<u8>;
pub type Alias = Vec<u8>;
pub type IpfsHash = u64;


#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct MusicData<AccountId, Moment> {
    // The song ID would typically be a ISRC code (International Standard Recording Code),
    // or ISWC code (International Standard Musical Work Code), or similar.
    id: SongId,
    // This is account that represents the ownership of the music created.
    owner: AccountId,
    // This is a series of properties describing the music data.
    props: Option<Vec<MusicProperty>>,
    // Timestamp (approximate) at which the music was registered on-chain.
    registered: Moment,
}

// Contains a name-value pair for a music property
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct MusicProperty {
    name: SongName,
    artist: ArtistName,
	composer: Composer,
	lyricist: Lyricist,
	year: YOR,
}

pub struct ArtistAlias {
    artist: ArtistName,
    aliases: Alias
}

impl MusicProperty {
    pub fn new(name: &[u8], artist: &[u8], composer: &[u8], lyricist: &[u8], year: &[u8]) -> Self {
        Self {
            name: name.to_vec(),
            artist: artist.to_vec(),
			composer: composer.to_vec(),
			lyricist: lyricist.to_vec(),
			year: year.to_vec(),
        }
    }

    pub fn name(&self) -> &[u8] {
        self.name.as_ref()
    }

    pub fn artist(&self) -> &[u8] {
        self.artist.as_ref()
    }

	pub fn composer(&self) -> &[u8] {
        self.composer.as_ref()
    }

	pub fn lyricist(&self) -> &[u8] {
        self.lyricist.as_ref()
    }

	pub fn year(&self) -> &[u8] {
        self.year.as_ref()
    }
}

pub struct Album<AccountId, Moment> {
    owner: AccountId,
    album_artist: ArtistName,
    album_producer: Vec<u8>,
    album_title: Vec<u8>,
    album_type: Vec<u8>,
    c_line: Vec<u8>,
    country_of_origin: Vec<u8>,
    display_label_name: Vec<u8>,
    explicit_: bool,
    genre_1: u32,
    master_label_name: Vec<u8>,
    p_line: Vec<u8>,
    part_of_album: bool,
    release_date: Moment,
    sales_start_date: u32,
    upc_or_ean: bool
}

pub struct Comp<Moment> {
    pro: Vec<u8>,
    composition_title: Vec<u8>,
    publishers: Vec<Vec<u8>>,
    third_party_publishers: bool,
    writers: Vec<Vec<u8>>,
    created: Moment
}

pub mod distributions {
    pub struct DistributionsMaster {
        payee: super::ArtistName,
        bp: u32,
    }
    pub struct DistributionsComp {
        payee: super::Composer,
        bp: u32,
    }
}

pub struct Track{
    isrc: Vec<u8>, // meant to create an empty string
    genre_1: u32,
    genre_2: u32,
    p_line: Vec<u8>,
    samples: bool,
    track_artists: Vec<ArtistName>,
    zero: ArtistAlias,
    one: ArtistAlias,
    track_duration: u32,
    track_no: u32,
    track_producer: Vec<u8>,
    track_title: Vec<u8>,
    track_volume: u32,
    ipfs: IpfsHash,
    }


// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Config> as RightsMgmtPortal {
		pub MusicCollections get(fn music_by_id): map hasher(blake2_128_concat) SongId => Option<MusicData<T::AccountId, T::Moment>>;
        pub OwnerOf get(fn owner_of): map hasher(blake2_128_concat) SongId => Option<T::AccountId>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		MusicRegistered(AccountId, SongId, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		SongIdMissing,
        SongIdTooLong,
        SongIdExists,
        SongTooManyProps,
        SongInvalidSongName,
        SongInvalidArtistName,
		SongInvalidComposer,
		SongInvalidLyricist,
		SongInvalidYOR
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn register_music(origin, id: SongId, owner: T::AccountId, props: Option<Vec<MusicProperty>>) -> dispatch::DispatchResult {
            // T::CreateRoleOrigin::ensure_origin(origin.clone())?;
            let who = ensure_signed(origin)?;

            // Validate song ID
            Self::validate_song_id(&id)?;

            // Validate song props
            Self::validate_song_props(&props)?;

            // Check song doesn't exist yet (1 DB read)
            Self::validate_new_song(&id)?;

            // Create a song instance
            let song = Self::new_song()
                .identified_by(id.clone())
                .owned_by(owner.clone())
                .registered_on(<timestamp::Module<T>>::get())
                .with_props(props)
                .build();

            // Add product & ownerOf (2 DB writes)
            <MusicCollections<T>>::insert(&id, song);
            <OwnerOf<T>>::insert(&id, &owner);

            Self::deposit_event(RawEvent::MusicRegistered(who, id, owner));

            Ok(())
        }
	}
}

impl<T: Config> Module<T> {
    
    fn new_song() -> SongBuilder<T::AccountId, T::Moment> {
        SongBuilder::<T::AccountId, T::Moment>::default()
    }

    pub fn validate_song_id(id: &[u8]) -> Result<(), Error<T>> {
        // Basic song ID validation
        ensure!(!id.is_empty(), Error::<T>::SongIdMissing);
        ensure!(
            id.len() <= SONG_ID_MAX_LENGTH,
            Error::<T>::SongIdTooLong
        );
        Ok(())
    }

    pub fn validate_new_song(id: &[u8]) -> Result<(), Error<T>> {
        // Song existence check
        ensure!(
            !<MusicCollections<T>>::contains_key(id),
            Error::<T>::SongIdExists
        );
        Ok(())
    }

    pub fn validate_song_props(props: &Option<Vec<MusicProperty>>) -> Result<(), Error<T>> {
        if let Some(props) = props {
            ensure!(
                props.len() <= SONG_MAX_PROPS,
                Error::<T>::SongTooManyProps,
            );
            for prop in props {
                ensure!(
                    prop.name().len() <= SONG_NAME_MAX_LENGTH,
                    Error::<T>::SongInvalidSongName
                );
                ensure!(
                    prop.artist().len() <= ARTIST_NAME_MAX_LENGTH,
                    Error::<T>::SongInvalidArtistName
                );
				ensure!(
                    prop.composer().len() <= COMPOSER_MAX_LENGTH,
                    Error::<T>::SongInvalidComposer
                );
				ensure!(
                    prop.lyricist().len() <= LYRICIST_MAX_LENGTH,
                    Error::<T>::SongInvalidLyricist
                );
				ensure!(
                    prop.year().len() <= YOR_MAX_LENGTH,
                    Error::<T>::SongInvalidYOR
				);
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct SongBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    id: SongId,
    owner: AccountId,
    props: Option<Vec<MusicProperty>>,
    registered: Moment,
}

impl<AccountId, Moment> SongBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    pub fn identified_by(mut self, id: SongId) -> Self {
        self.id = id;
        self
    }

    pub fn owned_by(mut self, owner: AccountId) -> Self {
        self.owner = owner;
        self
    }

    pub fn with_props(mut self, props: Option<Vec<MusicProperty>>) -> Self {
        self.props = props;
        self
    }

    pub fn registered_on(mut self, registered: Moment) -> Self {
        self.registered = registered;
        self
    }

    pub fn build(self) -> MusicData<AccountId, Moment> {
        MusicData::<AccountId, Moment> {
            id: self.id,
            owner: self.owner,
            props: self.props,
            registered: self.registered,
        }
    }
}

