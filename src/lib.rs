mod jvmti;

extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject, GlobalRef};
use jni::sys::{JavaVM, JNI_VERSION_1_8, JNI_OK, jint, jobject, jclass};
use std::ffi::CStr;
use crate::jvmti::{jvmtiEnv, JVMTI_VERSION_1_2, jvmtiCapabilities, jvmtiError_JVMTI_ERROR_NONE, jvmtiEventCallbacks, jvmtiEventMode_JVMTI_ENABLE, jvmtiEvent_JVMTI_EVENT_CLASS_FILE_LOAD_HOOK};
use std::ptr::null_mut;
use std::os::raw::{c_void, c_int, c_uchar, c_char};
use std::mem::{zeroed, size_of};
use obfstr::{obfstr};

static mut G_BYTEMAP: Option<GlobalRef> = None;


#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(
	mut _env: JavaVM, _reserved: &mut c_void) -> c_int {
	JNI_VERSION_1_8
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_cookiedragon234_falcon_NativeAccessor_b(
	env: JNIEnv, _class: JClass,
	str: JString, byte_map: JObject, _class_loader: JObject, _redefinitions: JObject) {
	let chars = env.get_string_utf_chars(str).unwrap();
	println!("Hello '{}'", CStr::from_ptr(chars).to_str().unwrap());
	env.release_string_utf_chars(str, chars).unwrap();
	
	G_BYTEMAP = Some(env.new_global_ref(byte_map).unwrap());
	
	let mut void_ptr: *mut c_void = null_mut() as *mut c_void;
	let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
	let vm = env.get_java_vm().unwrap().get_java_vm_pointer();
	
	{
		let result = (**vm).GetEnv.unwrap()(vm, penv_ptr, JVMTI_VERSION_1_2 as i32);
		if result != JNI_OK {
			panic!("{} ({})", obfstr!("Couldn't fetch jvmti instance"), result);
		}
	}
	
	let jvmti: *mut jvmtiEnv = *penv_ptr as *mut jvmtiEnv;
	
	{
		let mut capabilities: jvmtiCapabilities = zeroed();
		capabilities.set_can_retransform_classes(1);
		capabilities.set_can_retransform_classes(1);
		capabilities.set_can_generate_native_method_bind_events(1);
		
		let result = (**jvmti).AddCapabilities.unwrap()(jvmti, &capabilities);
		if result != jvmtiError_JVMTI_ERROR_NONE {
			panic!("{} ({})", obfstr!("Couldn't add jvmti capabilities"), result);
		}
	}
	
	{
		let mut callbacks: jvmtiEventCallbacks = zeroed();
		callbacks.ClassFileLoadHook = Some(loadHookHandler);
		
		let result = (**jvmti).SetEventCallbacks.unwrap()(jvmti, &callbacks, size_of::<jvmtiEventCallbacks>() as i32);
		if result != jvmtiError_JVMTI_ERROR_NONE {
			panic!("{} ({})", obfstr!("Couldn't add jvmti callbacks"), result);
		}
	}
	
	{
		let result = (**jvmti).SetEventNotificationMode.unwrap()(jvmti, jvmtiEventMode_JVMTI_ENABLE, jvmtiEvent_JVMTI_EVENT_CLASS_FILE_LOAD_HOOK, null_mut());
		if result != jvmtiError_JVMTI_ERROR_NONE {
			panic!("{} ({})", obfstr!("Couldn't enable jvmti callbacks"), result);
		}
	}
	
	println!("{}", obfstr!("Raion native libary initialisation finished (4)"));
}

pub unsafe extern "C" fn loadHookHandler(
	jvmti_env: *mut jvmtiEnv,
	jni_env: *mut jni::sys::JNIEnv,
	class_being_redefined: jclass,
	loader: jobject,
	name: *const c_char,
	protection_domain: jobject,
	class_data_len: jint,
	class_data: *const c_uchar,
	new_class_data_len: *mut jint,
	new_class_data: *mut *mut c_uchar) {
	if name.is_null() {
		return;
	}
	println!("Class loaded {}", CStr::from_ptr(name).to_str().unwrap());
}
