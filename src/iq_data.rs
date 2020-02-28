use std::fs::{File};
use std::io::{BufWriter, Write};

//Struct to store IQ data in. Would be interesting to see if
#[derive(Debug)]
pub struct IQdata{
    in_phase: Vec<f32>,
    quad: Vec<f32>,
}
//**Implementation of IQdata
impl IQdata{
    pub fn new(raw_data:Vec<u8>,size:i32 ) -> Self {
        assert!(size%2==0, "uneven number of samples how?");
        let mut i = Vec::with_capacity((size/2)as usize);
        let mut q = Vec::with_capacity((size/2)as usize);
        for (num,val) in raw_data.iter().enumerate(){
             if num%2 == 0 {i.push(((*val as i16 -127)as f32)/127.5);}//NOTE Dc bias makes me wanna do this kinda different 127.5 rather than 128
             else {q.push(((*val as i16 -127)as f32)/127.5);}//TODO this just has to be the dumbest way to do this
        }
        IQdata{
            in_phase:i,
            quad:   q}
    }
    //Returns vec(rust data struct not math thing) of scalar values using sqrt(q^2+i^2)
    pub fn get_mag(&self) -> Vec<u32>{ //TODO experiment with shifting values up to 256
        let mut mag = Vec::with_capacity(self.in_phase.len());
        let mut temp:f32;
        for it in self.in_phase.iter().zip(self.quad.iter()) {
            let (i, q) = it;
            temp = (*i * *i  + *q  *  *q).sqrt();
            mag.push((temp*360.0).round()as u32);

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
                    writeln!(buf, "{}", line).unwrap();
        }
        for line in self.quad {
                    writeln!(buf, "{}", line).unwrap();
        }
        0
    }    
}

