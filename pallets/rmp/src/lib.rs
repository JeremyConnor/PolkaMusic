//! # Rights Management Pallet

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use core::result::Result;
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, 
 	sp_std::prelude::*};
use frame_system::ensure_signed;
pub use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// General constraints to limit data size
pub const SRC_ID_MAX_LENGTH: usize = 36;
pub const SONG_ID_MAX_LENGTH: usize = 36;
pub const SONG_NAME_MAX_LENGTH: usize = 20;
pub const ARTIST_NAME_MAX_LENGTH: usize = 20;
pub const COMPOSER_MAX_LENGTH: usize = 20;
pub const LYRICIST_MAX_LENGTH: usize = 20;
pub const YOR_MAX_LENGTH: usize = 4;
pub const SONG_MAX_PROPS: usize = 6;

pub trait Config: frame_system::Config + timestamp::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// Custom types
pub type SrcId = Vec<u8>;
pub type SongId = Vec<u8>;
pub type SongName = Vec<u8>;
pub type AlbumTitle = Vec<u8>;
pub type ArtistName = Vec<u8>;
pub type Composer = Vec<u8>;
pub type Lyricist = Vec<u8>;
pub type YOR = Vec<u8>;
pub type Alias = Vec<u8>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct MusicData<AccountId, Moment> {
    // music file hash
    src_id: SrcId,
    
    // This is account that represents the ownership of the music created.
    owner: AccountId,

    // The song ID would typically be a ISRC code (International Standard Recording Code),
    // or ISWC code (International Standard Musical Work Code), or similar.
    song_id: Option<SongId>,

    // Timestamp (approximate) at which the music was registered on-chain.
    registered: Moment,

    // This is a series of properties describing the music test data.
    props: Option<Vec<TestData>>,
    // album: Option<Vec<Album<Moment>>>,
    // track: Option<Vec<Track>>,
    // artist_alias: Option<Vec<ArtistAlias>>,
    // comp: Option<Vec<Comp>>,
    // distributions_comp: Option<Vec<DistributionsComp>>,
    // distributions_master: Option<Vec<DistributionsMaster>>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct TestData {
    name: SongName,
    artist: ArtistName,
	composer: Composer,
	lyricist: Lyricist,
	year: YOR,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct ArtistAlias {
    artist: ArtistName,
    aliases: Alias
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Album<Moment> {
    album_artist: ArtistName,
    album_producer: Vec<u16>,
    album_title: Vec<u16>,
    album_type: Vec<u16>,
    c_line: Vec<u16>,
    country_of_origin: Vec<u8>,
    display_label_name: Vec<u16>,
    explicit_: bool,
    genre_1: u32,
    master_label_name: Vec<u16>,
    p_line: Vec<u16>,
    part_of_album: bool,
    release_date: Moment,
    sales_start_date: Vec<u16>,
    upc_or_ean: bool
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Track {
    track_no: u32,
    track_producer: Vec<u8>,
    track_title: Vec<u8>,
    track_volume: u32,
    track_duration: Vec<u32>,
    genre_1: Vec<u8>,
    genre_2: Vec<u8>,
    p_line: Vec<u8>,
    samples: bool,
    track_artists: Vec<ArtistName>,
    zero: ArtistAlias,
    one: ArtistAlias,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Comp<Moment> {
    pro: Vec<u16>,
    composition_title: Vec<u16>,
    publishers: Vec<Vec<u16>>,
    third_party_publishers: bool,
    writers: Vec<Vec<u16>>,
    created: Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct DistributionsMaster {
	payee: ArtistName,
	bp: u32,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct DistributionsComp {
	payee: Composer,
	bp: u32,
}

impl TestData {
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

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Config> as RightsMgmtPallet {
		pub MusicCollections get(fn music_by_src_id): map hasher(blake2_128_concat) SrcId => Option<MusicData<T::AccountId, T::Moment>>;
        pub SrcCollections get(fn products_of_org): map hasher(blake2_128_concat) T::AccountId => Vec<SrcId>;
        pub OwnerOf get(fn owner_of): map hasher(blake2_128_concat) SrcId => Option<T::AccountId>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		SrcCreated(AccountId, SrcId, SongId, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
        SrcIdMissing,
        SrcIdTooLong,
        SrcIdExists,
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
		pub fn register_music(origin, src_id: SrcId, song_id: SongId, owner: T::AccountId, props: Option<Vec<TestData>>) -> dispatch::DispatchResult {
            
            let who = ensure_signed(origin)?;

            // Validate music file hash
            Self::validate_src_id(&src_id)?;

            // Validate song ID
            Self::validate_song_id(&song_id)?;

            // Validate song props
            Self::validate_song_props(&props)?;

            // Check SRC doesn't exist yet (1 DB read)
            Self::validate_new_src_id(&src_id)?;

            // Create a song instance
            let song = Self::new_song()
                .verified_by(src_id.clone())
                .identified_by(Some(song_id.clone()))
                .owned_by(owner.clone())
                .registered_on(<timestamp::Module<T>>::get())
                .with_props(props)
                .build();

            <MusicCollections<T>>::insert(&src_id, song);
            <SrcCollections<T>>::append(&owner, &src_id);
            <OwnerOf<T>>::insert(&src_id, &owner);

            Self::deposit_event(RawEvent::SrcCreated(who, src_id, song_id, owner));

            Ok(())
        }
	}
}

impl<T: Config> Module<T> {
    
    fn new_song() -> SongBuilder<T::AccountId, T::Moment> {
        SongBuilder::<T::AccountId, T::Moment>::default()
    }

    pub fn validate_src_id(src_id: &[u8]) -> Result<(), Error<T>> {
        // File Hash validation
        ensure!(!src_id.is_empty(), Error::<T>::SrcIdMissing);
        ensure!(
            src_id.len() <= SRC_ID_MAX_LENGTH,
            Error::<T>::SrcIdTooLong
        );
        Ok(())
    }

    pub fn validate_song_id(song_id: &[u8]) -> Result<(), Error<T>> {
        // Basic song ID validation
        ensure!(!song_id.is_empty(), Error::<T>::SongIdMissing);
        ensure!(
            song_id.len() <= SONG_ID_MAX_LENGTH,
            Error::<T>::SongIdTooLong
        );
        Ok(())
    }

    pub fn validate_new_src_id(src_id: &[u8]) -> Result<(), Error<T>> {
        // SRC existence check
        ensure!(
            !<MusicCollections<T>>::contains_key(src_id),
            Error::<T>::SrcIdExists
        );
        Ok(())
    }


    pub fn validate_song_props(props: &Option<Vec<TestData>>) -> Result<(), Error<T>> {
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
    src_id: SrcId,
    song_id: Option<SongId>,
    owner: AccountId,
    props: Option<Vec<TestData>>,
    // album: Option<Vec<Album<Moment>>>,
    // track: Option<Vec<Track>>,
    registered: Moment,
}

impl<AccountId, Moment> SongBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{   
    pub fn verified_by(mut self, src_id: SrcId) -> Self {
        self.src_id = src_id;
        self
    }
    pub fn identified_by(mut self, song_id: Option<SongId>) -> Self {
        self.song_id = song_id;
        self
    }

    pub fn owned_by(mut self, owner: AccountId) -> Self {
        self.owner = owner;
        self
    }

    pub fn with_props(mut self, props: Option<Vec<TestData>>) -> Self {
        self.props = props;
        self
    }

    // pub fn with_album(mut self, album: Option<Vec<Album<Moment>>>) -> Self {
    //     self.album = album;
    //     self
    // }

    // pub fn with_track(mut self, track: Option<Vec<Track>>) -> Self {
    //     self.track = track;
    //     self
    // }
    
    pub fn registered_on(mut self, registered: Moment) -> Self {
        self.registered = registered;
        self
    }

    pub fn build(self) -> MusicData<AccountId, Moment> {
        MusicData::<AccountId, Moment> {
            song_id: self.song_id,
            src_id: self.src_id,
            owner: self.owner,
            props: self.props,
            // album: self.album,
            // track: self.track,
            registered: self.registered,
        }
    }
}

