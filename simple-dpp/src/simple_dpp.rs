use hex::{encode, decode};
use std::sync::Arc;
use dpp::identity::{/*Identity*/ validation::PublicKeysValidator};
use dpp::identity::factory::IdentityFactory;
use dpp::{Convertible, ProtocolError};
use dpp::identity::state_transition::identity_create_transition::IdentityCreateTransition;
use dpp::identity::validation::IdentityValidator;
use platform_value::{BinaryData, Identifier, Value};
use platform_value::string_encoding::Encoding;
use platform_value::platform_value;
use dpp::version::{ProtocolVersionValidator, LATEST_VERSION, COMPATIBILITY_MAP};
use dpp::NativeBlsModule;
use dpp::state_transition::StateTransitionConvert;

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
//let factory: IdentityFacade<NativeBlsModule> = IdentityFacade::new(LATEST_VERSION, protocol_version_validator, public_keys_validator);


    let identity = factory.create_from_object(
        raw_identity, false,
    ).unwrap();

    identity.to_cbor_buffer()
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


