use libc;
use std::ffi::CString;
use std::{mem, os::raw::c_void};

pub fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
pub fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
pub fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
pub fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

pub unsafe fn get_gl_string(name: gl::types::GLenum) -> String {
    std::ffi::CStr::from_ptr(gl::GetString(name) as *mut libc::c_char)
        .to_string_lossy()
        .to_string()
}

// Debug callback to panic upon enountering any OpenGL error
pub extern "system" fn debug_callback(
    source: u32,
    e_type: u32,
    id: u32,
    severity: u32,
    _length: i32,
    msg: *const libc::c_char,
    _data: *mut std::ffi::c_void,
) {
    if e_type != gl::DEBUG_TYPE_ERROR {
        return;
    }
    if severity == gl::DEBUG_SEVERITY_HIGH
        || severity == gl::DEBUG_SEVERITY_MEDIUM
        || severity == gl::DEBUG_SEVERITY_LOW
    {
        let severity_string = match severity {
            gl::DEBUG_SEVERITY_HIGH => "high",
            gl::DEBUG_SEVERITY_MEDIUM => "medium",
            gl::DEBUG_SEVERITY_LOW => "low",
            _ => "unknown",
        };
        unsafe {
            let string = CString::from_raw(msg as *mut libc::c_char);
            let error_message = String::from_utf8_lossy(string.as_bytes()).to_string();
            panic!(
                "{}: Error of severity {} raised from {}: {}\n",
                id, severity_string, source, error_message
            );
        }
    }
}
