uniffi::include_scaffolding!("dpp");
use dpp::version::LATEST_VERSION;
pub use dpp::data_contract::generate_data_contract_id;

pub use dpp::contracts::SystemIDs;
pub use dpp::contracts::dashpay_contract::system_ids;
pub use dpp::prelude::Identifier;
pub use dpp::identity::key_type::KeyType;
pub use dpp::identity::purpose::Purpose;
pub use dpp::identity::security_level::SecurityLevel;
pub use dpp::platform_value::BinaryData;
pub use dpp::identity::IdentityPublicKey;
pub use dpp::identity::Identity;
pub use dpp::NativeBlsModule;
pub use dpp::errors::protocol_error::ProtocolError;
pub use dpp::identity::state_transition::identity_create_transition::IdentityCreateTransition;

pub fn latest_protocol_version() -> u32 {
    LATEST_VERSION
}
