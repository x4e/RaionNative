use crate::jvm::JVM_GetManagement;
use jni::sys::{JNIEnv, jobjectArray, jsize, jstring, jboolean};
use crate::jmm::{JMM_VERSION_1_0, JmmInterface};
use obfstr::{obfstr, obflocal};
use std::ffi::CStr;
use std::borrow::Borrow;

pub unsafe fn anti_launch_args(env: *mut JNIEnv) {
	let management = JVM_GetManagement(JMM_VERSION_1_0 as i32) as *mut JmmInterface;
	if management.is_null() {
		panic!("{}", obfstr!("Couldn't fetch jvm management"));
	}
	let args: jobjectArray = (*management).GetInputArgumentArray.unwrap()(env);
	if args.is_null() {
		panic!("{}", obfstr!("Couldn't fetch jvm arguments"));
	}
	
	let naughty_strings = vec![
		String::from(obfstr!("javaagent")),
		String::from(obfstr!("xdebug")),
		String::from(obfstr!("agentlib")),
		String::from(obfstr!("runjdwp")),
		String::from(obfstr!("noagent")),
		String::from(obfstr!("verbose")),
		String::from(obfstr!("proxyset")),
		String::from(obfstr!("proxyhost")),
		String::from(obfstr!("proxyport")),
		String::from(obfstr!("javax.net.ssl.truststore")),
		String::from(obfstr!("javax.net.ssl.truststorepassword")),
		String::from(obfstr!("startattachlistener")),
		String::from(obfstr!("bootclasspath"))
	];
	
	let arr_length: jsize = (**env).GetArrayLength.unwrap()(env, args);
	for i in 0..arr_length {
		let obj: jstring = (**env).GetObjectArrayElement.unwrap()(env, args, i);
		let mut is_copy: jboolean = 0;
		let cstr = (**env).GetStringUTFChars.unwrap()(env, obj, &mut is_copy);
		
		let string: String = match CStr::from_ptr(cstr).to_str() {
			Ok(x) => x,
			_ => continue
		}.to_lowercase();
		
		for naughty in &naughty_strings {
			let str: &str = naughty.borrow();
			if string.contains(str) {
				println!("{} ({})", obfstr!("[RAION] Please adjust your launch flags - this incident has been reported."), string);
				std::process::exit(1);
			}
		}
		
		(**env).ReleaseStringUTFChars.unwrap()(env, obj, cstr);
	}
}
