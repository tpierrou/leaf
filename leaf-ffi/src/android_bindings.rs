use jni::{
    objects::{JClass, JString, GlobalRef, JObject},
    JNIEnv,
};
use jni::sys::jobject;
use std::sync::Mutex;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use libc::dup;

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

use std::sync::OnceLock;
use jni::JavaVM;
use leaf_jni_core::save_vm;
use jni::sys::{jint, JNI_VERSION_1_6};


static JVM: OnceLock<JavaVM> = OnceLock::new();


#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut std::ffi::c_void) -> jint {
    save_vm(vm);
    JNI_VERSION_1_6
}
