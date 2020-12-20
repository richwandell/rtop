use winapi::ctypes::c_void;

pub struct Process {
    pub handle: *mut c_void,
    pub name: String,
    pub pid: u32,
    pub user: String,
    pub gid: u32,
    pub parent_pid: u32,
    pub thread_count: u32,
    pub base_priority: u32,
    pub page_fault_count: u32,
    pub peak_working_set_size: u32,
    pub working_set_size: u32,
    pub quota_peak_paged_pool_usage: u32,
    pub quota_paged_pool_usage: u32,
    pub quota_peak_non_paged_pool_usage: u32,
    pub quota_non_paged_pool_usage: u32,
    pub page_file_usage: u32,
    pub peak_page_file_usage: u32,
    pub cpu_usage: f32
}