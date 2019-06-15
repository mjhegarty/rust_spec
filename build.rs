fn main() {
    //println!("cargo:rustc-link-search=static=../../../projects/rtl-sdr/build/src/"); // the "-L" flag
    println!("cargo:rustc-link-lib=rtlsdr"); // the "-l" flag
}
