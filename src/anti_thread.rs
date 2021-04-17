use crate::jvmti::{jvmtiThreadInfo, jvmtiEnv, jthread, jvmtiError_JVMTI_ERROR_NONE};
use std::ffi::{CStr};
use jni::sys::{JNINativeInterface_};
use std::mem::zeroed;
use std::os::raw::{c_uchar};
use obfstr::{obfstr};

pub unsafe fn shutdown_attach_listener(jvmti: *mut jvmtiEnv, thread: jthread) {
	(**jvmti).SuspendThread.unwrap()(jvmti, thread);
}

/*impl Display for jvmtiThreadInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"(Name: {}, Daemon: {}, Priority: {}, Has Group: {})",
			unsafe { CStr::from_ptr(self.name).to_str().unwrap() },
			self.is_daemon,
			self.priority,
			!self.thread_group.is_null(),
		)
	}
}

impl Display for jvmtiThreadGroupInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"(Name: {}, Daemon: {}, MaxPriority: {}, Has Parent: {})",
			unsafe { CStr::from_ptr(self.name).to_str().unwrap() },
			self.is_daemon,
			self.max_priority,
			!self.parent.is_null()
		)
	}
}*/

pub unsafe extern "C" fn on_thread_start(jvmti: *mut jvmtiEnv, _jni: *mut *const JNINativeInterface_, thread: jthread) {
	{
		let mut info: jvmtiThreadInfo = zeroed();
		let result = (**jvmti).GetThreadInfo.unwrap()(jvmti, thread, &mut info);
		if result != jvmtiError_JVMTI_ERROR_NONE {
			println!("{} {}", obfstr!("Couldnt get thread info"), result);
		} else if !info.name.is_null() {
			let name = CStr::from_ptr(info.name).to_str().unwrap();
			// JMX and RMI are used for remote instrumentation
			if name.contains(obfstr!("RMI")) || name.contains(obfstr!("JMX")) {
				println!("{}", obfstr!("[RAION] This incident has been reported"));
				std::process::exit(1);
			}
			dealloc(jvmti, info.name);
		}
	}
	/*{
		let mut info: jvmtiThreadGroupInfo = zeroed();
		let result = (**jvmti).GetThreadGroupInfo.unwrap()(jvmti, thread, &mut info);
		if result != jvmtiError_JVMTI_ERROR_NONE {
			println!("Couldnt get thread group info {}", result);
		} else if !info.name.is_null() {
			println!("Thread parent {}", info);
			dealloc(jvmti, info.name);
			//assert_eq!((**jvmti).Deallocate.unwrap()(jvmti, info.name.borrow_mut() as *mut *mut i8 as *mut c_uchar), jvmtiError_JVMTI_ERROR_NONE);
		}
	}*/
}

pub unsafe fn dealloc<T>(jvmti: *mut jvmtiEnv, ptr: *mut T) {
	(**jvmti).Deallocate.unwrap()(jvmti, ptr as *mut c_uchar);
}
