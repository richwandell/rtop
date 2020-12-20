use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPALL, PROCESSENTRY32, Process32First, Process32Next};
use winapi::shared::minwindef::DWORD;
use crate::process::process::Process;
use crate::process::get_process_mem::get_process_mem;
use crate::process::get_process_name::get_process_name;
use crate::process::get_process_username::get_process_username;
use winapi::um::winnt::{PROCESS_ALL_ACCESS, PROCESS_QUERY_LIMITED_INFORMATION};
use winapi::um::processthreadsapi::OpenProcess;
use sysinfo::{ProcessExt, System, SystemExt};
use std::collections::HashMap;
use sysinfo::Process as SIProcess;

pub fn get_process_list(sys_proc_info: &HashMap<usize, SIProcess>) -> Vec<Process> {

    let mut processes = vec![];

    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPALL, 0);
        let mut pe32 = std::mem::zeroed::<PROCESSENTRY32>();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;

        Process32First(handle, &mut pe32);
        loop {
            if Process32Next(handle, &mut pe32) == 0 {
                break;
            } else {
                if pe32.th32ProcessID == 0 {
                    continue;
                }
                let name = get_process_name(&pe32);
                let pid = DWORD::from(pe32.th32ProcessID.clone());
                let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 1, pid.clone());
                get_process_username(&pe32, process_handle);
                let proc_mem_cnt = get_process_mem(&pe32, process_handle);

                let mut cpu_usage = 0.0;
                if let Some(process) = sys_proc_info.get(&(pid as usize)) {
                    cpu_usage = process.cpu_usage();
                }

                processes.push(Process {
                    handle: handle,
                    name,
                    pid: pe32.th32ProcessID.clone(),
                    user: "SYSTEM".to_string(),
                    gid: 0,
                    parent_pid: pe32.th32ParentProcessID,
                    thread_count: pe32.cntThreads,
                    base_priority: pe32.pcPriClassBase as u32,
                    page_fault_count: proc_mem_cnt.PageFaultCount as u32,
                    peak_working_set_size: proc_mem_cnt.PeakWorkingSetSize as u32,
                    working_set_size: proc_mem_cnt.WorkingSetSize as u32,
                    quota_peak_paged_pool_usage: proc_mem_cnt.QuotaPeakPagedPoolUsage as u32,
                    quota_paged_pool_usage: proc_mem_cnt.QuotaPagedPoolUsage as u32,
                    quota_peak_non_paged_pool_usage: proc_mem_cnt.QuotaPeakNonPagedPoolUsage as u32,
                    quota_non_paged_pool_usage: proc_mem_cnt.QuotaNonPagedPoolUsage as u32,
                    page_file_usage: proc_mem_cnt.PagefileUsage as u32,
                    peak_page_file_usage: proc_mem_cnt.PeakPagefileUsage as u32,
                    cpu_usage: cpu_usage
                });
            }
        }
    };
    return processes;
}