use winapi::um::sysinfoapi::{SYSTEM_INFO, GetSystemInfo};
use winapi::um::winreg::{RegOpenKeyW, HKEY_LOCAL_MACHINE, RegOpenKeyA};
use winapi::shared::minwindef::{HKEY};
use winapi::_core::ptr::null_mut;
use winapi::um::winnt::LPCWSTR;
use std::ffi::{CString, OsStr};
use std::ffi::OsString;
use std::os::windows::prelude::*;
use winreg::RegKey;

pub fn get_cpu_name() -> String {
    unsafe {
        let mut systemInfo = std::mem::zeroed::<SYSTEM_INFO>();

        GetSystemInfo(&mut systemInfo);

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        match hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0\\") {
            Ok(key) => {
                let pns: String = key.get_value("ProcessorNameString").unwrap();
                return pns.trim().to_string() + " (" + &systemInfo.dwNumberOfProcessors.to_string() + " Cores)"
            }
            Err(e) => {
                return "".to_string();
            }
        }
    }
}