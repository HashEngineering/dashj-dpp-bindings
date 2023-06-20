#[allow(non_snake_case)]

pub mod dpp_jni {
    use dpp::version::LATEST_VERSION;
    use jni::JNIEnv;
    use jni::objects::{JClass};
    use jni::sys::{jint, jbyteArray};

    use mylibrary::simple_dpp::create_identity;


    #[no_mangle]
    pub unsafe extern fn Java_org_dashj_dpp_ProtocolVersion_getLatestProtocolVersionBinding(_env: JNIEnv, _: JClass) -> jint {
        LATEST_VERSION as jint
    }

    #[no_mangle]
    pub unsafe extern fn Java_org_dashj_dpp_DPP_createIdentity(env: JNIEnv, _: JClass) -> jbyteArray {
        let result = create_identity();

        match result {
            Err(_) => {
                let error = result.err();
                println!("Error decoding json: {:?}", error);
                let empty_vec: Vec<u8> = vec![];
                let byte_array = env.byte_array_from_slice(&empty_vec).unwrap();
                byte_array
            }
            Ok(bytes) => {
                let byte_array = env.byte_array_from_slice(&*bytes).unwrap();
                byte_array
            }
        }
    }
}