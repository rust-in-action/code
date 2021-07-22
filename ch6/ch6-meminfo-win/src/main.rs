use kernel32;
use winapi;

use winapi::{
    DWORD,                                        // <1>
    HANDLE,                                       // <2>
    LPVOID,                                       // <2>
    PVOID,                                        // <3>
    SIZE_T,                                       // <4>
    LPSYSTEM_INFO,                                // <5>
    SYSTEM_INFO,                                  // <6>
    MEMORY_BASIC_INFORMATION as MEMINFO,          // <6>
};

fn main() {
    let this_pid: DWORD;                          // <7>
    let this_proc: HANDLE;                        // <7>
    let min_addr: LPVOID;                         // <7>
    let max_addr: LPVOID;                         // <7>
    let mut base_addr: PVOID;                     // <7>
    let mut proc_info: SYSTEM_INFO;               // <7>
    let mut mem_info: MEMORY_BASIC_INFORMATION;   // <7>

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();

    unsafe {                                      // <8>
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe {                                        // <9>
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(                   // <10>
          &mut proc_info as LPSYSTEM_INFO          // <10>
        );                                         // <10>
    };

    min_addr = proc_info.lpMinimumApplicationAddress; // <11>
    max_addr = proc_info.lpMaximumApplicationAddress; // <11>

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);


    loop {                                         // <12>
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(              // <13>
                                    this_proc, base_addr,
                                    &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
