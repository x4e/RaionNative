
#[cfg(target_os = "windows")]
extern crate winreg;
#[cfg(target_os = "windows")]
use winreg::enums::HKEY_LOCAL_MACHINE;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use obfstr::{obfstr};

#[cfg(target_os = "windows")]
pub fn get_id() -> std::string::String {
	let hive = winreg::RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(obfstr!("\\\\SOFTWARE\\Microsoft\\Cryptography")).expect(obfstr!("Failed to open subkey"));
	let id = hive.get_value(obfstr!("MachineGuid")).expect(obfstr!("Failed to get MachineGuid"));
	return id
}

#[cfg(target_os = "macos")]
pub fn get_id() -> std::string::String {
	let output = Command::new(obfstr!("ioreg"))
		.arg(obfstr!("-rd1")).arg(obfstr!("-c")).arg(obfstr!("IOExpertPlatformDevice"))
		.output().expect(obfstr!("Failed to get HWID"));
	return String::from_utf8(output.stdout).unwrap();
}

#[cfg(target_os = "linux")]
pub fn get_id() -> std::string::String {
	let mut id = String::new();
	let mut path: String = obfstr!("/etc/machine-id").to_owned();
	if Path::new(&path).exists() {
		File::open(&path).unwrap().read_to_string(&mut id).unwrap();
	} else {
		path = obfstr!("/etc/machine-id").to_owned();
		if Path::new(&path).exists() {
			File::open(&path).unwrap().read_to_string(&mut id).unwrap();
		}
	}
	
	let output = Command::new(obfstr!("lsblk"))
		.arg(obfstr!("-nro")).arg(obfstr!("SERIAL"))
		.output().expect(obfstr!("Failed to get HWID"));
	let disk_serial = String::from_utf8(output.stdout);
	
	if disk_serial.is_ok() {
		id.push_str(disk_serial.unwrap().as_str());
		return id;
	}
	return id;
}

#[cfg(target_os = "freebsd")]
#[cfg(target_os = "dragonfly")]
#[cfg(target_os = "openbsd")]
#[cfg(target_os = "netbsd")]
pub fn get_id() -> std::string::String {
	unimplemented!("*BSD support is not implemented")
}
