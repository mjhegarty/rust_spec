use super::sdr_reader::{sync_return_samples_max_gain};
use super::IQ_data::{IQdata};
use itertools::Itertools;

fn get_iq_data(n_samples:i32) ->IQdata {
    //For ADS-B we use the 1050 MHz frequency with a sampling rate of 2MHz
    let iq_data = sync_return_samples_max_gain(n_samples, 1_050_000_000, 2_000_000);
    println!("samples read successfully");
    iq_data
}

fn get_mag(raw_data:IQdata) -> Vec<u32> {
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
    let mag = get_mag(data);
    println!("mag iq_data is {:?}", mag);
}
pub fn simple_preamble_test(){
    let data = get_iq_data(1024*10000);
    let mag = get_mag(data);
    println!("Number of preambles detected in sequence is {}", detect_preamble(mag)); 
}
//Think I'm going to have this function change the differerntial encoding to 1s and 0s
//Not sure if I want it to make it into bytes or not tbd
pub fn wave_to_data(mag: &[u32]) -> Vec<u8>{
    let mut data: Vec<u8> = Vec::with_capacity(mag.len()/2 as usize);
    let mut iter = mag.iter().peekable();
    while iter.peek()!=None{
        if iter.next() >= iter.next(){
            data.push(1);
        }
        else {
            data.push(0);
        }
    }
    data
}
//data processing function will change to take data, not
//just reference it
pub fn data_processing(data: &[u32]) -> u32{
    unimplemented!()
}
//This function is going to check for the crc
//somehow
pub fn check_crc(data: &[u32]) -> bool{

    unimplemented!()

}
pub fn is_preamble(mag: &[u32]) -> bool
{
    //this will do for now want it to look cooler though
    //check that impluses are above half
    //So it turns out I can't acatually check if they are above half b/c thats some arbitrary
    //number. I really need to check the relationship between bits
    //mag[0] mag[2] mag[7] mag[9]
    ////For now I just copied what other people did, but i want to think of a more effiecient way
    // test the relationship between these bits
    if mag[0] < mag[1] ||mag[0] < mag[3] || mag[0]<mag[4] || mag[0] < mag[5]
    {
        return false;
    }
    if mag[2] < mag[1] || mag[2] < mag[3] || mag[2] < mag[5] {
        return false;    
    }
    if mag[7] < mag[6] || mag[7] < mag[5] || mag[7] < mag[8] {
        return false;    
    }
    if mag[9] < mag[6] || mag[9] < mag[10] || mag[9] < mag[8] {
        return false;    
    }
    let high = (mag[0] + mag[2] + mag[7] + mag[9])/6;
    if mag[4] >= high || mag[5] >= high {
        return false;
    }
    if mag[11] >= high || mag[12] >= high || mag[13] >=high || mag[14] >= high {
        return false;
    }
    else {
        return true;
    }
}
pub fn detect_preamble(mag: Vec<u32>) -> i32 {
    let mut count = 0;
    let mut i = 0;
    loop{        
        if i>=(mag.len()-240){ break;}
        if is_preamble(&mag[i..(i+15)]){
            println!("differential data read: {:?}",wave_to_data(&mag[i+16.. (i+16+224)]));
            count+=1;
            i += 240;
        }
        else{
            i += 1;
        }
    }
    count //Return a count of how many preambles are found for now
}

//NOTE: Iterators seem like a really really good idea for checksum stuff.
