mod debugger_detector;
mod anti_launch_args;
mod jvm;
mod bitfield;
mod jmm;
mod anti_thread;
mod hwid;
mod utils;
mod jvmti;

extern crate jni;

use hwid::get_id;
use jni::{JNIEnv};
use jni::objects::{JClass, JString, JObject, GlobalRef, ReleaseMode, JValue};
use jni::sys::{JavaVM, JNI_VERSION_1_8, JNI_OK, jint, jobject, jclass, JNINativeInterface_, jlong};
use std::ffi::CStr;
use crate::jvmti::{jvmtiEnv, JVMTI_VERSION_1_2, jvmtiCapabilities, jvmtiError_JVMTI_ERROR_NONE, jvmtiEventCallbacks, jvmtiEventMode_JVMTI_ENABLE, jvmtiEvent_JVMTI_EVENT_CLASS_FILE_LOAD_HOOK, jvmtiEvent_JVMTI_EVENT_THREAD_START, jthread};
use std::ptr::{null_mut};
use std::os::raw::{c_void, c_int, c_uchar, c_char};
use std::mem::{zeroed, size_of};
use obfstr::{obfstr};
use jni::strings::{JNIString, JNIStr};
use jni::signature::JavaType;
use crate::anti_thread::{on_thread_start, shutdown_attach_listener};
use anti_launch_args::anti_launch_args;
use crate::debugger_detector::check_debugger;
use crate::jvm::JVM_ClassDepth;

static mut G_BYTEMAP_REF: Option<GlobalRef> = None;


#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(_vm: *mut JavaVM, _reserved: &mut c_void) -> c_int {
	
	JNI_VERSION_1_8
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_cookiedragon234_falcon_NativeAccessor_d<'a>(
	env: JNIEnv<'a>, _class: JClass,
	_1: JObject, _2: JObject, _3: JObject, _4: JObject) -> JString<'a> {
	let hwid = get_id();
	let str = env.new_string(JNIString::from(hwid));
	return str.unwrap();
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_cookiedragon234_falcon_NativeAccessor_c(
	env: JNIEnv, _class: JClass,
	str: JObject) {
	let jstring = JString::from(str);
	let chars = env.get_string_utf_chars(jstring).unwrap();
	println!("{} {}", obfstr!("[RAION]"), CStr::from_ptr(chars).to_str().unwrap());
	env.release_string_utf_chars(jstring, chars).unwrap();
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_cookiedragon234_falcon_NativeAccessor_o(
	_env: JNIEnv, _class: JClass,
	addr: jlong, value: jlong, _unused: JObject) {
	*(addr as *mut jlong) = value;
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_cookiedragon234_falcon_NativeAccessor_b(
	env: JNIEnv, _class: JClass,
	str: JString, byte_map: JObject, _class_loader: JObject, attach_thread: jthread) {
	
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
	
	if str.is_null() {
		println!("{}", obfstr!("Raion is quitting..."));
		std::process::exit(1);
	}
	
	if !_class_loader.is_null() {
		anti_launch_args(env.get_native_interface());
	}
	if check_debugger() {
		println!("{}", obfstr!("[RAION] Please do not attach a debugger - this incident has been reported"));
		return
	}
	
	let chars = env.get_string_utf_chars(str).unwrap();
	println!("{} '{}'", obfstr!("Hello"), CStr::from_ptr(chars).to_str().unwrap());
	env.release_string_utf_chars(str, chars).unwrap();
	
	let global_ref = env.new_global_ref(byte_map).unwrap();
	G_BYTEMAP_REF = Some(global_ref);
	
	{
		let mut capabilities: jvmtiCapabilities = zeroed();
		capabilities.set_can_retransform_classes(1);
		capabilities.set_can_signal_thread(1);
		
		let result = (**jvmti).AddCapabilities.unwrap()(jvmti, &capabilities);
		if result != jvmtiError_JVMTI_ERROR_NONE {
			panic!("{} ({})", obfstr!("Couldn't add jvmti capabilities"), result);
		}
	}
	
	{
		let mut callbacks: jvmtiEventCallbacks = zeroed();
		callbacks.ClassFileLoadHook = Some(load_hook_handler);
		callbacks.ThreadStart = Some(on_thread_start);
		
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
		let result = (**jvmti).SetEventNotificationMode.unwrap()(jvmti, jvmtiEventMode_JVMTI_ENABLE, jvmtiEvent_JVMTI_EVENT_THREAD_START, null_mut());
		if result != jvmtiError_JVMTI_ERROR_NONE {
			panic!("{} ({})", obfstr!("Couldn't enable jvmti callbacks"), result);
		}
	}
	
	if !attach_thread.is_null() {
		shutdown_attach_listener(jvmti, attach_thread);
	}
	
	println!("{}", obfstr!("Raion native libary initialisation finished (4)"));
}

pub unsafe extern "C" fn load_hook_handler(
	jvmti: *mut jvmtiEnv,
	jni_env: *mut *const JNINativeInterface_,
	_class_being_redefined: jclass,
	_loader: jobject,
	name: *const c_char,
	_protection_domain: jobject,
	_class_data_len: jint,
	_class_data: *const c_uchar,
	new_class_data_len: *mut jint,
	new_class_data: *mut *mut c_uchar) {
	if name.is_null() {
		return;
	}
	
	let env: JNIEnv = JNIEnv::from_raw(jni_env).unwrap();
	
	let map_obj = G_BYTEMAP_REF.as_ref().unwrap().as_obj();
	
	let class = env.auto_local(env.find_class("java/util/Map").unwrap());
	let j_name = env.auto_local(env.new_string(JNIStr::from_ptr(name).to_owned()).unwrap());
	let remove_method = env.get_method_id(&class, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;").unwrap();
	
	let result = env.call_method_unchecked(
		map_obj,
		remove_method,
		JavaType::Object("java/lang/Object".into()),
		&[JValue::Object(j_name.as_obj())],
	);
	if result.is_err() {
		return;
	}
	
	let bytes = result.unwrap().l().unwrap();
	if bytes.is_null() {
		return;
	}
	println!("Begin Succ {}", CStr::from_ptr(name).to_str().unwrap());
	
	let byte_num = env.get_array_length(bytes.into_inner()).unwrap();
	let c_bytes = env.get_byte_array_elements(bytes.into_inner()).unwrap().0;
	assert!(!c_bytes.is_null(), "Uh oh, bytes couldnt be fetched");
	
	{
		let result = (**jvmti).Allocate.unwrap()(jvmti, byte_num as i64, new_class_data);
		if result != jvmtiError_JVMTI_ERROR_NONE {
			panic!("{} ({})", obfstr!("Couldn't allocate my stuffs"), result);
		}
	}
	*new_class_data_len = byte_num;
	
	for i in 0..(byte_num as isize) {
		let ptr = (*new_class_data).offset(i);
		*ptr = *(c_bytes.offset(i)) as u8;
	}
	
	env.release_byte_array_elements(bytes.into_inner(), &mut *c_bytes, ReleaseMode::NoCopyBack).unwrap();
	
	println!("End Succ {}", CStr::from_ptr(name).to_str().unwrap());
}


