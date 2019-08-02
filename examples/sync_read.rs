#[macro_use] extern crate enum_primitive;
extern crate rust_spec;
//pub mod rtl_sdr;
//pub mod sdr_reader;
//pub mod IQ_data;
//pub mod ads_b;
use rust_spec::sdr_reader::{sync_read_samples, sync_read_samples_max_gain};
use rust_spec::ads_b::{simple_print_test, simple_preamble_test,simple_crc_test};

fn main() {
    println!("reading samples: storing in unformatted.txt");
    sync_read_samples(1024*2000, 99_500_000, 2_048_000)
}
