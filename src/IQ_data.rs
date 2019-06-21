use std::fs::{File};
use std::io::{BufWriter, Write};
use std::cmp::{max, min};

//Struct to store IQ data in. Would be interesting to see if
#[derive(Debug)]
pub struct IQdata{
    in_phase: Vec<i8>,
    quad: Vec<i8>,
}
//**Implementation of IQdata
impl IQdata{
    pub fn new(raw_data:Vec<u8>,size:i32 ) -> Self {
        assert!(size%2==0, "uneven number of samples how?");
        let mut i = Vec::with_capacity((size/2)as usize);
        let mut q = Vec::with_capacity((size/2)as usize);
        for (num,val) in raw_data.iter().enumerate(){
             if num%2 == 0 {i.push((*val as i16 -128)as i8);}//NOTE Dc bias makes me wanna do this kinda different 127.5 rather than 128
             else {q.push((*val as i16 -128)as i8);}//TODO this just has to be the dumbest way to do this
        }
        IQdata{
            in_phase:i,
            quad:   q}
    }
    //Returns vec(rust data struct not math thing) of scalar values using sqrt(q^2+i^2)
    pub fn get_mag(&self) -> Vec<u32>{ //TODO experiment with shifting values up to 256
        let mut mag = Vec::with_capacity(self.in_phase.len());
        for it in self.in_phase.iter().zip(self.quad.iter()) {
            let (i, q) = it;
            mag.push((((*i as i32).pow(2)+(*q as i32).pow(2))as f64).sqrt() as u32);

        }
        mag
    }
    pub fn get_mag_quick(&self) -> Vec<u32>{
        let mut mag =Vec::with_capacity(self.in_phase.len()) as Vec<u32>;
        let alpha = 4;
        let beta = 1;
        for it in self.in_phase.iter().zip(self.quad.iter()) {
            let (i, q) = it;
            let (max, min) = larger_smaller_abs(i,q);
            let up_value = (max)*alpha + (min)*beta;
            //TODO get abs value (custom function?)
            mag.push((up_value/4) as u32);
        }
        mag

    }
    pub fn write(self,filename: String) -> i32{
        let out: Result<File, std::io::Error> = File::create(filename);
        let file = match out {
            Ok(file) => file,
            Err(..) => panic!("Couldn't output to file"),
        };
        let mut buf = BufWriter::new(file);
        for line in self.in_phase {
                    writeln!(buf, "{}", line);
        }
        for line in self.quad {
                    writeln!(buf, "{}", line);
        }
        0
    }    
}
fn larger_smaller_abs(a:&i8,b: &i8) -> (u16, u16){
    let a1 = (*a).abs() as u16;
    let b1 = (*b).abs() as u16;
    (max(a1,b1), min(a1,b1))
}
