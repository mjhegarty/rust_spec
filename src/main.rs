#[macro_use] extern crate enum_primitive;
pub mod rtl_sdr;
pub mod sdr_reader;
pub mod IQ_data;
pub mod ads_b;
use sdr_reader::{sync_read_samples, sync_read_samples_max_gain};
use ads_b::{simple_print_test, simple_preamble_test,simple_crc_test};

fn main() {
    println!("running simple test of ads_b");
//    sync_read_samples(1024*2000, 99_500_000, 2_048_000)
    println!("result of simple crc test is {}",simple_crc_test());
   // simple_preamble_test(); 
//    simple_print_test();
}

