extern crate kernel32;
extern crate winapi;

use winapi::{
    DWORD, // <1> u32
    HANDLE, // <2> Pointer types for various internal APIs without an associated type. In Rust, defined in `std::os::raw::c_void`
    LPVOID, // <2>
    PVOID,  // <3> 
    SIZE_T, // <4> u64 (usize on this machine)
    LPSYSTEM_INFO, // <5> A pointer to a SYSTEM_INFO struct
    SYSTEM_INFO, // <6> Some structs defined by Windows internally 
    MEMORY_BASIC_INFORMATION, // <6>
};

fn main() {
    let this_pid: DWORD;                          // <7> These variables will be initialized from within `unsafe` blocks. To make them accessible in the outer scope, they need to be defined here.
    let this_proc: HANDLE;                        // <7>
    let min_app_addr: LPVOID;                     // <7>
    let max_app_addr: LPVOID;                     // <7>
    let mut base_addr: PVOID;                     // <7>
    let mut proc_info: SYSTEM_INFO;               // <7>
    let mut mem_info: MEMORY_BASIC_INFORMATION;   // <7>

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    unsafe { // <8> This block guarantees that all memory is initialized 
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe { // <9> This block of code is where system calls are made
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO); // <10> Rather than use a return value, this function makes use of a C idiom to provide its result to the caller. We provide a pointer to some pre-defined struct, then read that struct's new values once the function has returned to see the results. 
    };

    min_app_addr = proc_info.lpMinimumApplicationAddress; // <11> Renaming these variables for convienence.
    max_app_addr = proc_info.lpMaximumApplicationAddress; // <11>

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_app_addr, max_app_addr);

    
    loop { // <12> This loop does the work of scanning through the address space 
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(this_proc, base_addr,
                                      &mut mem_info, MEMINFO_SIZE as SIZE_T) 
        };
                
        if rc == 0 {
            break
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
        }
}