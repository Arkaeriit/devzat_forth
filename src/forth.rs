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
    fn sef_init(state: *mut c_void);
    fn sef_eval_string(state: *mut c_void, s: *const c_char);
    fn malloc(size: usize) -> *mut c_void;
    fn cancellable_thread();
    fn sef_ready_to_run(state: *mut c_void) -> bool;
    fn sef_restart(state: *mut c_void);
    fn sef_register_block_file(state: *mut c_void, filename: *const c_char, number_fo_blocks: isize);
}

const STATE_SIZE: usize = (1 + ((40000000 / 8) + (200 / 8) + 1000 + 1000 + 100 + 17)) * 8;

#[derive(Clone, Copy)]
pub struct SEForth<'a> {
    state: *mut c_void,
    block_file: &'a str,
    number_fo_blocks: isize,
}

const TIMEOUT_MILLIS: u64 = 10000;
const POLLING_STEPS_MILLIS: u64 = 10;

impl<'a> SEForth<'a> {
    pub fn init(block_file: &'a str, number_fo_blocks: isize) -> SEForth<'a> {
        let mut state = SEForth{state: unsafe {malloc(STATE_SIZE)}, block_file, number_fo_blocks};
        unsafe {
            sef_init(state.state);
        }
        state.run_default_code();
        state
    }

    fn run_default_code(&mut self) {
        self.parse_string(": 🥕 dup 0> if 1 swap 0 do over * loop swap drop else 2drop 1 then ;\n");
        self.parse_string(": :carrot: 🥕 ;\n");
        if self.number_fo_blocks > 0 {
            unsafe {
                let c_s = CString::new(self.block_file).expect("CString::new failed");
                sef_register_block_file(self.state, c_s.as_ptr(), self.number_fo_blocks);
            }
        }
    }

    pub fn parse_string(&mut self, s: &str) {
        println!("Call to state '{:?}' with data '{}'.", self.state, s);

        unsafe {
            if !sef_ready_to_run(self.state) {
                println!("Need to restart state.");
                sef_restart(self.state);
            }
        }

        #[repr(C)]
        struct StateAndString {
            state: *mut c_void,
            string: *const c_char,
        }

        let mut running_thread: libc::pthread_t = 0;
        let c_s = CString::new(s).expect("CString::new failed");
        let mut thread_arguments = StateAndString{
            state: self.state,
            string: c_s.as_ptr()
        };
        let arg_ptr: *mut c_void = &mut thread_arguments as *mut _ as *mut c_void;

        extern "C" fn run_thread(arg: *mut c_void) -> *mut c_void {
            let arg: &mut StateAndString = unsafe { &mut *(arg as *mut StateAndString) }; 
            unsafe {
                cancellable_thread();
                sef_eval_string(arg.state, arg.string);
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
                sef_init(self.state);
                self.run_default_code();
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
            sef_parse_string(self.parser, c_s.as_ptr());
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
pub extern "C" fn sef_input() -> c_char {
    return 0;
}

#[no_mangle]
pub extern "C" fn sef_output(c: u8) {
    let c_as_slice = &[c];
    let _ = stdout().write_all(c_as_slice);
    OUTPUT_STREAM.lock().unwrap().push(c);
}

    //for b in unsafe { cstr.to_bytes() as &[i8] } {
#[no_mangle]
pub extern "C" fn sef_print_string(s: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(s) };
    for b in cstr.to_bytes() {
        sef_output(*b);
    }
    // Get copy-on-write Cow<'_, str>, then guarantee a freshly-owned String allocation
    //println!("{}", String::from_utf8_lossy(cstr.to_bytes()).to_string());
}

fn send_string_to_output(s: &str) {
    let c_s = CString::new(s).expect("CString::new failed");
    sef_print_string(c_s.as_ptr());
}

