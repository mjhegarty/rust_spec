#[macro_use] extern crate enum_primitive;
pub mod rtl_sdr;
pub mod sdr_reader;
pub mod IQ_data;
use sdr_reader::{sync_read_samples, sync_read_samples_max_gain};

fn main() {
    sync_read_samples(1024, 99_500_000, 1_140_000);
    println!("samples read successfully");
}

