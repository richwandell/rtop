use winapi::um::tlhelp32::PROCESSENTRY32;
use crate::process::process::Process;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
use winapi::um::psapi::{PROCESS_MEMORY_COUNTERS, GetProcessMemoryInfo};
use winapi::shared::minwindef::DWORD;
use winapi::_core::ffi::c_void;

pub unsafe fn get_process_mem(process_entry: &PROCESSENTRY32, handle: *mut c_void) -> PROCESS_MEMORY_COUNTERS {

    let mut proc_mem_counters = std::mem::zeroed::<PROCESS_MEMORY_COUNTERS>();
    GetProcessMemoryInfo(handle, &mut proc_mem_counters, std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD);

    proc_mem_counters
}