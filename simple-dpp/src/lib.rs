pub mod simple_dpp;
mod state_repository;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub use async_trait;
// #[allow(non_snake_case)]
// pub mod simple-dpp {
//     //extern crate jni;
//     //extern crate hex;
//     //use super::*;
//
//     //use std::convert::TryInto;
//     use hex::{encode, decode};
//     //use std::ops::Deref;
//     //use jni::JNIEnv;
//     //use jni::objects::{JClass, JObject};
//     //use jni::sys::{jstring, jbyteArray, jobject, jboolean, JNI_FALSE, JNI_TRUE};
//     //use std::convert::TryInto;
//     use std::sync::Arc;
//     use dpp::identity::{/*Identity*/ validation::PublicKeysValidator};
//     use dpp::identity::factory::IdentityFactory;
//     //use dpp::identity::state_transition::asset_lock_proof::InstantAssetLockProof;
//     //use dpp::identity::identity_facade::IdentityFactory;
//     use dpp::{Convertible, ProtocolError};
//     use dpp::identity::state_transition::identity_create_transition::IdentityCreateTransition;
//     use dpp::identity::validation::IdentityValidator;
//     use platform_value::{BinaryData, Identifier};
//     use platform_value::string_encoding::Encoding;
//     use platform_value::platform_value;
//     use dpp::version::{ProtocolVersionValidator, LATEST_VERSION, COMPATIBILITY_MAP};
//     use dpp::NativeBlsModule;
//     use dpp::state_transition::StateTransitionConvert;
//     //use dpp::prelude::AssetLockProof;
//     //use dpp::state_transition::StateTransitionConvert;
//     // use dash_platform_protocol::DPPOptions;
//
//     // this is private in dpp, wasm-dpp defines it also
//     // #[derive(Serialize, Deserialize)]
//     // pub struct DPPOptions {
//     //     #[serde(rename = "protocolVersion")]
//     //     pub current_protocol_version: Option<u32>,
//     // }
//     //
//     // use dpp::{
//     //      state_repository::MockStateRepositoryLike,
//     //      DashPlatformProtocol, NativeBlsModule,
//     //  };
//     //
//     // #[derive(Debug, Clone)]
//     // pub struct MyDashPlatformProtocol<SR> {
//     //     pub state_repository: SR,
//     // }
//     // impl<SR> MyDashPlatformProtocol<SR> {
//     //     pub fn new(state_repository: SR) -> Self {
//     //         MyDashPlatformProtocol { state_repository }
//     //     }
//     //
//     //     pub fn get_protocol_version(&self) -> u32 {
//     //         1
//     //     }
//     //
//     //     pub fn get_state_repository(&self) -> &SR {
//     //         &self.state_repository
//     //     }
//     // }
//
//     //
//     // // TODO creation of DPP object for testing needs to be improved
//     // pub fn get_dpp() -> MyDashPlatformProtocol<MockStateRepositoryLike, NativeBlsModule> {
//     //     MyDashPlatformProtocol::new().unwrap()
//     // }
//
//     pub fn create_identity_create_transition(_key1: Vec<u8>, _key2: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
//         // // can we use DashPlatformProtocol?
//         // let protocol_version_validator = Arc::new(ProtocolVersionValidator::new(
//         //     LATEST_VERSION,
//         //     LATEST_VERSION,
//         //     COMPATIBILITY_MAP.clone(),
//         // ));
//         //
//         // let bls_validator = NativeBlsModule::default();
//         // let public_keys_validator = Arc::new(PublicKeysValidator::new(bls_validator)?);
//         //
//         //
//         // let identity_validator = Arc::new(IdentityValidator::new(
//         //     protocol_version_validator,
//         //     public_keys_validator,
//         // )?);
//         //
//         // let factory: IdentityFactory<NativeBlsModule> = IdentityFactory::new(LATEST_VERSION, Arc::new(Arc::<IdentityValidator<PublicKeysValidator<NativeBlsModule>>>::into_inner(identity_validator).unwrap()));
//         // //let factory: IdentityFacade<NativeBlsModule> = IdentityFacade::new(LATEST_VERSION, protocol_version_validator, public_keys_validator);
//         //
//
//         let raw_identity_transition = platform_value!({
//             "assetLockProof": {
//                 "type": 0u64,
//                 "instantLock":
//                 decode("01011DBBDA5861B12D7523F20AA5E0D42F52DE3DCD2D5C2FE919BA67B59F050D206E00000000EB1F668BC2821C0D001AE3B5740E8485ACB028C591A6C3C2A137774BE9553B782E02C76C7E57779AFD9942F983AFBFE2F1E0DD07CAB57A75A776B062DFD0C80D000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
//                 "transaction": decode("03000000018C6A321936A4DA0649AA8EEB2DA3280C10088C057A22BAF2BF987951924E8CC8000000006B483045022100913A974DFAE27128F7311ABE69969238B1ECF351F4873BE8C82B97AE438B1E7202203ACEFDEDB7E588A338E12141289760D41C355633C9645459AE5AC9FB5EF9F1F60121027D916607B92BAC0A1527AA126ADACBDC24E749989ADC0CD2C62DFC0A0D4035E3FFFFFFFF0140420F0000000000166A1494DEAED081B3C2FE239F4E2880F94D7B9B3E456E00000000").unwrap(),
//                 "outputIndex": 0
//             },
//             "publicKeys": [
//                 {
//                     "id": 0,
//                     "purpose": 0,
//                     "securityLevel": 0,
//                     "type": 0,
//                     "data": decode("036E6F84B4B4525605F819FBA38388E7E088606663AFA789730413664C71D0ECC2").unwrap(),
//                     "readOnly": false,
//                     "signature":
//                     decode("1F0AEA53BDCB687A0AE763E3D638529F9AC751AD6D038D71691EA3BE650B9120B2730DBC21EB5D508FC9323C0823850701E65DFD7389147FAD1382EA3D6F6F1A85").unwrap()
//                 },
//                 {
//                     "id": 1,
//                     "purpose": 0,
//                     "securityLevel": 2,
//                     "type": 0,
//                     "data": decode("025CEFCE88EE5484256814FAD4619DDF6B845E6CD2429AF0115115F1BB87169A26").unwrap(),
//                     "readOnly": false,
//                     "signature": decode("1FD0A4618BC9D054A7D0089E5723E8DC392E269837D72893AA66726E63BA8E7FDD43EFA5E87A8889EC4786C2A77F8C7BD3FF6425B788BEBA616846FFC4D6CBED21").unwrap()
//                 }
//             ],
//             "signature": decode("208231FAAFD1394B2B1B4DB6C25B5613203485594C36E424C547A47B5F3DBE07C978B44B82CC03E71928243A6448BADC201D0115F96B950928576AF22DCFDC97F9").unwrap(),
//             "type": 2
//         });
//
//         let identity_create_transition = IdentityCreateTransition::from_raw_object(raw_identity_transition);
//
//
//         identity_create_transition.unwrap().to_cbor_buffer(false)
//     }
//
//     pub fn create_identity() -> Result<Vec<u8>, ProtocolError> {
//         // can we use DashPlatformProtocol?
//         let protocol_version_validator = Arc::new(ProtocolVersionValidator::new(
//             LATEST_VERSION,
//             LATEST_VERSION,
//             COMPATIBILITY_MAP.clone(),
//         ));
//
//         let bls_validator = NativeBlsModule::default();
//         let public_keys_validator = Arc::new(PublicKeysValidator::new(bls_validator)?);
//
//
//         let identity_validator = Arc::new(IdentityValidator::new(
//             protocol_version_validator,
//             public_keys_validator,
//         )?);
//
//         let factory: IdentityFactory<NativeBlsModule> = IdentityFactory::new(LATEST_VERSION, Arc::new(Arc::<IdentityValidator<PublicKeysValidator<NativeBlsModule>>>::into_inner(identity_validator).unwrap()));
//         //let factory: IdentityFacade<NativeBlsModule> = IdentityFacade::new(LATEST_VERSION, protocol_version_validator, public_keys_validator);
//
//
//         let raw_identity = platform_value!({
//             "protocolVersion": 1u32,
//             "id": Identifier::from([198, 23, 40, 120, 58, 93, 0, 165, 27, 49, 4, 117, 107, 204,  67, 46, 164, 216, 230, 135, 201, 92, 31, 155, 62, 131, 211, 177, 139, 175, 163, 237]),
//             "publicKeys": [
//                 {
//                     "id": 0u32,
//                     "type": 0u8,
//                     "purpose": 0u8,
//                     "securityLevel": 0u8,
//                     "data": BinaryData::from_string("AuryIuMtRrl/VviQuyLD1l4nmxi9ogPzC9LT7tdpo0di", Encoding::Base64).unwrap(),
//                     "readOnly": false
//                 },
//                 {
//                     "id": 1u32,
//                     "type": 0u8,
//                     "purpose": 1u8,
//                     "securityLevel": 3u8,
//                     "data": BinaryData::from_string("A8AK95PYMVX5VQKzOhcVQRCUbc9pyg3RiL7jttEMDU+L", Encoding::Base64).unwrap(),
//                     "readOnly": false
//                 }
//             ],
//             "balance": 10u64,
//             "revision": 0u64
//         });
//
//         let identity = factory.create_from_object(
//             raw_identity, false,
//         ).unwrap();
//
//         identity.to_cbor_buffer()
//     }
//
//     #[test]
//     fn test1() {
//         // let key1 = Vec::new();
//         // let key2 = Vec::new();
//         // let _result2 = create_identity_create_transition(key1, key2);
//         let result = create_identity();
//
//         let hex: String = encode(result.unwrap());
//         println!("identity {}", hex);
//         //assert!(result.unwrap(), Vec::new());
//     }
//
//
//     // #[no_mangle]
//     // pub unsafe extern fn Java_org_dashj_merk_MerkVerifyProof_getVersion(env: JNIEnv, _: JClass) -> jstring {
//     //     let world_ptr = CString::new("0.22-SNAPSHOT");
//     //     let output = env.new_string(world_ptr.unwrap().to_str().unwrap()).expect("Couldn't create java string!");
//     //     let key1 = Vec::new();
//     //     let key2 = Vec::new();
//     //     let _answer: Vec<u8> = Vec::new();
//     //     let _result = create_identity_create_transition(key1, key2);
//     //     //assert_eq!(result.unwrap(), answer);
//     //     output.into_inner()
//     // }
//     /*
//             fn pop(barry: &[u8]) -> &[u8; 32] {
//                 barry.try_into().expect("slice with incorrect length")
//             }
//
//             #[no_mangle]
//             pub unsafe extern fn Java_org_dashj_merk_MerkVerifyProof_verify(env: JNIEnv, _: JClass, java_bytes: jbyteArray, java_expected_hash: jbyteArray, java_map: jobject) -> jboolean {
//
//                 let bytes = env.convert_byte_array(java_bytes).expect("invalid bytes");
//                 let expected_hash = env.convert_byte_array(java_expected_hash).expect("invalid expected hash");
//                 println!("Parameters passed from Kotlin that were converted to Rust:");
//                 println!("hash:  {}", hex::encode(expected_hash.clone()));
//
//                 let output = verify(bytes.as_slice(), *pop(expected_hash.as_slice()));
//
//                 match output  {
//                     Err(_) => {
//                         let error = output.err();
//                         println!("Error decoding json: {:?}", error);
//                         JNI_FALSE
//                     }
//                     Ok(map) => {
//                         let jmap = env.get_map(java_map.into()).unwrap();
//
//                         for key_value_pair in map.all() {
//                             let key = key_value_pair.0;
//                             let (_exists, value) = key_value_pair.1;
//
//                             let javaKey = env.byte_array_from_slice(&*key).unwrap();
//                             let javaValue = env.byte_array_from_slice(value).unwrap();
//
//                             jmap.put(JObject::from(javaKey), JObject::from(javaValue)).expect("Error adding elements to map");
//                         }
//                         JNI_TRUE
//                     }
//                 }
//             }
//
//             #[no_mangle]
//             pub unsafe extern fn Java_org_dashj_merk_MerkVerifyProof_extractProofNative(env: JNIEnv, _: JClass, java_bytes: jbyteArray, java_map: jobject) -> jbyteArray{
//
//                 let bytes = env.convert_byte_array(java_bytes).expect("invalid bytes");
//                 println!("Parameters passed from Kotlin that were converted to Rust:");
//                 println!("bytes: {}", hex::encode(bytes.clone()));
//
//                 let output = execute_proof(bytes.as_slice());
//
//                 match output  {
//                     Err(_) => {
//                         let error = output.err();
//                         println!("Error decoding json: {:?}", error);
//                         return env.new_byte_array(0).unwrap()
//                     }
//                     Ok((hash, map)) => {
//                         let jmap = env.get_map(java_map.into()).unwrap();
//
//                         for key_value_pair in map.all() {
//                             let key = key_value_pair.0;
//                             let (_exists, value) = key_value_pair.1;
//
//                             let javaKey = env.byte_array_from_slice(&*key).unwrap();
//                             let javaValue = env.byte_array_from_slice(value).unwrap();
//
//                             jmap.put(JObject::from(javaKey), JObject::from(javaValue)).expect("Error adding elements to map");
//                         }
//                         env.byte_array_from_slice(hash.as_ref()).unwrap()
//                     }
//                 }
//             }
//             */
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
