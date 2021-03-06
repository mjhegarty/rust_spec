extern crate num;
use num::FromPrimitive;
use libc::{c_int,c_char, c_void};
use std::{str};
use std::ffi::CStr;
//person who also did this did something like this
enum RtlSdrDevT{}
//Errors as listed in jpoirier's implementation
enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum Error {
    NoError=0,
    Io=-1,
    InvalidParam=-2,
    Access=-3,
    NoDevice=-4,
    NotFound=-5,
    Busy=-6,
    Timeout=-7,
    Overflow=-8,
    Pipe=-9,
    Interrupted=-10,
    NoMem=-11,
    NotSupported=-12,
    NoValidEEPROMHeader=-13,
    StringValueTooLong=-14,
    StringDescriptorInvalid=-15,
    StringDescriptorTooLong=-16,
    Unknown=-17,
}
}
pub struct RtlSdr{
   dev: *mut RtlSdrDevT,
}

//c code transformation functions
//




#[allow(dead_code)]
fn c_string_to_r_string(c: *const c_char) -> String{
    let c_str = unsafe { CStr::from_ptr(c) };
    String::from(str::from_utf8(c_str.to_bytes()).unwrap())
}

#[link(name = "rtlsdr")]
#[allow(dead_code)]
extern "C" {
    fn rtlsdr_get_device_count() -> u32;
    fn rtlsdr_get_device_name(index:u32) -> *const c_char;
    fn rtlsdr_open(dev: *mut *mut RtlSdrDevT, index:u32) -> c_int;
    fn rtlsdr_close(dev: *mut RtlSdrDevT) -> c_int;
    fn rtlsdr_set_center_freq(dev: *mut RtlSdrDevT, freq:u32) -> c_int;
    fn rtlsdr_get_center_freq(dev: *mut RtlSdrDevT) ->c_int;
    fn rtlsdr_set_sample_rate(dev: *mut RtlSdrDevT, samp_rate:u32) -> c_int;
    fn rtlsdr_get_sample_rate(dev: *mut RtlSdrDevT) -> c_int;
    fn rtlsdr_set_tuner_bandwidth(dev: *mut RtlSdrDevT, bw:u32) -> c_int;
    fn rtlsdr_read_sync(dev: *mut RtlSdrDevT,buf:*mut c_void,len: i32,n_read: *mut c_int) -> c_int;
    fn rtlsdr_reset_buffer(dev: *mut RtlSdrDevT) -> c_int;
    fn rtlsdr_set_agc_mode(dev: *mut RtlSdrDevT, on:i32) -> c_int;
    fn rtlsdr_set_tuner_gain_mode(dev: *mut RtlSdrDevT, mode:i32) -> c_int;
    fn rtlsdr_set_tuner_gain(dev: *mut RtlSdrDevT, gain: i32) -> c_int;
    fn rtlsdr_get_tuner_gains(dev: *mut RtlSdrDevT,gains: *mut c_int)->c_int;
}

pub fn c_to_err(err: i32) -> Error {
    Error::from_i32(err).unwrap()
}
//rtlsdrlibrary overhead functions

impl RtlSdr{
    //TODO have all functions return custom error type defined above
    pub fn new() ->Self{
        //TODO add indexing for multiple devices?
        unsafe{//not sure if all of this needs unsafe
            assert!(rtlsdr_get_device_count()>0, "No device found");
            let mut dev: *mut RtlSdrDevT = std::ptr::null_mut();
            let result = rtlsdr_open(&mut dev as *mut *mut RtlSdrDevT, 0);
            println! ("result of open is ... {}", result);
            RtlSdr{dev: dev as *mut RtlSdrDevT}
        }
    }
    pub fn close_device(self) -> Result<(), Error> {
        unsafe{
            let result = rtlsdr_close(self.dev);
            //TODO add some sorta free function for self.dev
            println!("result of close is ... {}", result);
            if result >=0 { Ok(())}
            else {Err(c_to_err(result))}

        }
    }
    pub fn set_center_freq(&mut self, freq: u32) -> Result<(),Error> {
        unsafe{
            let result = rtlsdr_set_center_freq(self.dev, freq);
            println! ("result of setting center frequency is ... {}", result);
            if result >=0 { Ok(())}
            else {Err(c_to_err(result))}
        }
    }
    pub fn get_center_freq(&self) -> u32 {
        unsafe{
            let center_freq = rtlsdr_get_center_freq(self.dev) as u32;
            center_freq
        }
    }
    pub fn set_sample_rate(&mut self, samp_rate: u32) -> Result<(), Error> {
        unsafe{
            let result = rtlsdr_set_sample_rate(self.dev, samp_rate);
            println! ("result of setting sample rate is ... {}", result);
            if result >=0 { Ok(())}
            else {Err(c_to_err(result))}
        }
    }
    pub fn get_sample_rate(&self) -> u32 {
        unsafe{
            let sample_rate = rtlsdr_get_sample_rate(self.dev) as u32;
            sample_rate
        }
    }
    pub fn set_bandwidth(&mut self, bw: u32) -> Result<(), Error> {
        unsafe{
            let result = rtlsdr_set_tuner_bandwidth(self.dev, bw);
            println!("result of setting bw is ... {}", result);
            if result >=0 { Ok(())}
            else {Err(c_to_err(result))}
        }
    } 
    //AGC===Automatic gain control. Basically the rtl has a couple of stages where it can have a
    //variable gain, and what setting agc does is that their gain is automatically calculated by a
    //power measurer in the following stage to maximize SNR(signal to noise ratio). I'm not sure if
    //this enables AGC for the entire system or just on part, or if it is enabled by default
    pub fn set_agc(&mut self, on: i32) -> Result<(), Error> {
        unsafe{
            let result = rtlsdr_set_agc_mode(self.dev, on);
            println!("result of setting AGC mode is ... {}", result);
            if result >=0 { Ok(())}
            else {Err(c_to_err(result))}
        }
    }
    pub fn get_tuner_gains(&self) ->Result<Vec<i32>, Error> {
        let mut gains = vec![0i32;30 as usize];//Highest tuner gains amount is like 28
 
        unsafe {
            let result = rtlsdr_get_tuner_gains(self.dev, gains.as_mut_ptr() as *mut c_int);
            if result >=0 { Ok(gains)}
            else {Err(c_to_err(result))}
        }

    }
    //Sets tuner gain to one of the values from get_tuner_gains
    pub fn set_tuner_gain(&mut self, gain: i32) ->Result <(), Error> {
        unsafe{
            let result = rtlsdr_set_tuner_gain(self.dev,gain); 
            if result >=0 {Ok(())}
            else {Err(c_to_err(result))}
        }
    }

    //Think 0 tuner gain is using AGC. I'm gonna set it to zero whenever
    //I use AGC just to be safe.
    pub fn set_tuner_gain_mode(&mut self, mode:i32) -> Result<(), Error> {
        unsafe{
            let result = rtlsdr_set_tuner_gain_mode(self.dev, mode);
            println!("result of setting tuner gain mode is ... {}", result);
            if result >=0 { Ok(())}
            else {Err(c_to_err(result))}
        }
    }
    //So reading through the documentation of the c functions, I am fairly
    //certain that read_sync returns alternating IQ data values starting with I
    //so therefore a higher level function that calls read sync to read values
    //should double the number of points it puts on
    //
    pub fn read_sync(&mut self,block_size:i32,bytes_to_read: i32) -> (Vec<u8>, Error) {
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
        while bytes_left>=n_read {
            unsafe{
                err = rtlsdr_read_sync(self.dev,buf.as_mut_ptr() as *mut c_void, block_size,&mut n_read as *mut c_int);
            }
            if err != 0{
                println!("read error, something went wrong");
                return (buf, c_to_err(err));
            }
            else if n_read != block_size {
                println!("read error, samples were lost!");
            }
            else {
               read_data.append(&mut buf.clone()); 
               bytes_left -= n_read;

            }
        } 
        (read_data, c_to_err(err)) 
    }
    //Documentation for C code says to run this function before any reads
    pub fn reset_buffer(&mut self) -> Result<(), Error>{
        unsafe{
            let result = rtlsdr_reset_buffer(self.dev);
            println!("The result of resetting buffer is ... {}", result);
            if result >=0 { Ok(())}
            else {Err(c_to_err(result))}
        }

    }
}









