use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPALL, PROCESSENTRY32, Process32First, Process32Next};
use winapi::shared::minwindef::DWORD;
use crate::process::process::Process;
use crate::process::gather_process_info::gather_process_info;

pub fn get_process_list() -> Vec<Process> {
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
                let process = gather_process_info(&pe32);

                processes.push(process);
            }
        }
    };
    return processes;
}