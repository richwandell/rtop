use winapi::um::tlhelp32::PROCESSENTRY32;

pub unsafe fn get_process_name(process_entry: &PROCESSENTRY32) -> String {
    std::ffi::CStr::from_ptr(&process_entry.szExeFile[0])
        .to_string_lossy()
        .into_owned()
}