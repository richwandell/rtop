use winapi::um::processthreadsapi::{OpenProcess, OpenProcessToken};
use winapi::um::winnt::{STANDARD_RIGHTS_READ, PSECURITY_INFORMATION, PSECURITY_DESCRIPTOR, PHANDLE, TOKEN_READ, PTOKEN_USER, HANDLE, TokenUser, LPSTR, PSID_NAME_USE, PROCESS_ALL_ACCESS};
use winapi::shared::minwindef::{LPDWORD, DWORD, MAX_PATH};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::winerror::{ERROR_NOACCESS, ERROR_INSUFFICIENT_BUFFER};
use winapi::ctypes::c_void;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::shared::lmcons::UNLEN;
use winapi::_core::ptr::null;
use winapi::um::winbase::LookupAccountSidA;
use winapi::um::winuser::GetUserObjectSecurity;
use winapi::um::tlhelp32::PROCESSENTRY32;

pub unsafe fn get_process_username(process_entry: &PROCESSENTRY32, handle: *mut c_void) {

    // let mut sec_info = std::mem::zeroed::<PSECURITY_INFORMATION>();
    // let mut length = 0 as LPDWORD;
    // GetUserObjectSecurity(handle, sec_info, 0 as PSECURITY_DESCRIPTOR, 0 as DWORD, length);

    let mut process_token_handle = 0 as PHANDLE;
    if OpenProcessToken(handle, TOKEN_READ, process_token_handle) == 0 {
        if GetLastError() == ERROR_NOACCESS {
            println!("{}", "no access");
        } else {
            println!("{}", "some other error");
        }
    } else {
        println!("{}", "no error");
        let mut token_user = libc::malloc(std::mem::size_of::<PTOKEN_USER>()) as *mut c_void;
        let size = std::mem::size_of::<PTOKEN_USER>() as u32;
        let mut ret_size = size;
        GetTokenInformation(process_token_handle as HANDLE, TokenUser, token_user, size, &mut ret_size);

        let token_user_struct: PTOKEN_USER = *(token_user as *mut PTOKEN_USER);
        let user_sid = (*token_user_struct).User.Sid;

        let mut user_name = libc::malloc(UNLEN as usize) as LPSTR;
        let mut name_length = UNLEN as LPDWORD;
        let mut domain_name = DWORD::from(UNLEN) as LPSTR;
        let mut domain_length = DWORD::from(MAX_PATH as u32) as LPDWORD;
        let mut name_use = libc::malloc(std::mem::size_of::<PSID_NAME_USE>()) as PSID_NAME_USE;
        LookupAccountSidA(null(), user_sid, user_name, name_length, domain_name, domain_length, name_use);

        if ret_size as usize > 0 {
            println!("{}", "hi")
        }
        if GetLastError() == ERROR_INSUFFICIENT_BUFFER {
            println!("{}", "hi")
        }
    }
}