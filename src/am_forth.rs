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
use std::time::Duration;
use std::thread;

lazy_static! {
    static ref OUTPUT_STREAM: Mutex<Vec<u8>> = Mutex::new(vec![]);
    static ref PTHREAD_RUNNING: Mutex<bool> = Mutex::new(false);
}

extern "C" {
    fn amf_init_parser() -> *mut c_void;
    fn amf_parse_string(parser: *mut c_void, s: *const c_char);
    //fn amf_clean_parser(parser: *mut c_void);
    fn cancellable_thread();
}

#[derive(Clone, Copy)]
pub struct AMForth {
    parser: *mut c_void,
}

const TIMEOUT_MILLIS: u64 = 10000;
const POLLING_STEPS_MILLIS: u64 = 10;

impl AMForth {
    pub fn init() -> AMForth {
        let mut state = AMForth{parser: unsafe {amf_init_parser()}};
        state.parse_string(": 🥕 dup 0> if 1 swap 0 do over * loop swap drop else 2drop 1 then ;\n");
        state.parse_string(": :carrot: 🥕 ;\n");
        state
    }

    pub fn parse_string(&mut self, s: &str) {
        println!("Call to state '{:?}' with data '{}'.", self.parser, s);
        #[repr(C)]
        struct StateAndString {
            parser: *mut c_void,
            string: *const c_char,
        }

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
                cancellable_thread();
                amf_parse_string(arg.parser, arg.string);
            }
            *PTHREAD_RUNNING.lock().unwrap() = false;
            std::ptr::null_mut()
        }

        *PTHREAD_RUNNING.lock().unwrap() = true;
        unsafe {
            libc::pthread_create(&mut running_thread, std::ptr::null(), run_thread, arg_ptr);
        }


        for _ in 0..(TIMEOUT_MILLIS/POLLING_STEPS_MILLIS) {
            thread::sleep(Duration::from_millis(POLLING_STEPS_MILLIS.into()));
            if !*PTHREAD_RUNNING.lock().unwrap() {
                break;
            }
        }

        if *PTHREAD_RUNNING.lock().unwrap() {
            println!("timeout");
            unsafe {
                libc::pthread_cancel(running_thread);
                libc::pthread_join(running_thread, std::ptr::null_mut());
                //amf_clean_parser(self.parser); /* This is a memory leak but for some reason, the thread that should have been joined sometimes still write to the freed memory. This might not even be the worst abomination in this file. */
                self.parser = amf_init_parser();
                send_string_to_output("[ERROR] Timeout while executing forth. Resetting state\n");
            }
        } else {
            unsafe {
                libc::pthread_join(running_thread, std::ptr::null_mut());
            }
        }
        
        /*
        // Simpler version with no timeout
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

fn send_string_to_output(s: &str) {
    let c_s = CString::new(s).expect("CString::new failed");
    amf_print_string(c_s.as_ptr());
}

