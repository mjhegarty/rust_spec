use std::fs::{File};
use std::io::{BufWriter, Write};

//Struct to store IQ data in. Would be interesting to see if
pub struct IQdata{
    in_phase: Vec<u8>,
    quad: Vec<u8>,
}
//**Implementations of IQdata
impl IQdata{
    pub fn new(raw_data:Vec<u8>,size:i32 ) -> Self {
        assert!(size%2==0, "uneven number of samples how?");
        let mut i = Vec::with_capacity((size/2)as usize);
        let mut q = Vec::with_capacity((size/2)as usize);
        for (num,val) in raw_data.iter().enumerate(){
             if num%2 == 0 {i.push(*val);}
             else {q.push(*val);}
        }
        IQdata{
            in_phase:i,
            quad:   q}
    }
    //Returns vec(rust data struct not math thing) of scalar values using sqrt(q^2+i^2)
    pub fn get_mag(&self) -> Vec<u8>{ //TODO experiment with shifting values up to 256
       unimplemented!()
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
