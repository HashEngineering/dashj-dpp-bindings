use std::convert::Infallible;
use hex::{encode, decode};
use std::sync::Arc;
use dpp::identity::{KeyID, validation::PublicKeysValidator};
use dpp::identity::factory::IdentityFactory;
use dpp::{Convertible, ProtocolError};
use dpp::dashcore::anyhow::Result;
//use dpp::dashcore::hashes::serde::Deserialize;
use dpp::dashcore::InstantLock;
use dpp::data_contract::DataContract;
use dpp::document::{Document, ExtendedDocument};
use dpp::identity::state_transition::identity_create_transition::IdentityCreateTransition;
use dpp::identity::validation::{IdentityValidator, PUBLIC_KEY_SCHEMA_FOR_TRANSITION, RequiredPurposeAndSecurityLevelValidator};
use dpp::identity::Identity;
use dpp::identity::state_transition::asset_lock_proof::{AssetLockProofValidator, AssetLockTransactionValidator, ChainAssetLockProofStructureValidator, InstantAssetLockProofStructureValidator};
use dpp::identity::state_transition::identity_create_transition::validation::basic::IdentityCreateTransitionBasicValidator;
use dpp::identity::state_transition::validate_public_key_signatures::TPublicKeysSignaturesValidator;
use platform_value::{BinaryData, Identifier, Value};
use platform_value::string_encoding::Encoding;
use platform_value::platform_value;
use dpp::version::{ProtocolVersionValidator, LATEST_VERSION, COMPATIBILITY_MAP};
use dpp::NativeBlsModule;
use dpp::prelude::{IdentityPublicKey, Revision, TimestampMillis};
use dpp::serialization_traits::{PlatformDeserializable, PlatformSerializable, Signable};
use dpp::state_repository::{FetchTransactionResponse, StateRepositoryLike};
use dpp::state_transition::StateTransition::IdentityCreate;
use dpp::state_transition::StateTransitionConvert;
use dpp::state_transition::state_transition_execution_context::StateTransitionExecutionContext;
use dpp::validation::{SimpleConsensusValidationResult, ValidationResult};
use crate::state_repository::MyStateRepository;

pub fn create_identity_create_transition(_key1: Vec<u8>, _key2: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
    let raw_identity_transition = platform_value!({
        "assetLockProof": {
            "type": 0u64,
            "instantLock":
            decode("01011DBBDA5861B12D7523F20AA5E0D42F52DE3DCD2D5C2FE919BA67B59F050D206E00000000EB1F668BC2821C0D001AE3B5740E8485ACB028C591A6C3C2A137774BE9553B782E02C76C7E57779AFD9942F983AFBFE2F1E0DD07CAB57A75A776B062DFD0C80D000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            "transaction": decode("03000000018C6A321936A4DA0649AA8EEB2DA3280C10088C057A22BAF2BF987951924E8CC8000000006B483045022100913A974DFAE27128F7311ABE69969238B1ECF351F4873BE8C82B97AE438B1E7202203ACEFDEDB7E588A338E12141289760D41C355633C9645459AE5AC9FB5EF9F1F60121027D916607B92BAC0A1527AA126ADACBDC24E749989ADC0CD2C62DFC0A0D4035E3FFFFFFFF0140420F0000000000166A1494DEAED081B3C2FE239F4E2880F94D7B9B3E456E00000000").unwrap(),
            "outputIndex": 0
        },
        "publicKeys": [
            {
                "id": 0,
                "purpose": 0,
                "securityLevel": 0,
                "type": 0,
                "data": decode("036E6F84B4B4525605F819FBA38388E7E088606663AFA789730413664C71D0ECC2").unwrap(),
                "readOnly": false,
                "signature":
                decode("1F0AEA53BDCB687A0AE763E3D638529F9AC751AD6D038D71691EA3BE650B9120B2730DBC21EB5D508FC9323C0823850701E65DFD7389147FAD1382EA3D6F6F1A85").unwrap()
            },
            {
                "id": 1,
                "purpose": 0,
                "securityLevel": 2,
                "type": 0,
                "data": decode("025CEFCE88EE5484256814FAD4619DDF6B845E6CD2429AF0115115F1BB87169A26").unwrap(),
                "readOnly": false,
                "signature": decode("1FD0A4618BC9D054A7D0089E5723E8DC392E269837D72893AA66726E63BA8E7FDD43EFA5E87A8889EC4786C2A77F8C7BD3FF6425B788BEBA616846FFC4D6CBED21").unwrap()
            }
        ],
        "signature": decode("208231FAAFD1394B2B1B4DB6C25B5613203485594C36E424C547A47B5F3DBE07C978B44B82CC03E71928243A6448BADC201D0115F96B950928576AF22DCFDC97F9").unwrap(),
        "type": 2
    });

    let identity_create_transition = IdentityCreateTransition::from_raw_object(raw_identity_transition);


    identity_create_transition.unwrap().to_cbor_buffer(false)
}

pub fn identity_create_transition_signable_bytes_from_raw_object(raw_identity_transition: Value) -> Result<Vec<u8>, ProtocolError> {
    let result = IdentityCreateTransition::from_raw_object(raw_identity_transition);
    match result {
        Ok(identity_create_transition) => identity_create_transition.signable_bytes(),
        Err(error) => Result::Err(error)
    }
}

pub fn identity_create_transition_serialize_from_raw_object(raw_identity_transition: Value) -> Result<Vec<u8>, ProtocolError> {
    let result = IdentityCreateTransition::from_raw_object(raw_identity_transition);
    match result {
        Ok(identity_create_transition) => identity_create_transition.serialize(),
        Err(error) => Result::Err(error)
    }
}

pub fn identity_create_transition_cbor_from_raw_object(raw_identity_transition: Value) -> Result<Vec<u8>, ProtocolError> {
    let result = IdentityCreateTransition::from_raw_object(raw_identity_transition);
    match result {
        Ok(identity_create_transition) => identity_create_transition.to_cbor_buffer(false),
        Err(error) => Result::Err(error)
    }
}

pub fn create_identity_cbor() -> Result<Vec<u8>, ProtocolError> {
    // can we use DashPlatformProtocol?
    let protocol_version_validator = Arc::new(ProtocolVersionValidator::new(
        LATEST_VERSION,
        LATEST_VERSION,
        COMPATIBILITY_MAP.clone(),
    ));

    let bls_validator = NativeBlsModule::default();
    let public_keys_validator = Arc::new(PublicKeysValidator::new(bls_validator)?);


    let identity_validator = Arc::new(IdentityValidator::new(
        protocol_version_validator,
        public_keys_validator,
    )?);

    let factory: IdentityFactory<NativeBlsModule> = IdentityFactory::new(LATEST_VERSION, Arc::new(Arc::<IdentityValidator<PublicKeysValidator<NativeBlsModule>>>::into_inner(identity_validator).unwrap()));
    //let factory: IdentityFacade<NativeBlsModule> = IdentityFacade::new(LATEST_VERSION, protocol_version_validator, public_keys_validator);


    let raw_identity = platform_value!({
        "protocolVersion": 1u32,
        "id": Identifier::from([198, 23, 40, 120, 58, 93, 0, 165, 27, 49, 4, 117, 107, 204,  67, 46, 164, 216, 230, 135, 201, 92, 31, 155, 62, 131, 211, 177, 139, 175, 163, 237]),
        "publicKeys": [
            {
                "id": 0u32,
                "type": 0u8,
                "purpose": 0u8,
                "securityLevel": 0u8,
                "data": BinaryData::from_string("AuryIuMtRrl/VviQuyLD1l4nmxi9ogPzC9LT7tdpo0di", Encoding::Base64).unwrap(),
                "readOnly": false
            },
            {
                "id": 1u32,
                "type": 0u8,
                "purpose": 1u8,
                "securityLevel": 3u8,
                "data": BinaryData::from_string("A8AK95PYMVX5VQKzOhcVQRCUbc9pyg3RiL7jttEMDU+L", Encoding::Base64).unwrap(),
                "readOnly": false
            }
        ],
        "balance": 10u64,
        "revision": 0u64
    });

    let identity = factory.create_from_object(
        raw_identity, false,
    ).unwrap();

    identity.to_cbor_buffer()
}

pub fn create_identity_from_raw_object(raw_identity: Value) -> Result<Vec<u8>, ProtocolError> {
// can we use DashPlatformProtocol?
    let protocol_version_validator = Arc::new(ProtocolVersionValidator::new(
        LATEST_VERSION,
        LATEST_VERSION,
        COMPATIBILITY_MAP.clone(),
    ));

    let bls_validator = NativeBlsModule::default();
    let public_keys_validator = Arc::new(PublicKeysValidator::new(bls_validator)?);


    let identity_validator = Arc::new(IdentityValidator::new(
        protocol_version_validator,
        public_keys_validator,
    )?);

    let factory: IdentityFactory<NativeBlsModule> = IdentityFactory::new(LATEST_VERSION, Arc::new(Arc::<IdentityValidator<PublicKeysValidator<NativeBlsModule>>>::into_inner(identity_validator).unwrap()));

    println!("initialized factory");
    let identity = factory.create_from_object(
        raw_identity, false,
    ).unwrap();

    identity.to_cbor_buffer()
}

pub fn get_public_keys_validator_for_transition() -> PublicKeysValidator<NativeBlsModule> {
    PublicKeysValidator::new_with_schema(
        PUBLIC_KEY_SCHEMA_FOR_TRANSITION.clone(),
        NativeBlsModule::default(),
    )
        .unwrap()
}
#[derive(Default)]
pub struct SignaturesValidatorMock {}

impl TPublicKeysSignaturesValidator for SignaturesValidatorMock {
    fn validate_public_key_signatures<'a>(
        &self,
        _raw_state_transition: &Value,
        _raw_public_keys: impl IntoIterator<Item = &'a Value>,
    ) -> Result<SimpleConsensusValidationResult, dpp::NonConsensusError> {
        Ok(SimpleConsensusValidationResult::default())
    }
}


pub fn identity_create_transition_verify_from_raw_object(raw_identity_transition: Value) -> Result<bool, ProtocolError> {
    let result = IdentityCreateTransition::from_raw_object(raw_identity_transition.clone());
    match result {
        Ok(identity_create_transition) => {
            let protocol_version_validator = ProtocolVersionValidator::default();
            let public_keys_validator = Arc::new(get_public_keys_validator_for_transition());
            let public_keys_transition_validator = Arc::new(RequiredPurposeAndSecurityLevelValidator::default());
            let state_repository = Arc::new(MyStateRepository::new());
            let asset_lock_transaction_validator =
                Arc::new(AssetLockTransactionValidator::new(state_repository.clone()));

            let instant_asset_lock_validator = InstantAssetLockProofStructureValidator::new(
                state_repository.clone(),
                asset_lock_transaction_validator.clone(),
            )
                .unwrap();

            let chain_asset_lock_validator = ChainAssetLockProofStructureValidator::new(
                state_repository,
                asset_lock_transaction_validator,
            )
                .unwrap();

            let asset_lock_proof_validator = Arc::new(AssetLockProofValidator::new(
                instant_asset_lock_validator,
                chain_asset_lock_validator,
            ));
            let idt_validator =
                IdentityCreateTransitionBasicValidator::new(
                    protocol_version_validator,
                    public_keys_validator,
                    public_keys_transition_validator,
                    asset_lock_proof_validator,
                    NativeBlsModule::default(),
                    Arc::new(SignaturesValidatorMock::default()),
                ).unwrap();

            let runtime = tokio::runtime::Runtime::new().unwrap();

            let result = runtime.block_on(idt_validator.validate(&raw_identity_transition, &StateTransitionExecutionContext::default()));

            //async {
            //    match result {
            //        ValidationResult { errors: err, data } => data.unwrap(),//.unwrap(),
            //        Err(e) => panic!("{}", e)
            //    }
            //}
            //Result::Ok(true)
            Ok(result.unwrap().is_valid())
        },
        Err(error) => Result::Err(error)
    }
}

pub fn serialize_identity_from_raw_object(raw_identity: Value) -> Result<Vec<u8>, ProtocolError> {
// can we use DashPlatformProtocol?
    let protocol_version_validator = Arc::new(ProtocolVersionValidator::new(
        LATEST_VERSION,
        LATEST_VERSION,
        COMPATIBILITY_MAP.clone(),
    ));

    let bls_validator = NativeBlsModule::default();
    let public_keys_validator = Arc::new(PublicKeysValidator::new(bls_validator)?);


    let identity_validator = Arc::new(IdentityValidator::new(
        protocol_version_validator,
        public_keys_validator,
    )?);

    let factory: IdentityFactory<NativeBlsModule> = IdentityFactory::new(LATEST_VERSION, Arc::new(Arc::<IdentityValidator<PublicKeysValidator<NativeBlsModule>>>::into_inner(identity_validator).unwrap()));

    println!("initialized factory");
    let identity = factory.create_from_object(
        raw_identity, false,
    ).unwrap();

    identity.serialize()
}

pub fn create_identity_from_raw_bytes(bytes: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
    let result: Result<Identity, ProtocolError> = Identity::deserialize(bytes.as_slice());
    match result {
        Ok(identity) => identity.to_cbor_buffer(),
        Err(error) => Result::Err(error)
    }
}

#[test]
fn create_identity_public_keys_test() {
    let raw_ipk = platform_value!({
                "id": 0u32,
                "type": 0u8,
                "purpose": 0u8,
                "securityLevel": 0u8,
                "data": BinaryData::from_string("AuryIuMtRrl/VviQuyLD1l4nmxi9ogPzC9LT7tdpo0di", Encoding::Base64).unwrap(),
                "readOnly": false
            }
    );

    let result = IdentityPublicKey::from_value(raw_ipk.clone());
    let bytes = match result {
        Ok(ipk) => ipk.serialize(),
        Err(error) => panic!("error: {}", error)
    };

    let result2 = IdentityPublicKey::from_value(raw_ipk);
    let bytes2 = match result2 {
        Ok(ipk) => ipk.to_cbor_buffer(),
        Err(error) => panic!("error: {}", error)
    };

    println!("ipk bytes: {}", encode(bytes.unwrap()));
    println!("ipk bytes cbor: {}", encode(bytes2.unwrap()))
}

#[test]
fn decode_identity_create_statetransition_test() {
    let unsigned = hex::decode("0202000000000021036e6f84b4b4525605f819fba38388e7e088606663afa789730413664c71d0ecc200010000020021025cefce88ee5484256814fad4619ddf6b845e6cd2429af0115115f1bb87169a26000000c601011dbbda5861b12d7523f20aa5e0d42f52de3dcd2d5c2fe919ba67b59f050d206e00000000c00964ff90e9c29e60682e0fe18598ddd462f3a8eb92615cf422888c95f1dcad2e02c76c7e57779afd9942f983afbfe2f1e0dd07cab57a75a776b062dfd0c80d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000bc03000000018c6a321936a4da0649aa8eeb2da3280c10088c057a22baf2bf987951924e8cc8000000006a473044022073ed0ee14acdfd31ae0e3627109676b065b97651342187f7501e42525404ad2602205b7f12c535d0a7a5693390d3456c93752b15d39467a63c687b516c5aafb93a7b0121027d916607b92bac0a1527aa126adacbdc24e749989adc0cd2c62dfc0a0d4035e3ffffffff0140420f0000000000166a1425ec403d0cef684745e7741e90183cc0f1282775000000000001009ef3dc44553db4b808be2cab7052f20a96d5d184e77625d9c27257ecd950f9b5").unwrap();
    let signed = hex::decode("0202000000000021036e6f84b4b4525605f819fba38388e7e088606663afa789730413664c71d0ecc24120f9353522ce1f724b70c87f5f9454f7453da703fcc5f4bc4f5c36e3d9de16eb86501d7ea8b42dfbaaa384deabf35e3165668b7a7dde1c214e81b4504b97f3cd9b010000020021025cefce88ee5484256814fad4619ddf6b845e6cd2429af0115115f1bb87169a26411f1b04469f1507cbae51b990b6608f215e44017c43b858b0e21f874c379f9cf3a8153a02408641624dded0568c0854817a368c2ae2cd309df6aefb50812b3fd0510000c601011dbbda5861b12d7523f20aa5e0d42f52de3dcd2d5c2fe919ba67b59f050d206e00000000c00964ff90e9c29e60682e0fe18598ddd462f3a8eb92615cf422888c95f1dcad2e02c76c7e57779afd9942f983afbfe2f1e0dd07cab57a75a776b062dfd0c80d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000bc03000000018c6a321936a4da0649aa8eeb2da3280c10088c057a22baf2bf987951924e8cc8000000006a473044022073ed0ee14acdfd31ae0e3627109676b065b97651342187f7501e42525404ad2602205b7f12c535d0a7a5693390d3456c93752b15d39467a63c687b516c5aafb93a7b0121027d916607b92bac0a1527aa126adacbdc24e749989adc0cd2c62dfc0a0d4035e3ffffffff0140420f0000000000166a1425ec403d0cef684745e7741e90183cc0f1282775000000000001412058cb28b00b749ef0736363e110dd2ac2f26917ddffd086532dd3fa3885a0993f54abeb9e17960a2704feb96632595f67d9ce3692a443b62e8969740c874962859ef3dc44553db4b808be2cab7052f20a96d5d184e77625d9c27257ecd950f9b5").unwrap();

    let result = IdentityCreateTransition::deserialize(unsigned.as_slice());
    match result {
        Ok(ict) => println!("{}", encode(ict.to_cbor_buffer(false).unwrap())),
        Err(error) => println!("{}", error)
    }

    let signed_result = IdentityCreateTransition::deserialize(signed.as_slice());
    match signed_result {
        Ok(ict) => {
            println!("cbor: {}", encode(ict.to_cbor_buffer(false).unwrap()));
            println!("buffer: {}", encode(ict.serialize().unwrap()))
        },
        Err(error) => panic!("{}", error)
    }
}

#[test]
fn create_identity_test() {
    let raw_identity = platform_value!({
        "protocolVersion": 1u32,
        "id": Identifier::from([198, 23, 40, 120, 58, 93, 0, 165, 27, 49, 4, 117, 107, 204,  67, 46, 164, 216, 230, 135, 201, 92, 31, 155, 62, 131, 211, 177, 139, 175, 163, 237]),
        "publicKeys": [
            {
                "id": 0u32,
                "type": 0u8,
                "purpose": 0u8,
                "securityLevel": 0u8,
                "data": BinaryData::from_string("AuryIuMtRrl/VviQuyLD1l4nmxi9ogPzC9LT7tdpo0di", Encoding::Base64).unwrap(),
                "readOnly": false
            },
            {
                "id": 1u32,
                "type": 0u8,
                "purpose": 1u8,
                "securityLevel": 3u8,
                "data": BinaryData::from_string("A8AK95PYMVX5VQKzOhcVQRCUbc9pyg3RiL7jttEMDU+L", Encoding::Base64).unwrap(),
                "readOnly": false
            }
        ],
        "balance": 10u64,
        "revision": 0u64
    });

    let result = create_identity_from_raw_object(raw_identity);

    let hex: String = encode(result.unwrap());
    let expected_hex = "01a46269645820c61728783a5d00a51b3104756bcc432ea4d8e687c95c1f9b3e83d3b18bafa3ed6762616c616e63650a687265766973696f6e006a7075626c69634b65797382a6626964006464617461582102eaf222e32d46b97f56f890bb22c3d65e279b18bda203f30bd2d3eed769a3476264747970650067707572706f73650068726561644f6e6c79f46d73656375726974794c6576656c00a6626964016464617461582103c00af793d83155f95502b33a17154110946dcf69ca0dd188bee3b6d10c0d4f8b64747970650067707572706f73650168726561644f6e6c79f46d73656375726974794c6576656c03";
    assert_eq!(expected_hex, hex);
}
#[test]
fn create_identity_transition_test() {
    let raw_identity_transition = platform_value!({
        "protocolVersion": LATEST_VERSION,
        "assetLockProof": {
            "type": 0u8, 
            "instantLock": BinaryData::new(decode("01011DBBDA5861B12D7523F20AA5E0D42F52DE3DCD2D5C2FE919BA67B59F050D206E00000000C00964FF90E9C29E60682E0FE18598DDD462F3A8EB92615CF422888C95F1DCAD2E02C76C7E57779AFD9942F983AFBFE2F1E0DD07CAB57A75A776B062DFD0C80D000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap()),
            "transaction": BinaryData::new(decode("03000000018C6A321936A4DA0649AA8EEB2DA3280C10088C057A22BAF2BF987951924E8CC8000000006A473044022073ED0EE14ACDFD31AE0E3627109676B065B97651342187F7501E42525404AD2602205B7F12C535D0A7A5693390D3456C93752B15D39467A63C687B516C5AAFB93A7B0121027D916607B92BAC0A1527AA126ADACBDC24E749989ADC0CD2C62DFC0A0D4035E3FFFFFFFF0140420F0000000000166A1425EC403D0CEF684745E7741E90183CC0F128277500000000").unwrap()),
            "outputIndex": 0u32
        }, 
        "publicKeys": [
            {
                "id": 0u32,
                "type": 0u8,
                "purpose": 0, 
                "securityLevel": 0, 
                "readOnly": false, 
                "data": BinaryData::new(decode("036E6F84B4B4525605F819FBA38388E7E088606663AFA789730413664C71D0ECC2").unwrap()),
                "signature": BinaryData::new(decode("20F9353522CE1F724B70C87F5F9454F7453DA703FCC5F4BC4F5C36E3D9DE16EB86501D7EA8B42DFBAAA384DEABF35E3165668B7A7DDE1C214E81B4504B97F3CD9B").unwrap())
            },
            {
                "id": 1,
                "type": 0,
                "purpose": 0,
                "securityLevel": 2,
                "readOnly": false,
                "data": BinaryData::new(decode("025CEFCE88EE5484256814FAD4619DDF6B845E6CD2429AF0115115F1BB87169A26").unwrap()),
                "signature": BinaryData::new(decode("1F1B04469F1507CBAE51B990B6608F215E44017C43B858B0E21F874C379F9CF3A8153A02408641624DDED0568C0854817A368C2AE2CD309DF6AEFB50812B3FD051").unwrap())
            }
        ],
        "signature": BinaryData::new(decode("2058CB28B00B749EF0736363E110DD2AC2F26917DDFFD086532DD3FA3885A0993F54ABEB9E17960A2704FEB96632595F67D9CE3692A443B62E8969740C87496285").unwrap()),
        "type": 2
    });

    let result = identity_create_transition_cbor_from_raw_object(raw_identity_transition.clone());

    match result {
        Ok(bytes) => {
            let hex: String = encode(bytes);
            let expected_hex = "01a46e61737365744c6f636b50726f6f66a46474797065006b696e7374616e744c6f636b58c601011dbbda5861b12d7523f20aa5e0d42f52de3dcd2d5c2fe919ba67b59f050d206e00000000c00964ff90e9c29e60682e0fe18598ddd462f3a8eb92615cf422888c95f1dcad2e02c76c7e57779afd9942f983afbfe2f1e0dd07cab57a75a776b062dfd0c80d0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006b7472616e73616374696f6e58bc03000000018c6a321936a4da0649aa8eeb2da3280c10088c057a22baf2bf987951924e8cc8000000006a473044022073ed0ee14acdfd31ae0e3627109676b065b97651342187f7501e42525404ad2602205b7f12c535d0a7a5693390d3456c93752b15d39467a63c687b516c5aafb93a7b0121027d916607b92bac0a1527aa126adacbdc24e749989adc0cd2c62dfc0a0d4035e3ffffffff0140420f0000000000166a1425ec403d0cef684745e7741e90183cc0f1282775000000006b6f7574707574496e646578006a7075626c69634b65797382a76269640064747970650067707572706f7365006d73656375726974794c6576656c0068726561644f6e6c79f464646174615821036e6f84b4b4525605f819fba38388e7e088606663afa789730413664c71d0ecc2697369676e6174757265584120f9353522ce1f724b70c87f5f9454f7453da703fcc5f4bc4f5c36e3d9de16eb86501d7ea8b42dfbaaa384deabf35e3165668b7a7dde1c214e81b4504b97f3cd9ba76269640164747970650067707572706f7365006d73656375726974794c6576656c0268726561644f6e6c79f464646174615821025cefce88ee5484256814fad4619ddf6b845e6cd2429af0115115f1bb87169a26697369676e617475726558411f1b04469f1507cbae51b990b6608f215e44017c43b858b0e21f874c379f9cf3a8153a02408641624dded0568c0854817a368c2ae2cd309df6aefb50812b3fd051697369676e617475726558412058cb28b00b749ef0736363e110dd2ac2f26917ddffd086532dd3fa3885a0993f54abeb9e17960a2704feb96632595f67d9ce3692a443b62e8969740c87496285647479706502";
            assert_eq!(expected_hex, hex);
        }
        Err(error) => panic!("failure: {}", error)
    }

    let transition = identity_create_transition_verify_from_raw_object(raw_identity_transition);
    match transition {
        Ok(validated) => {
            println!("validation {}", validated);
            assert!(validated)
        },
        Err(error) => panic!("validation failed {}", error)
    }

    let signable = decode("0202000000000021038035e6856dd646654eb1a76dd9bd93af0e21889feb68a652fb8360974be3b6a90100000200210396605ff4ca17f88a6294d8ce5b65d7ae797ab7ef61f7ff38acdf036cf9c61c0d0101fc0001edce9683e679611f3c2f0bfaf7f8ee55f9312fbf059a421e36ab5d3c4a854946ca220000000001").unwrap();
    let serialized = decode("0202000000000021038035e6856dd646654eb1a76dd9bd93af0e21889feb68a652fb8360974be3b6a9412085d4139c1f81b223beee448f92b117fd489a995424102c40b2ca4ec6cd31ce7e7b9410e2aa8a88f47f6fcf757bdd801b7f8f091c3bf803f850c64ee24e7c62e90100000200210396605ff4ca17f88a6294d8ce5b65d7ae797ab7ef61f7ff38acdf036cf9c61c0d4120ca609cbb2f4a2fe563ffb4d093e59e3f2bb3f3bdb56cf7fe2ddf7b4e16f43d685d1a56caac7e8476adb7e2cc131db1b9a3a3fa78fa10f36724d20fb69de69d5a0101fc0001edce9683e679611f3c2f0bfaf7f8ee55f9312fbf059a421e36ab5d3c4a854946ca220000000001411ff1764ead95ff03f90b177fa138bd0b510309da5ac75b6b0498d18f4a1f55036c77d6626b9dc660b54ca9f7085fef1a9070d2694e064175c6b68cb9d947b2017ee4adf70cd99961ea20fac1b639b9c72e8511914f547877857bd560a1dca71d76").unwrap();

    let real_transition = IdentityCreateTransition::deserialize(serialized.as_slice()).unwrap();

    let transition = identity_create_transition_verify_from_raw_object(real_transition.to_object(false).unwrap());
    match transition {
        Ok(validated) => {
            println!("validation {}", validated);
            assert!(validated)
        },
        Err(error) => panic!("validation failed {}", error)
    }
}

#[test]
fn decode_identity() {
    let hex = "0120413f39f3cc096a47bb025eddbe7f8d7289d1f3323cd75a65e50f73052c75b6d9020000000000002103292da1684067c3b78ecef3d3e2c132434e7aa9930c0a520bfc4e2ad73a3edd6b0001010002000021020a066d33abcd94c00f01fc84ae3376f7103c79d8dbacba2b4b20e0eb733ec0150000000000";
    match create_identity_from_raw_bytes(decode(hex).unwrap()) {
        Ok(cbor) => println!("{}", encode(cbor)),
        Err(error) => panic!("{}", error)
    }
}

