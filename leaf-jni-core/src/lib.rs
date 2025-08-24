use std::sync::OnceLock;
use jni::JavaVM;

static JVM: OnceLock<JavaVM> = OnceLock::new();

pub fn save_vm(vm: JavaVM) {
    let _ = JVM.set(vm); // ignore if already set
}

pub fn get_saved_vm() -> &'static JavaVM {
    JVM.get().expect("JavaVM has not been initialized.")
}
