pub mod rtl_sdr;
pub mod IQ_reader;
use rtl_sdr::{RTL_SDR, IQdata};
use IQ_reader::{sync_read_samples};

fn main() {
    sync_read_samples(1024*1000, 99_500_000, 1_140_000);
    println!("samples read successfully");
}

