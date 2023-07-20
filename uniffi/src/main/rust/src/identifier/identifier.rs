use dpp::platform_value::string_encoding::Encoding;
use crate::Identifier;

pub struct IdentifierWrapper {
    pub identifier: Identifier
}

impl IdentifierWrapper {
    pub fn new(bytes: Vec<u8>) -> Self {
        IdentifierWrapper { identifier: Identifier::from_bytes(bytes.as_slice()).unwrap() }
    }
    pub fn random() -> Self {
        IdentifierWrapper { identifier: Identifier::random() }
    }

    pub fn from_string_with_encoding_string(encoded_value: String,
                                        encoding_string: String) -> IdentifierWrapper {
        IdentifierWrapper { identifier: Identifier::from_string_with_encoding_string(&*encoded_value, Some(&*encoding_string)).unwrap().into() }
    }

    pub fn to_base58(&self) -> String {
        self.identifier.to_string(Encoding::Base58)
    }

    //fn to_string_with_encoding(self, encoding_string: String) {
    //    self.identifier.to_string(encoding_string.into())
    //}
}

impl From<Identifier> for IdentifierWrapper {
    fn from(identifier: Identifier) -> Self {
        Self {
            identifier,  // .0 to access the first (and only) field of the tuple struct
        }
    }
}

impl Into<Identifier> for IdentifierWrapper {
    fn into(self) -> Identifier {
        Identifier(self.identifier.0)  // no .0 needed here, because you're constructing a new BinaryData
    }
}