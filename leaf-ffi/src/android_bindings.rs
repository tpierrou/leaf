use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

use std::ffi::CString;
use std::os::raw::c_char;

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
pub extern "system" fn Java_com_example_staysafe_webprotection_LeafBridge_startLeaf(
    mut env: JNIEnv,
    _class: JClass,
    jconfig_path: JString,
) -> i32 {
    let config_path: String = match env.get_string(&jconfig_path) {
        Ok(s) => s.into(),
        Err(_) => return 1, // ERR_CONFIG_PATH
    };

    let config_c = match CString::new(config_path) {
        Ok(c) => c,
        Err(_) => return 1,
    };

    unsafe {
        leaf_run_with_options(
            RUNTIME_ID_CLIENT,
            config_c.as_ptr(),
            true,     // auto_reload
            true,      // multi_thread
            true,      // auto_threads
            4,         // threads (ignored if auto_threads is true)
            1 * 1024 * 1024, // stack size: 1MB
        )
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_staysafe_webprotection_LeafBridge_shutdownLeaf(
    _env: JNIEnv,
    _class: JClass,
) -> bool {
    unsafe { leaf_shutdown(RUNTIME_ID_CLIENT) }
}
