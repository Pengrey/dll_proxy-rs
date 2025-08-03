#![no_main]

use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

type BOOL = i32;
pub const TRUE: BOOL = 1;

#[unsafe(export_name = "DllMain")]
extern "system" fn dll_main(_: usize, dw_reason: u32, _: usize) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            #[cfg(feature = "debug")] {
                use windows::{core::{w, PCWSTR}, Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK}};

                if let Ok(path) = std::env::current_exe() {
                    let wide_path: Vec<u16> = path.to_string_lossy().to_string().encode_utf16().chain(Some(0)).collect();

                    unsafe { MessageBoxW(None, PCWSTR(wide_path.as_ptr()), w!("Loaded In:") , MB_OK,); }
                }
            }

            // Execute payload
            payload::go();
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }

    TRUE
}
