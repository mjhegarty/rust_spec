use super::sdr_reader::{sync_return_samples_max_gain};
use super::iq_data::{IQdata};
use std::collections::VecDeque;




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
#[allow(dead_code)]
fn pack_data(_raw_data: Vec<u32>) -> Vec<u32> {        
    unimplemented!()
}
//Hard coded in test of crc checker to make sure that's not the problem
pub fn simple_print_test(){
    let data = get_iq_data(1024);
    println!("raw iq_data is {:?}", data);
    let mag = get_mag(data);
    println!("mag iq_data is {:?}", mag);
}
pub fn simple_preamble_test(){
        let data = get_iq_data(1024*10000);
        let mag = get_mag(data);
        let (detections,matches) = detect_preamble(mag);
        println!("Number of preambles detected in sequence is {}, {} matches", detections,matches); 
}
fn dl_format_check(data_bits : &[u8]) -> bool{
    assert!(data_bits.len()==5, "Wrong dl format check length");
    if data_bits[0]==1 && data_bits[1]==0 && data_bits[2]==0 && data_bits[3]==0 && data_bits[4]==1{
        true
    }
    else{
        false
    }
}
fn mod2_div(divisor: &Vec<u8>, buffer:&mut VecDeque<u8>, carrydown_bit:&u8){
    buffer.push_back(*carrydown_bit);
    if buffer.pop_front().unwrap()==1{
        for it in divisor.iter().zip(buffer.iter_mut()){
            let (div,buf) = it;
            *buf = (*buf)^(*div);
        }
    }
}
pub fn crc_check(data_bits : &Vec<u8>) -> bool{
    //NOTE so this is a 24 bit crc, so the gen is really 25 bits, but I only want the 24 least significant
    //because the 1st one is implied in my algorithim
    let gen = vec![1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,0,0,0,0,0,1,0,0,1];   

    let mut buffer: VecDeque<u8> = VecDeque::from(vec![0; 24]);
    assert!(data_bits.len() == (112), "data packet not correct size");
    for x in data_bits {
        mod2_div(&gen,&mut buffer,x);
    }
    println!("buffer is {:?}",buffer);
    if buffer.contains(&1){false}
    else {true}
}
//Think I'm going to have this function change the PPM encoding to 1s and 0s
//Not sure if I want it to make it into bytes or not tbd
pub fn wave_to_data(mag: &[u32]) -> Vec<u8>{
    assert!(mag.len()==224, "Wave wrong amount of data");
    let mut data: Vec<u8> = Vec::with_capacity(mag.len()/2 as usize);
    let mut iter = mag.iter().peekable();
    let mut first;
    let mut second;
    let mut last = 1;
    while iter.peek()!=None{
        first = *iter.next().unwrap();
        second = *iter.next().unwrap();
        //println!("first and second are {:?} and {:?}", first, second);
        if first==second{
            data.push(last);
        }
        else if first> second{
            last = 1;
            data.push(1);
        }
        else {
            last = 0;
            data.push(0);
        }
    }
    data
}
//wave to data but with a teeny bit of phase correction
pub fn wave_to_data_pc(mag: &[u32]) -> Vec<u8>{
    assert!(mag.len()==224, "Wave wrong amount of data");
    let mut data: Vec<u8> = Vec::with_capacity(mag.len()/2 as usize);
    let mut iter = mag.iter().peekable();
    let mut first = 0;
    let mut second;
    let mut last = 2;
    while iter.peek()!=None{
        if last ==0{
            first = (*iter.next().unwrap()*4)/5;
        }
        else if last ==1{
            first = (*iter.next().unwrap()*5)/4;

        }
        else if last ==2{
            first =*iter.next().unwrap();

        }
        second = *iter.next().unwrap();
        //println!("first and second are {:?} and {:?}", first, second);
        if first==second{
            if last == 2 {
                data.push(1);
                last =1;
            }
            else{
                data.push(last);
            }
        }
        else if first> second{
            last = 1;
            data.push(1);
        }
        else {
            last = 0;
            data.push(0);
        }
    }
    data
}
//data processing function will change to take data, not
//just reference it
pub fn data_processing(_data: &[u32]) -> u32{
    unimplemented!()
}
pub fn is_preamble(mag: &[u32]) -> bool
{
    assert!((mag.len())==16, "Wrong length preamble");
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
pub fn detect_preamble(mag: Vec<u32>) -> (i32, i32) {
    let mut count = 0;
    let mut passed_crc = 0;
    let mut i = 0;
    let mut data;
    let mut data_p;
    loop{        
        if i>(mag.len()-240){ break;}
        if is_preamble(&mag[i..(i+16)]){//So rust ranges are inclusive, exclusive
            data = wave_to_data(&mag[i+16..(i+16+224)]);
            //println!("differential data read: {:?}",data);
            /*
            if dl_format_check(&data[0 .. 5]){
                println!("DL format is correct!");
            }
            */
            if crc_check(&data){
                println!("CRC check passed!");
                passed_crc+=1;
                i +=240;
            }
            else{
             //   println!("crc failed...");
                data_p = wave_to_data_pc(&mag[i+16 ..(i+16+224)]);
                if crc_check(&data_p){
                    println!("CRC check passed in phase correction!");
                    passed_crc+=1;
                    i +=240;
                }
                else{
                    i+=1;
                }
            }
            count+=1;
        }
        else{
            i += 1;
        }
    }
    (count,passed_crc) //Return a count of how many preambles are found for now
}








#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn crc_test(){
        assert!(crc_check(&vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,0,0,1,1,1,0,1,1,0,0,1,0,1,0,0,1,1,0,0]), "crc check failed"); 
        assert!(crc_check(&vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,1,1,0,1,0,1,0,0,1,0,0,1,0,1,1,1,1,0,0,1,0,1]), "crc check failed");
        assert!(crc_check(&vec![1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,0,1,1,0,1,1,1,0,1,1,0,1,1,0,1,1,0,1,0,1,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0,1,0,0,1,1,0,1,0]), "crc check failed");
    }
    #[test]
    fn preamble_test_easy(){
        assert!(is_preamble(&vec![200, 0, 200, 50, 40, 30,30, 200, 50, 200, 50, 30, 20, 20, 20, 20]), "preamble check failed");
    }
    #[test]
    fn ppm_encoding_test_ones(){
        let mag = wave_to_data(&vec![1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0]);
        assert!(!mag.contains(&0), "ppm failed")
        
    }
    #[test]
    fn ppm_encoding_test_zeros(){
        let mag = wave_to_data(&vec![0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1]);
        assert!(!mag.contains(&1), "ppm failed")
        
    }
    #[test]
    fn detect_preamble_full_test(){
        let mag = vec![0,0,0,0,0,0,0,0, 200, 0, 200, 50, 40, 30,30, 200, 50, 200, 50, 30, 20, 20, 20, 20, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let (pre,crc) = detect_preamble(mag);
        assert!(pre>=1, "overall preamble detection failed");
        assert!(crc>=1, "crc detection failed");
    }
    #[test]
    fn check_dl_format(){
        let mag = vec![1,0,0,0,1];
        assert!(dl_format_check(&mag), "dl format checker doesn't work");
    }
}

