#[macro_use] extern crate enum_primitive;
pub mod rtl_sdr;
pub mod IQ_reader;
use rtl_sdr::{RTL_SDR, IQdata};
use IQ_reader::{sync_read_samples, sync_read_samples_max_gain};

fn main() {
    sync_read_samples_max_gain(1024*1000, 99_500_000, 1_140_000);
    println!("samples read successfully");
}

