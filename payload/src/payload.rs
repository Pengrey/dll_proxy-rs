use windows::{
    Win32::{
        System::Threading::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW},
        Foundation::{GetLastError, CloseHandle, ERROR_ALREADY_EXISTS},
    },
    core::{w, PCWSTR, PWSTR},
};

use std::{thread, time::Duration};

use windows::Win32::System::Threading::{CreateMutexW};

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
    thread::sleep(Duration::from_secs(10));

    if !create_mutex() {
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
