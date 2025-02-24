use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS, PROCESSENTRY32W
};
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows_sys::Win32::System::Threading::GetCurrentThreadId;

pub fn main() {

    // list all processes
    let processes: isize = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if processes == INVALID_HANDLE_VALUE {
        println!("invalid process, unable to retrieve data");
        return;
    } else {
        let mut process_single: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
        process_single.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32; 

        if unsafe { Process32FirstW(processes, &mut process_single) } == 0 {
            println!("unable to get the process");
            return;
        }

        loop {
            let name = String::from_utf16_lossy(&process_single.szExeFile);
            println!("{}, {}", unsafe { GetCurrentThreadId() }, name);

            if unsafe { Process32NextW(processes, &mut process_single) } == 0 {
                break; 
            }
        }
    }


    // simple testing the process id
    let current_id = unsafe { GetCurrentThreadId() };
    println!("{}", current_id);
}