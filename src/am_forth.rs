use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::str;
use std::io::stdout;
use std::io::Write;
use libc;

lazy_static! {
    static ref OUTPUT_STREAM: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

extern "C" {
    fn amf_init_parser() -> *mut c_void;
    fn amf_parse_string(parser: *mut c_void, s: *const c_char);
}

#[derive(Clone, Copy)]
pub struct AMForth {
    parser: *mut c_void,
}

#[repr(C)]
struct StateAndString {
    parser: *mut c_void,
    string: *const c_char,
}

impl AMForth {
    pub fn init() -> AMForth {
        return AMForth{parser: unsafe {amf_init_parser()}};
    }

    pub fn parse_string(&self, s: &str) {
        let mut running_thread: libc::pthread_t = 0;
        let c_s = CString::new(s).expect("CString::new failed");
        let mut thread_arguments = StateAndString{
            parser: self.parser,
            string: c_s.as_ptr()
        };
        let arg_ptr: *mut c_void = &mut thread_arguments as *mut _ as *mut c_void;

        extern "C" fn run_thread(arg: *mut c_void) -> *mut c_void {
           let arg: &mut StateAndString = unsafe { &mut *(arg as *mut StateAndString) }; 
           unsafe {
               amf_parse_string(arg.parser, arg.string);
           }
           std::ptr::null_mut()
        }

        unsafe {
            let _ = libc::pthread_create(&mut running_thread, std::ptr::null(), run_thread, arg_ptr);
            let mut thread_return: usize = 0;
            let thread_return_ptr: *mut usize = &mut thread_return;
            let thread_return_void: *mut *mut c_void = thread_return_ptr as *mut *mut c_void;
            let _ = libc::pthread_join(running_thread, thread_return_void);

        }

        /*
        let c_s = CString::new(s).expect("CString::new failed");
        unsafe {
            amf_parse_string(self.parser, c_s.as_ptr());
        }
        */
    }

    pub fn get_output(&self) -> String {
        let buff_copy = OUTPUT_STREAM.lock().unwrap().as_slice().to_vec();
        *OUTPUT_STREAM.lock().unwrap() = vec![];
        str::from_utf8(buff_copy.as_slice()).unwrap().to_string()
    }
}

#[no_mangle]
pub extern "C" fn amf_input() -> c_char {
    return 0;
}

#[no_mangle]
pub extern "C" fn amf_output(c: u8) {
    let c_as_slice = &[c];
    let _ = stdout().write_all(c_as_slice);
    OUTPUT_STREAM.lock().unwrap().push(c);
}

#[no_mangle]
pub extern "C" fn amf_init_io() {
}

#[no_mangle]
pub extern "C" fn amf_clean_io() {
}

    //for b in unsafe { cstr.to_bytes() as &[i8] } {
#[no_mangle]
pub extern "C" fn amf_print_string(s: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(s) };
    for b in cstr.to_bytes() {
        amf_output(*b);
    }
    // Get copy-on-write Cow<'_, str>, then guarantee a freshly-owned String allocation
    //println!("{}", String::from_utf8_lossy(cstr.to_bytes()).to_string());
}

