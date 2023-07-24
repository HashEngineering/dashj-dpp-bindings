use std::fmt;
use dpp::ProtocolError;

#[derive(Debug, thiserror::Error)]
pub enum ProtocolErrorType {
    IdentifierError,
    StringDecodeError,
    EmptyPublicKeyDataError,
    MaxEncodedBytesReachedError,
    EncodingError,
    DecodingError,
    FileNotFound,
    UnknownProtocolVersionError,
    NoProtocolVersionError,
    ParsingError,
    ParsingJsonError,
    Error,
    DataContractError,
    StateTransitionError,
    StructureError,
    ConsensusError,
    DocumentError,
    Generic,
    InvalidIdentityPublicKeyTypeError,
    StateTransitionIsNotSignedError,
    PublicKeySecurityLevelNotMetError,
    WrongPublicKeyPurposeError,
    PublicKeyMismatchError,
    InvalidSignaturePublicKeyError,
    NonConsensusError,
    CompatibleProtocolVersionIsNotDefinedError,
    DataContractAlreadyExistsError,
    InvalidDataContractError,
    InvalidDocumentTypeError,
    DataContractNotPresentError,
    InvalidSignaturePublicKeySecurityLevelError,
    InvalidStateTransitionTypeError,
    MissingDataContractIdError,
    PublicKeyIsDisabledError,
    IdentityNotPresentError,
    Overflow,
    DocumentKeyMissing,
    ValueError,
    PlatformSerializationError,
    PlatformDeserializationError,
    DashCoreError,
    InvalidIdentityError,
    PublicKeyGenerationError,
    CorruptedCodeExecution,
}

pub struct ProtocolErrorWrapper {
    //error_type: ProtocolErrorType,
    message: String
}

impl ProtocolErrorWrapper {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl From<ProtocolError> for ProtocolErrorWrapper {
    fn from(error: ProtocolError) -> Self {
        ProtocolErrorWrapper {
            //error_type: IdentifierError,
            message: error.to_string()
        }
    }
}

impl fmt::Display for ProtocolErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Error => write!(f, "Variant a"),
            _ => write!(f, "Variant 1")
        }
    }
}