pub mod rtl_sdr;
use rtl_sdr::{RTL_SDR, IQdata};

fn main() {
    println!("Hello, world!");
    let mut dev = RTL_SDR::new();
    dev.set_agc(1);
    dev.set_center_freq(99500000);
    println!("center freq verified as {}", dev.get_center_freq());
    //dev.set_sample_rate(2048000);
    dev.set_sample_rate(1140000);
    println!("sample rate verified as {}", dev.get_sample_rate());
    //dev.set_bandwidth(200000);
    dev.reset_buffer();
    let (buf, err) = dev.read_sync(1024 ,1024*8000);
    println!("read error returned...{:?}", err);
    dev.close_device();
    let data = IQdata::new(buf,1024*8000*2);
    data.write("test.txt".to_string());
}

