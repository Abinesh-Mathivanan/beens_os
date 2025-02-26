use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS, PROCESSENTRY32W,
};
use windows_sys::Win32::Foundation::{INVALID_HANDLE_VALUE, CloseHandle, GetLastError};
use windows_sys::Win32::System::Threading::{GetCurrentThreadId, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, GetCurrentProcessId};
use windows_sys::Win32::System::ProcessStatus::K32GetProcessImageFileNameW;

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

    // let's open a process and print info
    let random_process_id: u32 = unsafe {
        GetCurrentProcessId()
    };
    let open_random_process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, random_process_id) };
    if open_random_process == INVALID_HANDLE_VALUE {
        let last_error = unsafe {
            GetLastError()
        };
        println!("unable to access process info {}", last_error);
        return; 
    } else {
        println!("process info retrieved. {}", open_random_process);
        let mut process_image = [0u16; 260];

        let process_result = unsafe {
            K32GetProcessImageFileNameW(open_random_process, process_image.as_mut_ptr(), process_image.len() as u32,)
        };

        if process_result == 0 {
            println!("unable to retrieve data");
            return; 
        }

        let unicode_to_string_length = process_image.iter().position(|&c| c == 0).unwrap_or(process_image.len());
        let processed_file_path = OsString::from_wide(&process_image[..unicode_to_string_length]);
        println!("process result {:?}", processed_file_path.clone());

        unsafe { CloseHandle(open_random_process)};
    }


    // simple testing the process id
    let current_id = unsafe { GetCurrentThreadId() };
    println!("current id of the process: {}", current_id);
}

/*
    based on the observation, i found that the process file name is represented in UTF-16.
    and meow
*/