extern crate rust_spec;
use rust_spec::sdr_reader::{sync_read_samples, sync_read_samples_max_gain};

fn main() {
    println!("reading samples: storing in unformatted.txt");
    sync_read_samples(1024*4000, 99_500_000, 2_048_000)
}

