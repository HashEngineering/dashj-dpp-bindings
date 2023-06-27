#[allow(non_snake_case)]

pub mod dpp_jni {
    use dpp::version::LATEST_VERSION;
    use jni::JNIEnv;
    use jni::objects::{JClass, JList, JMap, JObject, JValue, JString, JPrimitiveArray, ReleaseMode};
    use jni::strings::JavaStr;
    use jni::sys::{JNI_TRUE, jbyteArray, jbyte, jint, jboolean};
    use platform_value::Value;
    //use std::any::Any;

    use mylibrary::simple_dpp::create_identity_cbor;
    use mylibrary::simple_dpp::create_identity_from_raw_object;
    use std::collections::BTreeMap;


    #[no_mangle]
    pub unsafe extern fn Java_org_dashj_dpp_ProtocolVersion_getLatestProtocolVersionBinding(_env: JNIEnv, _: JClass) -> jint {
        LATEST_VERSION as jint
    }

    #[no_mangle]
    pub unsafe extern fn Java_org_dashj_dpp_DPP_createIdentityCbor(env: JNIEnv, _: JClass) -> jbyteArray {
        let result = create_identity_cbor();

        match result {
            Err(_) => {
                let error = result.err();
                println!("Error decoding json: {:?}", error);
                let empty_vec: Vec<u8> = vec![];
                let byte_array = env.byte_array_from_slice(&empty_vec).unwrap();
                byte_array.as_raw()
            }
            Ok(bytes) => {
                let byte_array = env.byte_array_from_slice(&*bytes).unwrap();
                byte_array.as_raw()
            }
        }
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_dashj_dpp_DPP_convertMap(
        mut env: JNIEnv,
        _: JClass,
        jmap: JObject
    ) -> jboolean {

        //let jmap = JMap::from_env(&mut env, &java_map).unwrap();
        let btree_map = convert_jmap_to_btreemap(&mut env, &jmap);
        //println!("{}", btree_map);
        let _platform_value: Value = btree_map.into();
        JNI_TRUE
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_dashj_dpp_DPP_createIdentityFromRawObject(
        mut env: JNIEnv,
        _: JClass,
        jmap: JObject
    ) -> jbyteArray {

        let btree_map = convert_jmap_to_btreemap(&mut env, &jmap);
        let _platform_value: Value = btree_map.into();

        let result = create_identity_from_raw_object(_platform_value);

        match result {
            Err(_) => {
                let error = result.err();
                println!("Error decoding json: {:?}", error);
                let empty_vec: Vec<u8> = vec![];
                let byte_array = env.byte_array_from_slice(&empty_vec).unwrap();
                byte_array.as_raw()
            }
            Ok(bytes) => {
                let byte_array = env.byte_array_from_slice(&*bytes).unwrap();
                byte_array.as_raw()
            }
        }
    }

    fn convert_jmap_to_btreemap(env: &mut JNIEnv, jmap: &JObject) -> Value {
        let btreemap = convert_jobject_to_rust(env, &jmap);
        btreemap
    }

    fn get_byte_array_from_method(env: &mut JNIEnv, obj: &JObject, method: &str) -> Vec<u8> {
        let result = env
            .call_method(obj, method, "()[B", &[])
            .expect("Failed to call method");

        let byte_array_obj = result.l().expect("Expected JObject");

        get_byte_array_from_java_object(env, byte_array_obj)
    }

    fn get_byte_array_from_java_object(env: &mut JNIEnv, byte_array_obj: JObject) -> Vec<u8> {
        let byte_array: JPrimitiveArray<jbyte> = byte_array_obj.into();

        let length = env.get_array_length(&byte_array)
            .expect("Couldn't fetch array length") as usize;

        let mut output: Vec<i8> = vec![0; length];
        env.get_byte_array_region(byte_array, 0, &mut output)
            .expect("Couldn't get byte array region");

        output.into_iter().map(|b| b as u8).collect()
    }

    fn get_byte_array_from_java_primitive_array(env: &mut JNIEnv, byte_array: JPrimitiveArray<jbyte>) -> Vec<u8> {
        let length = env.get_array_length(&byte_array)
            .expect("Couldn't fetch array length") as usize;

        let mut output: Vec<i8> = vec![0; length];
        env.get_byte_array_region(byte_array, 0, &mut output)
            .expect("Couldn't get byte array region");

        output.into_iter().map(|b| b as u8).collect()
    }

    fn convert_jobject_to_rust(env: &mut JNIEnv, obj: &JObject) -> Value {
        if env.is_instance_of(obj, "java/lang/Integer").unwrap() {
            let val: i32 = env.call_method(obj, "intValue", "()I", &[]).unwrap().i().unwrap();
            Value::I32(val)
        } else if env.is_instance_of(obj, "java/lang/Long").unwrap() {
            let val: i64 = env.call_method(obj, "longValue", "()J", &[]).unwrap().j().unwrap();
            Value::I64(val)
        } else if env.is_instance_of(obj, "java/lang/Float").unwrap() {
            let val: f32 = env.call_method(obj, "floatValue", "()F", &[]).unwrap().f().unwrap();
            Value::Float(val.into())
        } else if env.is_instance_of(obj, "java/lang/Double").unwrap() {
            let val: f64 = env.call_method(obj, "doubleValue", "()D", &[]).unwrap().d().unwrap();
            Value::Float(val)
        } else if env.is_instance_of(obj, "java/lang/Boolean").unwrap() {
            let val: bool = env.call_method(obj, "booleanValue", "()Z", &[]).unwrap().z().unwrap();
            Value::Bool(val)
        } else if env.is_instance_of(obj, "org/dashj/platform/dpp/identifier/Identifier").unwrap() {
            let bytes = get_byte_array_from_method(env, obj,"toBuffer");
            Value::Bytes(bytes)
        } else if env.is_instance_of(obj, "java/util/List").unwrap() {
            let list: JList = JList::from_env(env, &obj).unwrap();
            let mut iterator = list.iter(env).unwrap();
            let mut vec: Vec<Value> = Vec::new();

            while let Ok(Some(item)) = iterator.next(env) {
                let rust_item = convert_jobject_to_rust(env, &item);
                vec.push(rust_item);
            }

            Value::Array(vec)
        } else if env.is_instance_of(obj, "java/util/Map").unwrap() {
            let map: JMap = JMap::from_env(env, &obj).unwrap();
            let mut iterator = map.iter(env).unwrap();
            let mut btree_map: BTreeMap<String, Value> = BTreeMap::new();

            while let Ok(Some((key, value))) = iterator.next(env) {
                let key_str: String = <JavaStr<'_, '_, '_> as Into<String>>::into(
                    env.get_string(&JString::from(key)).unwrap());
                println!("processing key {}", key_str);

                let rust_value = convert_jobject_to_rust(env, &value);

                btree_map.insert(key_str, rust_value);
            }
            btree_map.into()
        } else if env.is_instance_of(obj, "[B").unwrap() {
            // change from Java Byte Array to Vec<u8>
            // this does not match get_byte_array_from_java_object
            let byte_array = unsafe { env.get_array_elements(obj.into(), ReleaseMode::NoCopyBack).unwrap() };
            let byte_vec = byte_array.to_vec();
            Value::Bytes(byte_vec)

            // Value::Bytes(get_byte_array_from_java_primitive_array(env, (*obj).into()))
            //     |                                                      ^^^^^^ ------ `*obj` moved due to this method call
            //     move occurs because `*obj` has type `JObject<'_>`, which does not implement the `Copy` trait

            // Value::Bytes(get_byte_array_from_java_primitive_array(env, (*obj).into()))
        } else {
            let class_obj = env.get_object_class(obj).unwrap();
            let class_name_jstring: jni::objects::JObject = env.call_method(class_obj, "getName", "()Ljava/lang/String;", &[]).unwrap().l().unwrap().into();
            let class_name_rust_string: String = env.get_string(&jni::objects::JString::from(class_name_jstring)).unwrap().into();
            panic!("Unsupported Java type: {}", class_name_rust_string);
        }
    }
}