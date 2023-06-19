#[allow(non_snake_case)]
pub mod android {
    use dpp::version::{LATEST_VERSION};
    use jni::JNIEnv;
    use jni::objects::JClass;
    use jni::sys::jint;


    #[no_mangle]
    pub unsafe extern fn Java_org_dashj_dpp_ProtocolVersion_getLatestProtocolVersionBinding(_env: JNIEnv, _: JClass) -> jint {
        LATEST_VERSION as jint
    }
}