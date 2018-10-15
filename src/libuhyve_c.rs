use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn start_app(path_to_app_ptr: *const c_char) -> u16 {
    let path_to_app = unsafe { CStr::from_ptr(path_to_app_ptr) };

    match path_to_app.to_str() {
        Ok(str) => super::start_app(str).unwrap().code,
        Err(_) => 400,
    }
}

#[no_mangle]
pub extern "C" fn create_checkpoint(
    path_to_checkpoint_ptr: *const c_char,
    full_checkpoint: bool,
) -> u16 {
    let path_to_checkpoint = unsafe { CStr::from_ptr(path_to_checkpoint_ptr) };

    match path_to_checkpoint.to_str() {
        Ok(str) => super::create_checkpoint(str, full_checkpoint).unwrap().code,
        Err(_) => 400,
    }
}

#[no_mangle]
pub extern "C" fn load_checkpoint(path_to_checkpoint_ptr: *const c_char) -> u16 {
    let path_to_checkpoint = unsafe { CStr::from_ptr(path_to_checkpoint_ptr) };

    match path_to_checkpoint.to_str() {
        Ok(str) => super::load_checkpoint(str).unwrap().code,
        Err(_) => 400,
    }
}

#[no_mangle]
pub extern "C" fn migrate(
    destination: *const c_char,
    mig_type: *const c_char,
    mig_mode: *const c_char,
    use_odp: u8,
    prefetch: u8,
) -> u16 {
    let destination = unsafe { CStr::from_ptr(destination) }.to_str().unwrap();
    let mig_type = unsafe { CStr::from_ptr(mig_type) }.to_str().unwrap();
    let mig_mode = unsafe { CStr::from_ptr(mig_mode) }.to_str().unwrap();
    let use_odp = use_odp == 1;
    let prefetch = prefetch == 1;
    super::migrate(destination, mig_type, mig_mode, use_odp, prefetch)
        .unwrap()
        .code
}
