use crate::bindings;

pub struct ProcessInfo {
    /// current pid
    pub pid: u32,
    /// Current host CPU quantity
    pub cpu_count: u32,
    /// totoal cpu usage
    pub percent_cpu: f64,
    pub percent_cpu_user: f64,
    pub percent_cpu_system: f64,
    /// totoal memory usage
    pub percent_mem: f64,
    pub percent_cpu_process: f64,
    pub percent_mem_process: f64,
    pub mem_total: u64,
    pub mem_free: u64,
}

///
/// Get current process infomation for monitor.
///
/// # Examples
///
/// ```
/// use crate::proc_monitor;
///
/// let p = proc_monitor::get_info();
/// println!(
///    "cpus: {}, pid: {}, mem toal: {}, mem free: {}, cpu: {:.2}, usr: {:.2}, sys: {:.2}, mem: {:.2}, process cpu: {:.2}, process mem: {:.2}",
///    p.cpu_count,
///    p.pid,
///    p.mem_total,
///    p.mem_free,
///    p.percent_cpu,
///    p.percent_cpu_user,
///    p.percent_cpu_system,
///    p.percent_mem,
///    p.percent_cpu_process,
///    p.percent_mem_process
/// );
///
///
/// ```
///
pub fn get_info() -> ProcessInfo {
    unsafe {
        let mut p = bindings::proc_t {
            pid: 0,
            percent_cpu: 0.0,
            percent_cpu_user: 0.0,
            percent_cpu_system: 0.0,
            percent_mem: 0.0,
            percent_cpu_process: 0.0,
            percent_mem_process: 0.0,
            mem_total: 0,
            mem_free: 0,
            cpu_count: 0,
        };
        bindings::get_proc_info(&mut p);

        ProcessInfo {
            pid: p.pid,
            cpu_count: p.cpu_count,
            percent_cpu: p.percent_cpu,
            percent_cpu_user: p.percent_cpu_user,
            percent_cpu_system: p.percent_cpu_system,
            percent_mem: p.percent_mem,
            percent_cpu_process: p.percent_cpu_process,
            percent_mem_process: p.percent_mem_process,
            mem_total: p.mem_total,
            mem_free: p.mem_free,
        }
    }
}

///
/// Free memory on exit
/// 
/// # Examples
///
/// ```
/// use crate::proc_monitor;
///
/// proc_monitor::free_memory();
/// 
/// ```
///
pub fn free_memory() {
    unsafe {
        bindings::free_memory();
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_proc() {
        let p = get_info();
        println!(
            "cpus: {}, pid: {}, mem toal: {}, mem free: {}, cpu: {:.2}, usr: {:.2}, sys: {:.2}, mem: {:.2}, process cpu: {:.2}, process mem: {:.2}",
            p.cpu_count,
            p.pid,
            p.mem_total,
            p.mem_free,
            p.percent_cpu,
            p.percent_cpu_user,
            p.percent_cpu_system,
            p.percent_mem,
            p.percent_cpu_process,
            p.percent_mem_process
        );

        free_memory();
    }
}
