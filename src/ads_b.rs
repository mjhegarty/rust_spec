use super::sdr_reader::{sync_return_samples};
use super::IQ_data::{IQdata};

fn get_iq_data(n_samples:i32) ->IQdata {
    //For ADS-B we use the 1050 MHz frequency with a sampling rate of 2MHz
    let iq_data = sync_return_samples(n_samples, 1_050_000_000, 2_000_000);
    println!("samples read successfully");
    iq_data
}

fn get_mag_data(raw_data:IQdata) -> Vec<u32> {
    raw_data.get_mag()
}
pub fn simple_print_test(){
    let data = get_iq_data(1024);
    println!("raw iq_data is {:?}", data);
    let mag = get_mag_data(data);
    println!("mag iq_data is {:?}", mag);
}

