use libc::c_char;
use libc::c_int;
use std::ffi::CString;

unsafe extern "C" {
    fn count_spaces(s: *const c_char) -> c_int;
}

fn call_count_spaces(s: &str) -> i32 {
    let c_str = CString::new(s).expect("CString conversion failure");
    unsafe { count_spaces(c_str.as_ptr()) as i32 }
}

fn main() {
    println!("{}", call_count_spaces("Can this really count the spaces?"));
}
