Get current process infomation for monitor.

## Example


```
use crate::proc_monitor;

let p = proc_monitor::get_info();
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
```