//Different use cases for rtl_sdr and IQdata
use super::rtl_sdr::{RtlSdr, Error};
use super::iq_data::IQdata;
pub fn sync_read_samples(num_samples:i32,center_frequency:u32, sampling_rate:u32,) {
    let mut dev = RtlSdr::new();
    dev.reset_buffer().unwrap();
    dev.set_tuner_gain_mode(0).unwrap();
    dev.set_agc(1).unwrap();
    dev.set_center_freq(center_frequency).unwrap();
    dev.set_bandwidth(500_000).unwrap();
    assert!(dev.get_center_freq()==center_frequency, "failure in setting frequency");
    //dev.set_sample_rate(2048000);
    dev.set_sample_rate(sampling_rate).unwrap();
    assert!(dev.get_sample_rate()==sampling_rate, "failure in setting sampling rate");
    //Do a dummy read just to be safe
    dev.read_sync(1024*2, 1024*4);
    dev.reset_buffer().unwrap();
    let (buf, err) = dev.read_sync(1024*4,num_samples*2);//TODO find optimal block size
    assert!(err==Error::NoError, "error with reading samples"); //TODO have assert messages be jp's error messages
    dev.close_device().unwrap();
    let data = IQdata::new(buf,num_samples*2);
    data.write("unformatted.txt".to_string());
} 
pub fn sync_read_samples_max_gain(num_samples:i32,center_frequency:u32, sampling_rate:u32,) {
    let mut dev = RtlSdr::new();
    dev.reset_buffer().unwrap();
    dev.set_tuner_gain_mode(1).unwrap();
    dev.set_agc(0).unwrap();//NOTE not sure if turning this off affects other areas of the tuner TBD
    dev.set_center_freq(center_frequency).unwrap();
    let gains = max_gain(&dev.get_tuner_gains().unwrap());
    dev.set_tuner_gain(gains).unwrap();
    dev.set_bandwidth(300_000).unwrap();

    
    assert!(dev.get_center_freq()==center_frequency, "failure in setting frequency");
    dev.set_sample_rate(sampling_rate).unwrap();
    assert!(dev.get_sample_rate()==sampling_rate, "failure in setting sampling rate");
    //Do a dummy read just to be safe
    dev.read_sync(1024*2, 1024*4);
    dev.reset_buffer().unwrap();
    let (buf, err) = dev.read_sync(1024,num_samples*2);//TODO find optimal block size
    assert!(err==Error::NoError, "error with reading samples"); //TODO have assert messages be jp's error messages
    dev.close_device().unwrap();
    let data = IQdata::new(buf,num_samples*2);
    data.write("test.txt".to_string());
}
pub fn sync_return_samples_max_gain(num_samples:i32,center_frequency:u32, sampling_rate:u32,)-> IQdata {
    println!("Sync return samples max gain");
    let mut dev = RtlSdr::new();
    dev.reset_buffer().unwrap();
    dev.set_tuner_gain_mode(1).unwrap();
    dev.set_agc(0).unwrap();//NOTE not sure if turning this off affects other areas of the tuner TBD
    dev.set_center_freq(center_frequency).unwrap();
    dev.set_bandwidth(200_000).unwrap();
    let gains = max_gain(&dev.get_tuner_gains().unwrap());
    dev.set_tuner_gain(gains).unwrap(); 
    assert!(dev.get_center_freq()==center_frequency, "failure in setting frequency");
    dev.set_sample_rate(sampling_rate).unwrap();
    assert!(dev.get_sample_rate()==sampling_rate, "failure in setting sampling rate");
    //Do a dummy read just to be safe
    //dev.read_sync(1024*2, 1024*4);
    dev.reset_buffer().unwrap();
    let (buf, err) = dev.read_sync(1024,num_samples*2);//TODO find optimal block size
    assert!(err==Error::NoError, "error with reading samples"); //TODO have assert messages be jp's error messages
    dev.close_device().unwrap();
    let data = IQdata::new(buf,num_samples*2);
    data
}
pub fn sync_return_samples(num_samples:i32,center_frequency:u32, sampling_rate:u32,)->IQdata {
    println!("Sync return samples");
    let mut dev = RtlSdr::new();
    dev.reset_buffer().unwrap();
    dev.set_tuner_gain_mode(0).unwrap();
    dev.set_agc(1).unwrap();
    dev.set_bandwidth(200_000).unwrap();
    dev.set_center_freq(center_frequency).unwrap();
    assert!(dev.get_center_freq()==center_frequency, "failure in setting frequency");
    dev.set_sample_rate(sampling_rate).unwrap();
    assert!(dev.get_sample_rate()==sampling_rate, "failure in setting sampling rate");
    //Do a dummy read just to be safe
    dev.read_sync(1024*2, 1024*4);
    dev.reset_buffer().unwrap();
    let (buf, err) = dev.read_sync(1024*4,num_samples*2);//TODO find optimal block size
    assert!(err==Error::NoError, "error with reading samples"); //TODO have assert messages be jp's error messages
    dev.close_device().unwrap();
    let data = IQdata::new(buf,num_samples*2);
    data
} 


fn max_gain(gains: &Vec<i32>) -> i32{
    let mut max = 0;
    for i in gains {
        if *i>max {max = *i};
    }
    println!("Max gain is {}",max);
    max
}
