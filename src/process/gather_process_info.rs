use winapi::um::tlhelp32::PROCESSENTRY32;
use crate::process::process::Process;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
use winapi::um::psapi::{PROCESS_MEMORY_COUNTERS, GetProcessMemoryInfo};
use winapi::shared::minwindef::DWORD;

pub unsafe fn gather_process_info(process_entry: &PROCESSENTRY32) -> Process {
    let name = std::ffi::CStr::from_ptr(&process_entry.szExeFile[0])
        .to_string_lossy()
        .into_owned();

    let pid = DWORD::from(process_entry.th32ProcessID.clone());
    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);

    let mut proc_mem_counters = std::mem::zeroed::<PROCESS_MEMORY_COUNTERS>();
    GetProcessMemoryInfo(handle, &mut proc_mem_counters, std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD);

    return Process {
        name,
        pid: process_entry.th32ProcessID,
        user: "SYSTEM".to_string(),
        gid: 0,
        parent_pid: process_entry.th32ParentProcessID,
        thread_count: process_entry.cntThreads,
        page_fault_count: proc_mem_counters.PageFaultCount as u32,
        peak_working_set_size: proc_mem_counters.PeakWorkingSetSize as u32,
        working_set_size: proc_mem_counters.WorkingSetSize as u32,
        quota_peak_paged_pool_usage: proc_mem_counters.QuotaPeakPagedPoolUsage as u32,
        quota_paged_pool_usage: proc_mem_counters.QuotaPagedPoolUsage as u32,
        quota_peak_non_paged_pool_usage: proc_mem_counters.QuotaPeakNonPagedPoolUsage as u32,
        quota_non_paged_pool_usage: proc_mem_counters.QuotaNonPagedPoolUsage as u32,
        page_file_usage: proc_mem_counters.PagefileUsage as u32,
        peak_page_file_usage: proc_mem_counters.PeakPagefileUsage as u32
    }
}