use winapi::_core::ptr::{null, null_mut};
use winapi::ctypes::{c_void, c_ulong};
use winapi::shared::lmcons::UNLEN;
use winapi::shared::minwindef::{DWORD, LPDWORD, MAX_PATH, BYTE};
use winapi::shared::winerror::{ERROR_INSUFFICIENT_BUFFER, ERROR_NOACCESS};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::tlhelp32::PROCESSENTRY32;
use winapi::um::winbase::LookupAccountSidA;
use winapi::um::winnt::{HANDLE, LPSTR, PHANDLE, PROCESS_ALL_ACCESS, PROCESS_QUERY_INFORMATION, PSECURITY_DESCRIPTOR, PSECURITY_INFORMATION, PSID_NAME_USE, PTOKEN_USER, STANDARD_RIGHTS_READ, TOKEN_READ, TokenUser, TOKEN_USER, SID_NAME_USE, CHAR};
use winapi::um::winuser::GetUserObjectSecurity;
use std::ffi::{CStr, CString};

pub unsafe fn get_process_username_and_domain_name(process_entry: &PROCESSENTRY32, handle: *mut c_void) -> (String, String){

    let mut process_token_handle: HANDLE = null_mut();
    if OpenProcessToken(handle, TOKEN_READ, &mut process_token_handle) != 0 {
        let mut token_user = libc::malloc(std::mem::size_of::<PTOKEN_USER>()) as *mut c_void;
        let mut ret_size = 0;
        GetTokenInformation(
            process_token_handle as HANDLE,
            TokenUser,
            token_user,
            0,
            &mut ret_size
        );

        if ret_size > 0 {
            token_user = libc::malloc(ret_size as usize);
            if GetTokenInformation(
                process_token_handle as HANDLE,
                TokenUser,
                token_user,
                ret_size,
                &mut ret_size
            ) != 0 {
                let token_user_struct: TOKEN_USER = *(token_user as *mut TOKEN_USER);
                let sid = token_user_struct.User.Sid;
                let mut user_name_ptr = CString::from_vec_unchecked(vec![0; UNLEN as usize]).into_raw();
                let mut name_length = UNLEN;
                let mut domain_name_ptr = CString::from_vec_unchecked(vec![0; MAX_PATH as usize]).into_raw();
                let mut domain_length = MAX_PATH as c_ulong;
                let mut name_use = 1 as SID_NAME_USE;
                LookupAccountSidA(
                    null(),
                    sid,
                    user_name_ptr,
                    &mut name_length,
                    domain_name_ptr,
                    &mut domain_length,
                    &mut name_use
                );
                let user_name = CString::from_raw(user_name_ptr).to_str().unwrap().to_owned();
                let domain_name = CString::from_raw(domain_name_ptr).to_str().unwrap().to_owned();
                return (user_name, domain_name);
            }
        }
    }
    return ("SYSTEM".to_string(), "UNKNOWN".to_string());
}