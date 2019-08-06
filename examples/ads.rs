extern crate rust_spec;
use rust_spec::sdr_reader::{sync_read_samples, sync_read_samples_max_gain};
use rust_spec::ads_b::{simple_preamble_test};

fn main() {
    println!("running simple test of ads_b");
//    sync_read_samples(1024*2000, 99_500_000, 2_048_000)
    simple_preamble_test(); 
 //   println!("simple print test");
//    simple_print_test();
}

