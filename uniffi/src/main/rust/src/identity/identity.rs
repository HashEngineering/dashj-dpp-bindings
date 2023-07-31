use std::cell::RefCell;
use std::sync::{Arc, atomic, RwLock};
use std::borrow::Borrow;
use dpp::identity::{Identity, IdentityPublicKey, KeyType, Purpose, SecurityLevel};
use dpp::prelude::Revision;
use dpp::ProtocolError;
use dpp::serialization_traits::PlatformDeserializable;
use crate::identifier::identifier::IdentifierWrapper;
use crate::errors::protocol_error::ProtocolErrorType;
use core::sync::atomic::AtomicPtr;
use crate::IdentityPublicKeyWrapper;

pub struct IdentityWrapper {
    pub identity: Identity,
}

impl IdentityWrapper {
    pub fn deserialize(bytes: Vec<u8>) -> IdentityWrapper {
        IdentityWrapper { identity: <Identity as PlatformDeserializable>::deserialize(bytes.as_slice()).unwrap() }
    }
    pub fn get_protocol_version(&self) -> u32 {
        self.identity.get_protocol_version()
    }
    pub fn get_id(&self) -> Arc<IdentifierWrapper> {
        Arc::new(IdentifierWrapper { identifier: self.identity.get_id().clone() })
    }

    /// Returns balance
    pub fn get_balance(&self) -> u64 {
        self.identity.get_balance()
    }

    /// Set Identity balance
    pub fn set_balance(&mut self, balance: u64) {
        self.identity.set_balance(balance);
    }

    /// Increase Identity balance
    pub fn increase_balance(&mut self, amount: u64) -> u64 {
        self.identity.increase_balance(amount)
    }

    /// Reduce the Identity balance
    pub fn reduce_balance(&mut self, amount: u64) -> u64 {
        self.identity.reduce_balance(amount)
    }

    /// Get Identity revision
    pub fn get_revision(&self) -> Revision {
        self.identity.get_revision()
    }
    pub fn get_public_key_max_id(&self) -> u32 {
        self.identity.get_public_key_max_id() as u32
    }

    /// Returns a public key for a given id
    pub fn get_public_key_by_id(&self, key_id: u32) -> Option<Arc<IdentityPublicKeyWrapper>> {
        match self.identity.get_public_key_by_id(key_id) {
            Some(identity_public_key) => Some(Arc::new(identity_public_key.clone().into())),
            None => None
        }
    }

    /// Increment revision
    pub fn increment_revision(&mut self) -> Result<(), ProtocolErrorType> {
        match self.identity.increment_revision() {
            Ok(..) => Ok(()),
            Err(error) => Err(ProtocolErrorType::ConsensusError)
        }
    }
}