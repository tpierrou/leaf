use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};

extern "C" {
    fn leaf_run_with_options(
        rt_id: u16,
        config_path: *const c_char,
        auto_reload: bool,
        multi_thread: bool,
        auto_threads: bool,
        threads: i32,
        stack_size: i32,
    ) -> i32;

    fn leaf_shutdown(rt_id: u16) -> bool;
}

static mut RUNTIME_ID_CLIENT: u16 = 1;

#[no_mangle]
pub extern "C" fn startLeaf(config_path: *const c_char) -> c_int {
    if config_path.is_null() {
        return 1; // ERR_CONFIG_PATH
    }

    let config_str = unsafe { CStr::from_ptr(config_path) };
    let config_c = match CString::new(config_str.to_bytes()) {
        Ok(c) => c,
        Err(_) => return 1,
    };

    unsafe {
        leaf_run_with_options(
            RUNTIME_ID_CLIENT,
            config_c.as_ptr(),
            true,  // auto_reload
            true,  // multi_thread
            true,  // auto_threads
            4,     // threads (ignored with auto_threads)
            1 * 1024 * 1024, // stack size
        )
    }
}

#[no_mangle]
pub extern "C" fn stopLeaf() -> bool {
    unsafe { leaf_shutdown(RUNTIME_ID_CLIENT) }
}
