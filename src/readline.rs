// Copied from: https://github.com/kanaka/mal/blob/master/rust/src/readline.rs
extern crate libc;

use std::ffi::{CStr, CString};
use std::str;

mod ext_readline {
    extern crate libc;
    use self::libc::c_char;
    #[link(name = "readline")]
    extern {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

pub fn add_history(line: &str) {
    unsafe {
        ext_readline::add_history(CString::new(line).unwrap().as_ptr());
    }
}

pub fn readline(prompt: &str) -> Option<String> {
    let cprmt = CString::new(prompt).unwrap();
    unsafe {
        let ptr = ext_readline::readline(cprmt.as_ptr());
        if ptr.is_null() {  // user pressed Ctrl-D
            None
        } else {
            let ret = str::from_utf8(CStr::from_ptr(ptr).to_bytes());
            let ret = ret.ok().map(|s| s.to_string());
            libc::free(ptr as *mut _);
            return ret;
        }
    }
}

// --------------------------------------------

pub fn lish_readline (prompt: &str) -> Option<String> {
    let line = readline(prompt);
    if let Some(ref s) = line {
        add_history(s);
    }
    line
}
