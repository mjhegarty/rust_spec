use super::rtl_sdr::{RTL_SDR, IQdata};

pub fn sync_read_samples(num_samples:i32,center_frequency:u32, sampling_rate:u32,) {
    let mut dev = RTL_SDR::new();
    dev.reset_buffer();
    dev.set_tuner_gain_mode(0);
    dev.set_agc(1);
    dev.set_center_freq(center_frequency);
    assert!(dev.get_center_freq()==center_frequency, "failure in setting frequency");
    //dev.set_sample_rate(2048000);
    dev.set_sample_rate(sampling_rate);
    assert!(dev.get_sample_rate()==sampling_rate, "failure in setting sampling rate");
    //Do a dummy read just to be safe
    dev.read_sync(1024*2, 1024*4);
    dev.reset_buffer();
    let (buf, err) = dev.read_sync(1024,num_samples*2);//TODO find optimal block size
    assert!(err==0, "error with reading samples"); //TODO have assert messages be jp's error messages
    dev.close_device();
    let data = IQdata::new(buf,num_samples*2);
    data.write("test.txt".to_string());
}

pub fn sync_read_samples_max_gain(num_samples:i32,center_frequency:u32, sampling_rate:u32,) {
    let mut dev = RTL_SDR::new();
    dev.reset_buffer();
    dev.set_tuner_gain_mode(1);
    dev.set_agc(0);//NOTE not sure if turning this off affects other areas of the tuner TBD
    dev.set_center_freq(center_frequency);
    let gains = max_gain(&dev.get_tuner_gains().unwrap());
    dev.set_tuner_gain(gains).unwrap();

    
    assert!(dev.get_center_freq()==center_frequency, "failure in setting frequency");
    //dev.set_sample_rate(2048000);
    dev.set_sample_rate(sampling_rate);
    assert!(dev.get_sample_rate()==sampling_rate, "failure in setting sampling rate");
    //Do a dummy read just to be safe
    dev.read_sync(1024*2, 1024*4);
    dev.reset_buffer();
    let (buf, err) = dev.read_sync(1024,num_samples*2);//TODO find optimal block size
    assert!(err==0, "error with reading samples"); //TODO have assert messages be jp's error messages
    dev.close_device();
    let data = IQdata::new(buf,num_samples*2);
    data.write("test.txt".to_string());
}
fn max_gain(gains: &Vec<i32>) -> i32{
    let mut max = 0;
    for i in gains {
        if *i>max { max = *i};
    }
    max
}
