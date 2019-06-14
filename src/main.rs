extern crate libc;
use libc::{c_int,c_char, c_void};
use std::{str,ptr};
use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufRead,BufReader, Write};
use std::ffi::CStr;
use std::time;
use std::thread::{sleep};
//person who also did this did something like this
enum rtlsdr_dev_t {}
enum RtlWarn{
    NoIssue,
    Pipe,
    Etc,
}

//Struct to store IQ data in. Would be interesting to see if
struct IQdata{
    in_phase: Vec<u8>,
    quad: Vec<u8>,
}
struct RTL_SDR{
   dev: *mut rtlsdr_dev_t, 
}


fn main() {
    println!("Hello, world!");
    unsafe{
        println!("{}",rtlsdr_get_device_count());
        println!("{:?}", c_string_to_r_string(rtlsdr_get_device_name(0)));
    }
    let mut dev = RTL_SDR::new();
    dev.set_center_freq(99500000);
    println!("center freq verified as {}", dev.get_center_freq());
    dev.set_sample_rate(2048000); 
    println!("sample rate verified as {}", dev.get_sample_rate());
    dev.reset_buffer();
    let (buf, nread, err) = dev.read_sync(1024);
    println!("read error returned...{:?}", err);
    dev.close_device();
    let data = IQdata::new(buf,1024);
    data.write("test.txt".to_string());
}

//c code transformation functions
//




fn c_int_to_warning(error: c_int) -> RtlWarn {
    unimplemented!()
}
fn c_string_to_r_string(c: *const c_char) -> String{
    let c_str = unsafe { CStr::from_ptr(c) };
    String::from(str::from_utf8(c_str.to_bytes()).unwrap())
}

#[link(name = "rtlsdr")]
extern "C" {
//    fn rtl_sdr() -> bool;
    //fn rtl_test(argc:c_int,argv:*const *const c_char) -> c_int;
    fn rtlsdr_get_device_count() -> u32;
    fn rtlsdr_get_device_name(index:u32) -> *const c_char;
    fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index:u32) -> c_int;
    fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev_t, freq:u32) -> c_int;
    fn rtlsdr_get_center_freq(dev: *mut rtlsdr_dev_t) ->c_int;
    fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev_t, samp_rate:u32) -> c_int;
    fn rtlsdr_get_sample_rate(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_read_sync(dev: *mut rtlsdr_dev_t,buf:*mut c_void,len: i32,n_read: *mut c_int) -> c_int;
    fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> c_int;
}


//rtlsdrlibrary overhead functions
impl RTL_SDR{
    pub fn new() ->Self{
        //TODO add indexing for multiple devices? 
        unsafe{//not sure if all of this needs unsafe
            assert!(rtlsdr_get_device_count()>0, "No device found");
            let mut dev: *mut rtlsdr_dev_t = std::ptr::null_mut();
            let result = rtlsdr_open(&mut dev as *mut *mut rtlsdr_dev_t, 0);
            println! ("result of open is ... {}", result);
            RTL_SDR{dev: dev as *mut rtlsdr_dev_t}
        }
    }
    pub fn close_device(self) -> () {
        unsafe{
            let result = rtlsdr_close(self.dev);
            //TODO add some sorta free function for self.dev
            println!("result of close is ... {}", result);
            
        }
    }
    pub fn set_center_freq(&mut self, freq: u32) -> () {
        unsafe{
            let result = rtlsdr_set_center_freq(self.dev, freq);
            println! ("result of setting center frequency is ... {}", result);
        }
    }
    fn get_center_freq(&self) -> u32 {
        unsafe{
            let center_freq = rtlsdr_get_center_freq(self.dev) as u32;
            center_freq
        }
    }
    fn set_sample_rate(&mut self, samp_rate: u32) -> () {
        unsafe{
            let result = rtlsdr_set_sample_rate(self.dev, samp_rate);
            println! ("result of setting sample rate is ... {}", result);
        }
    }
    fn get_sample_rate(&self) -> u32 {
        unsafe{
            let sample_rate = rtlsdr_get_sample_rate(self.dev) as u32;
            sample_rate
        }
    }
    //So reading through the documentation of the c functions, I am fairly
    //certain that read_sync returns alternating IQ data values starting with I
    //so therefore a higher level function that calls read sync to read values
    //should double the number of points it puts on
    //
    fn read_sync(&mut self, len: i32) -> (Vec<u8>, i32, i32) {
        let mut buf = vec![0u8; len as usize];
        let mut n_read: i32 = 0;
        //TODO: how the c code does read sync is they have a block size of min 512 blocks and
        //they have a seperate bytes to read function that gets that amount subtracted from it
        //every time they read in bytes. See RTL_SDR.c line 236 for details
        unsafe{
            let err = rtlsdr_read_sync(self.dev,buf.as_mut_ptr() as *mut c_void, len,&mut n_read as *mut c_int); 
            (buf, n_read, err)
        }
    }
    //Documentation for C code says to run this function before any reads
    fn reset_buffer(&mut self) -> (){
        unsafe{
            let result = rtlsdr_reset_buffer(self.dev);
            println!("The result of resetting buffer is ... {}", result);
        }

    }

}
fn open_device(index: u32) -> *mut rtlsdr_dev_t {
    unsafe{
        let mut dev: *mut rtlsdr_dev_t = std::ptr::null_mut();
        let result = rtlsdr_open(&mut dev as *mut *mut rtlsdr_dev_t, index);
        println! ("result of open is ... {}", result);
        dev as *mut rtlsdr_dev_t
    }
}

//**Implementations of IQdata
impl IQdata{
    

    /*Line 282 in rtl_fm.c is how I think they make thier IQ data
     * it's really jank becuase it's in C so they do some B.S. magic
     * instead of just spelling out what they do but I think it has something to do with how every
     * value is 90 degrees away from each other
     *
     */
    fn new(raw_data:Vec<u8>,size:i32 ) -> Self {
        assert!(size%2==0, "uneven number of samples how?");
        //let mut I = vec![0u8; (size/2) as usize];
        //let mut Q = vec![0u8; (size/2) as usize];
        let mut I = vec![0u8];
        let mut Q = vec![0u8];
        for (num,val) in raw_data.iter().enumerate(){
             if num%2 == 0 {I.push(*val);}
             else {Q.push(*val);}
        }
        IQdata{
            in_phase:I,
            quad:   Q} 
    }
    fn write(self,filename: String) -> i32{
        let mut f = OpenOptions::new().read(true)
                                        .write(true)
                                        .create(true)
                                        .append(false)
                                        .open(filename)
                                        .unwrap();
        


        writeln!(f, "{:?}", self.in_phase);
        writeln!(f, "{:?}", self.quad);
        0

    }
}








