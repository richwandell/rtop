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
    // return Process {
    //     handle,
    //     name,
    //     pid: process_entry.th32ProcessID,
    //     user: "SYSTEM".to_string(),
    //     gid: 0,
    //     parent_pid: process_entry.th32ParentProcessID,
    //     thread_count: process_entry.cntThreads,
    //     base_priority: process_entry.pcPriClassBase as u32,
    //     page_fault_count: proc_mem_counters.PageFaultCount as u32,
    //     peak_working_set_size: proc_mem_counters.PeakWorkingSetSize as u32,
    //     working_set_size: proc_mem_counters.WorkingSetSize as u32,
    //     quota_peak_paged_pool_usage: proc_mem_counters.QuotaPeakPagedPoolUsage as u32,
    //     quota_paged_pool_usage: proc_mem_counters.QuotaPagedPoolUsage as u32,
    //     quota_peak_non_paged_pool_usage: proc_mem_counters.QuotaPeakNonPagedPoolUsage as u32,
    //     quota_non_paged_pool_usage: proc_mem_counters.QuotaNonPagedPoolUsage as u32,
    //     page_file_usage: proc_mem_counters.PagefileUsage as u32,
    //     peak_page_file_usage: proc_mem_counters.PeakPagefileUsage as u32
    // }
}