/*#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
use winapi::um::debugapi::IsDebuggerPresent;

#[cfg(any(target_os = "linux", target_os = "macos"))]
extern crate libc;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use libc::{ptrace, c_void, c_long, siginfo_t, c_uint, pid_t};

#[cfg(target_os = "windows")]
fn is_debugger_present() -> bool {
	IsDebuggerPresent()
}

const PTRACE_TRACEME: c_uint = 0;
const PTRACE_DETACH: c_uint = 17;

#[cfg(any(target_os = "linux", target_os = "macos"))]
static mut DEBUGGER_PRESENT: Option<bool> = None;

#[cfg(any(target_os = "linux", target_os = "macos"))]
unsafe fn is_debugger_present() -> bool {
	return match DEBUGGER_PRESENT {
		Some(x) => x,
		None => {
			if ptrace(PTRACE_TRACEME, 0, 1, 0) < 0 {
				DEBUGGER_PRESENT = Some(true);
				true
			} else {
				ptrace(PTRACE_DETACH, 0, 1, 0);
				DEBUGGER_PRESENT = Some(false);
				false
			}
		}
	}
}*/

pub fn is_debugger_present() -> bool { false }

pub unsafe fn check_debugger() -> bool {
	if is_debugger_present() {
		return true
	}
	false
}
