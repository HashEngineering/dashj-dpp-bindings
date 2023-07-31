mod identity;
mod identifier;
mod errors;
uniffi::include_scaffolding!("dpp");
//use std::convert::identity;
use std::sync::Arc;
//use serde::{Deserialize, Serialize};

use dpp::version::LATEST_VERSION;
pub use dpp::data_contract::generate_data_contract_id;

pub use dpp::contracts::SystemIDs;
pub use dpp::contracts::dashpay_contract::system_ids;
//use dpp::dashcore::hashes::serde::Deserialize;
pub use dpp::prelude::Identifier;
pub use dpp::identity::key_type::KeyType;
pub use dpp::identity::purpose::Purpose;
pub use dpp::identity::security_level::SecurityLevel;
pub use dpp::platform_value::BinaryData;
pub use dpp::identity::IdentityPublicKey;
pub use dpp::identity::Identity;
pub use dpp::NativeBlsModule;
pub use dpp::errors::protocol_error::ProtocolError;
use dpp::identity::KeyID;
use dpp::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;
use dpp::identity::state_transition::asset_lock_proof::instant::{
    InstantAssetLockProof//, RawInstantLock
};
pub use dpp::identity::state_transition::identity_create_transition::IdentityCreateTransition;
pub use dpp::identity::state_transition::identity_public_key_transitions::IdentityPublicKeyInCreation;
//use dpp::platform_value::string_encoding::Encoding;
use dpp::prelude::AssetLockProof;
pub use dpp::state_transition::StateTransitionType;
//use dpp::serialization_traits::{PlatformDeserializable, PlatformSerializable, Signable};

pub fn latest_protocol_version() -> u32 {
    LATEST_VERSION
}

// pub struct AssetLockProofWrapper {
//     pub instant: Option<InstantAssetLockProof>,
//     pub chain: Option<ChainAssetLockProof>,
// }

pub struct AssetLockProofWrapper {
    pub instant: Option<Arc<InstantAssetLockProof>>,
    pub chain: Option<Arc<ChainAssetLockProof>>,
}

impl From<AssetLockProof> for AssetLockProofWrapper {
    fn from(proof: AssetLockProof) -> Self {
        match proof {
            AssetLockProof::Instant(instant) => Self {
                instant: Some(Arc::new(instant)),
                chain: None,
            },
            AssetLockProof::Chain(chain) => Self {
                instant: None,
                chain: Some(Arc::new(chain)),
            },
        }
    }
}

impl Into<AssetLockProof> for AssetLockProofWrapper {
    fn into(self) -> AssetLockProof {
        match self {
            Self {
                instant: Some(instant),
                chain: None,
            } => AssetLockProof::Instant(Arc::try_unwrap(instant).unwrap_or_else(|_| panic!("More than one strong reference exists"))),
            Self {
                instant: None,
                chain: Some(chain),
            } => AssetLockProof::Chain(Arc::try_unwrap(chain).unwrap_or_else(|_| panic!("More than one strong reference exists"))),
            _ => panic!("Invalid AssetLockProofWrapper state"),
        }
    }
}
pub use errors::protocol_error::ProtocolErrorType;
pub use identifier::identifier::IdentifierWrapper;
pub use identity::identity::IdentityWrapper;
pub use crate::identity::identity_public_key::IdentityPublicKeyWrapper;
pub use dpp::prelude::Revision;
// The new struct that matches the Uniffi dictionary

pub struct BinaryDataWrapper {
    pub data: Vec<u8>,
}

// Implement conversions between BinaryData and BinaryDataWrapper
impl From<BinaryData> for BinaryDataWrapper {
    fn from(binary_data: BinaryData) -> Self {
        Self {
            data: binary_data.0,  // .0 to access the first (and only) field of the tuple struct
        }
    }
}

impl Into<BinaryData> for BinaryDataWrapper {
    fn into(self) -> BinaryData {
        BinaryData(self.data)  // no .0 needed here, because you're constructing a new BinaryData
    }
}



//#[derive(Debug, Serialize, Deserialize, Encode, Decode, PlatformSignable, Clone, PartialEq, Eq)]
pub struct IdentityPublicKeyInCreationWrapper {
    pub id: KeyID,
    //#[serde(rename = "type")]
    pub key_type: KeyType,
    pub purpose: Purpose,
    pub security_level: SecurityLevel,
    pub read_only: bool,
    pub data: BinaryDataWrapper,
    /// The signature is needed for ECDSA_SECP256K1 Key type and BLS12_381 Key type
    //#[platform_signable(exclude_from_sig_hash)]
    pub signature: BinaryDataWrapper,
}

impl From<IdentityPublicKeyInCreationWrapper> for IdentityPublicKeyInCreation {
    fn from(wrapper: IdentityPublicKeyInCreationWrapper) -> Self {
        let IdentityPublicKeyInCreationWrapper {
            id,
            key_type,
            purpose,
            security_level,
            read_only,
            data,
            signature,
        } = wrapper;

        let data = data.into();
        let signature = signature.into();

        IdentityPublicKeyInCreation {
            id,
            key_type,
            purpose,
            security_level,
            read_only,
            data,
            signature,
        }
    }
}

impl From<IdentityPublicKeyInCreation> for IdentityPublicKeyInCreationWrapper {
    fn from(public_key: IdentityPublicKeyInCreation) -> Self {
        let IdentityPublicKeyInCreation {
            id,
            key_type,
            purpose,
            security_level,
            read_only,
            data,
            signature,
        } = public_key;

        let data = data.into();
        let signature = signature.into();

        IdentityPublicKeyInCreationWrapper {
            id,
            key_type,
            purpose,
            security_level,
            read_only,
            data,
            signature,
        }
    }
}

