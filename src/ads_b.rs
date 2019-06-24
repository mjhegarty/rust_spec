use super::sdr_reader::{sync_return_samples_max_gain};
use super::IQ_data::{IQdata};
use itertools::Itertools;

fn get_iq_data(n_samples:i32) ->IQdata {
    //For ADS-B we use the 1050 MHz frequency with a sampling rate of 2MHz
    let iq_data = sync_return_samples_max_gain(n_samples, 1_050_000_000, 2_000_000);
    println!("samples read successfully");
    iq_data
}

fn get_mag_data(raw_data:IQdata) -> Vec<u32> {
    raw_data.get_mag()
}
//transform differntial encoded data into bytes or bits
//groups of 8(bytes) would be better for a lot of stuff but
//its tricky because the data section is going to be in groups
//of 6, so it might be easier to just leave it as bits.
//dump1090 uses bytes so it probably is the better option
fn pack_data(raw_data: Vec<u32>) -> Vec<u32> {
    unimplemented!()
}
pub fn simple_print_test(){
    let data = get_iq_data(1024);
    println!("raw iq_data is {:?}", data);
    let mag = get_mag_data(data);
    println!("mag iq_data is {:?}", mag);
}
pub fn simple_preamble_test(){
    let data = get_iq_data(1024*100);
    let mag = get_mag_data(data);
    println!("Number of preambles detected in sequence is {}", detect_preamble(mag)); 
}
//data processing function will change to take data, not
//just reference it
pub fn data_processing(mag_data: &[u32]) -> u32{
    unimplemented!()
}
pub fn is_preamble(mag_data: &[u32]) -> bool
{
    //this will do for now want it to look cooler though
    //check that impluses are above half
    if mag_data[0] < 128 || mag_data[2] <128 || mag_data[7] <128 || mag_data[9]<128{
        return false;
    }
    if mag_data[1] > 128 || mag_data[3] >128 || mag_data[4] > 128 || mag_data[5] > 128 || mag_data[6] > 128 || mag_data[8] > 128 {
        return false;
    }
    if mag_data[10] > 90 || mag_data[11] > 90 || mag_data[12] > 90 || mag_data[13] > 90 || mag_data[14] > 90 || mag_data[15] > 90 {
        return false;
    }
    else {
        return true;
    }
}
pub fn detect_preamble(mag_data: Vec<u32>) -> i32 {
    let mut count = 0;
    let mut i = 0;
    loop{        
        if is_preamble(&mag_data[i..(i+15)]){
           // data_processing(&mag_data[i..(i+119)]);
            count+=1;
            i += 120;
        }
        else{
            i += 1;
        }
        if i>=mag_data.len(){ break;}
    }
    count //Return a count of how many preambles are found for now
}

//NOTE: Iterators seem like a really really good idea for checksum stuff.
