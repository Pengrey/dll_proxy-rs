use windows::{
    Win32::System::Threading::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW},
    core::{PCWSTR, PWSTR},
};

use std::{thread, time::Duration};

pub fn go() {
    thread::sleep(Duration::from_secs(10));

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
