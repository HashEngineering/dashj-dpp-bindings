use std::sync::Arc;
use dpp::identity::{Identity, KeyType, Purpose, SecurityLevel};
use dpp::prelude::Revision;
use dpp::serialization_traits::PlatformDeserializable;
use crate::identifier::identifier::IdentifierWrapper;
use crate::errors::protocol_error::ProtocolErrorWrapper;

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
        self.set_balance(balance);
    }

    /// Increase Identity balance
    pub fn increase_balance(&mut self, amount: u64) -> u64 {
        self.increase_balance(amount)
    }

    /// Reduce the Identity balance
    pub fn reduce_balance(&mut self, amount: u64) -> u64 {
        self.reduce_balance(amount)
    }

    /// Get Identity revision
    pub fn get_revision(&self) -> Revision {
        self.get_revision()
    }

    /// Increment revision
    pub fn increment_revision(&mut self) -> Result<(), ProtocolErrorWrapper> {
        match self.increment_revision() {
            Ok(..) => Ok(()),
            Err(error) => Err(error.into())
        }
    }
}