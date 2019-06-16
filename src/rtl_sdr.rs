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
    NoError,
    Io,

    Pipe,
    Etc,
}
//Errors as listed in jpoirier's implementation
pub enum Error {
    NoError,
    Io,
    InvalidParam,
    Access,
    NoDevice,
    NotFound,
    Busy,
    Timeout,
    Overflow,
    Pipe,
    Interrupted,
    NoMem,
    NotSupported,
    NoValidEEPROMHeader,
    StringValueTooLong,
    StringDescriptorInvalid,
    StringDescriptorTooLong,
    Unknown,
}

//Struct to store IQ data in. Would be interesting to see if
pub struct IQdata{
    in_phase: Vec<u8>,
    quad: Vec<u8>,
}
pub struct RTL_SDR{
   dev: *mut rtlsdr_dev_t,
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
    fn rtlsdr_set_tuner_bandwidth(dev: *mut rtlsdr_dev_t, bw:u32) -> c_int;
    fn rtlsdr_read_sync(dev: *mut rtlsdr_dev_t,buf:*mut c_void,len: i32,n_read: *mut c_int) -> c_int;
    fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> c_int;
    fn rtlsdr_set_agc_mode(dev: *mut rtlsdr_dev_t, on:i32) -> c_int;
}


//rtlsdrlibrary overhead functions

impl RTL_SDR{
    //TODO have all functions return custom error type defined above
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
    pub fn get_center_freq(&self) -> u32 {
        unsafe{
            let center_freq = rtlsdr_get_center_freq(self.dev) as u32;
            center_freq
        }
    }
    pub fn set_sample_rate(&mut self, samp_rate: u32) -> () {
        unsafe{
            let result = rtlsdr_set_sample_rate(self.dev, samp_rate);
            println! ("result of setting sample rate is ... {}", result);
        }
    }
    pub fn get_sample_rate(&self) -> u32 {
        unsafe{
            let sample_rate = rtlsdr_get_sample_rate(self.dev) as u32;
            sample_rate
        }
    }
    pub fn set_bandwidth(&mut self, bw: u32) -> () {
        unsafe{
            let result = rtlsdr_set_tuner_bandwidth(self.dev, bw);
            println!("result of setting bw is ... {}", result);
        }
    } 
    //AGC===Automatic gain control. Basically the rtl has a couple of stages where it can have a
    //variable gain, and what setting agc does is that their gain is automatically calculated by a
    //power measurer in the following stage to maximize SNR(signal to noise ratio). I'm not sure if
    //this enables AGC for the entire system or just on part, or if it is enabled by default
    pub fn set_agc(&mut self, on: i32) ->() {
        unsafe{
            let result = rtlsdr_set_agc_mode(self.dev, on);
            println!("result of setting AGC mode is ... {}", result);
        }
    }
    //So reading through the documentation of the c functions, I am fairly
    //certain that read_sync returns alternating IQ data values starting with I
    //so therefore a higher level function that calls read sync to read values
    //should double the number of points it puts on
    //
    //TODO bascially have every function specifically this one has errors as the return instead of
    //how im doing it now.
    pub fn read_sync(&mut self,block_size:i32,bytes_to_read: i32) -> (Vec<u8>, i32) {
        let mut buf = vec![0u8;block_size as usize];
        let mut bytes_left = bytes_to_read;
        let mut read_data = Vec::with_capacity(bytes_to_read as usize);
        //TODO: setup sytem for short reads/writes. For now will assume bytes to read is a factor
        //of block size
        let mut n_read: i32 = 0;
        //TODO: how the c code does read sync is they have a block size of min 512 blocks and
        //they have a seperate bytes to read function that gets that amount subtracted from it
        //every time they read in bytes. See RTL_SDR.c line 236 for details
        let mut err = -1;
            while bytes_left>n_read {
                unsafe{
                    err = rtlsdr_read_sync(self.dev,buf.as_mut_ptr() as *mut c_void, block_size,&mut n_read as *mut c_int);
                }
                if err != 0{
                    println!("error");
                    return (buf, err);
                }
                else if n_read!=block_size {
                    println!("read error, samples lost!");
                }
                else {
                   read_data.append(&mut buf.clone()); 
                   bytes_left -= n_read;

                }
            } 
            (read_data, err) 
    }
    //Documentation for C code says to run this function before any reads
    pub fn reset_buffer(&mut self) -> (){
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
    pub fn new(raw_data:Vec<u8>,size:i32 ) -> Self {
        assert!(size%2==0, "uneven number of samples how?");
        //let mut I = vec![0u8; (size/2) as usize];
        //let mut Q = vec![0u8; (size/2) as usize];
        let mut I = Vec::with_capacity((size/2)as usize);
        let mut Q = Vec::with_capacity((size/2)as usize);
        for (num,val) in raw_data.iter().enumerate(){
             if num%2 == 0 {I.push(*val);}
             else {Q.push(*val);}
        }
        IQdata{
            in_phase:I,
            quad:   Q}
    }
    pub fn write(self,filename: String) -> i32{
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








