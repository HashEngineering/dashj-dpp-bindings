use std::sync::Arc;
use dpp::identity::{IdentityPublicKey, KeyType, Purpose, SecurityLevel, TimestampMillis};
//use crate::BinaryDataWrapper;

pub struct IdentityPublicKeyWrapper {
    identity_public_key: IdentityPublicKey
}

impl IdentityPublicKeyWrapper {
    /// Is public key disabled
    pub fn is_disabled(&self) -> bool {
        self.identity_public_key.is_disabled()
    }
    /// Checks if public key security level is MASTER
    pub fn is_master(&self) -> bool {
        self.identity_public_key.is_master()
    }
    pub fn get_id(&self) -> u32 {
        self.identity_public_key.id as u32
    }
    pub fn get_purpose(&self) -> Purpose {
        self.identity_public_key.purpose
    }
    pub fn get_security_level(&self) -> SecurityLevel {
        self.identity_public_key.security_level
    }
    pub fn get_key_type(&self) -> KeyType {
        self.identity_public_key.key_type
    }
    pub fn get_read_only(&self) -> bool {
        self.identity_public_key.read_only
    }
    pub fn get_data(&self) -> Vec<u8> {//Arc<BinaryDataWrapper> {
        //Arc::new(self.identity_public_key.data.clone().into())
        self.identity_public_key.data.0.clone()
    }
    pub fn get_disabled_at(&self) -> Option<u64> {
        match self.identity_public_key.disabled_at {
            Some(timestamp) => Some(timestamp as u64),
            None => None
        }
    }
}

impl From<IdentityPublicKey> for IdentityPublicKeyWrapper {
    fn from(identity_public_key: IdentityPublicKey) -> Self {
        Self {
            identity_public_key,
        }
    }
}

impl Into<IdentityPublicKey> for IdentityPublicKeyWrapper {
    fn into(self) -> IdentityPublicKey {
        IdentityPublicKey {
            id: self.identity_public_key.id,
            purpose: self.identity_public_key.purpose,
            security_level: self.identity_public_key.security_level,
            key_type: self.identity_public_key.key_type,
            read_only: self.identity_public_key.read_only,
            data: self.identity_public_key.data,
            disabled_at: self.identity_public_key.disabled_at,
        }
    }
}