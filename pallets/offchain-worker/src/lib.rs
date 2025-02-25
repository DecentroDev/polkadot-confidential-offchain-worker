#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
    self as system,
    offchain::{AppCrypto, CreateSignedTransaction},
    pallet_prelude::BlockNumberFor,
};
use sp_core::crypto::KeyTypeId;

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
pub mod crypto {
    use super::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };
    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for TestAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, DefaultNoBound};
    use frame_system::pallet_prelude::*;
    use offchain_utils::{
        offchain_api_key::OffchainApiKey, DefaultOffchainFetcher, HttpRequest, OffchainFetcher,
    };
    use scale_info::prelude::string::String;

    pub struct CustomApiKeyFetcher;
    impl OffchainApiKey for CustomApiKeyFetcher {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// A struct to store a single block-number. Has all the right derives to store it in storage.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_storage_derives/index.html>
    #[derive(
        Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct CompositeStruct<T: Config> {
        /// A block number.
        pub(crate) block_number: BlockNumberFor<T>,
    }

    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
        /// The identifier type for an offchain worker.
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// We usually use passive tense for events.
        SomethingStored {
            block_number: BlockNumberFor<T>,
            who: T::AccountId,
        },
    }

    #[pallet::storage]
    pub type Something<T: Config> = StorageValue<_, CompositeStruct<T>>;

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn do_something(origin: OriginFor<T>, bn: u32) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_origin/index.html>
            let who = ensure_signed(origin)?;
            // Convert the u32 into a block number. This is possible because the set of trait bounds
            // defined in [`frame_system::Config::BlockNumber`].
            let block_number: BlockNumberFor<T> = bn.into();

            // Update storage.
            <Something<T>>::put(CompositeStruct { block_number });

            // Emit an event.
            Self::deposit_event(Event::SomethingStored { block_number, who });

            // Return a successful [`DispatchResultWithPostInfo`] or [`DispatchResult`].
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn get_api_key() -> Result<String, &'static str> {
            match CustomApiKeyFetcher::fetch_api_key_for_request("api_key") {
                Ok(key) => Ok(key),
                Err(err) => {
                    log::error!("Failed to fetch API key: {:?}", err); // Use {:?} for Debug representation
                    Err("API key not found in offchain storage")
                }
            }
        }
    }
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Offchain Worker entry point.
        ///
        /// By implementing `fn offchain_worker` you declare a new offchain worker.
        /// This function will be called when the node is fully synced and a new best block is
        /// successfully imported.
        /// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
        /// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
        /// so the code should be able to handle that.
        /// You can use `Local Storage` API to coordinate runs of the worker.
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            // Note that having logs compiled to WASM may cause the size of the blob to increase
            // significantly. You can use `RuntimeDebug` custom derive to hide details of the types
            // in WASM. The `sp-api` crate also provides a feature `disable-logging` to disable
            // all logging and thus, remove any logging from the WASM.
            log::trace!(target: "logger", "Ping from offchain workers!");

            // Attempt to retrieve the API key
            let api_key = match Self::get_api_key() {
                Ok(api_key) => {
                    log::info!("API key retrieved: {}", api_key);
                    api_key
                }
                Err(e) => {
                    log::error!("Offchain worker error: {:?}", e); // Use {:?} for Debug representation
                    return; // Exit early if API key is missing
                }
            };

            let url = "https://postman-echo.com/basic-auth";

            let request = HttpRequest::new(url).add_header("Authorization", &api_key);

            let response = DefaultOffchainFetcher::fetch_string(request);

            match response {
                Ok(response) => {
                    log::info!("Response: {}", response);
                }
                Err(e) => {
                    log::error!("Failed to fetch data: {:?}", e);
                }
            }

            // Since off-chain workers are just part of the runtime code, they have direct access
            // to the storage and other included pallets.
            //
            // We can easily import `frame_system` and retrieve a block hash of the parent block.
            let parent_hash = <system::Pallet<T>>::block_hash(block_number - 1u32.into());
            log::debug!(
                "Current block: {:?} (parent hash: {:?})",
                block_number,
                parent_hash
            );
        }
    }
}
