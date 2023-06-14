fn main() {
    #[cfg(not(feature = "_can_driver"))]
    panic!("No CAN driver specified!");

    #[cfg(not(feature = "_time_driver"))]
    panic!("No Time driver specified!");
    
    #[cfg(feature = "peak_can_driver")]
    println!("cargo:rustc-link-lib=PCANBasic");

    #[cfg(target_arch = "x86")]
    println!("cargo:rustc-link-search=static=lib/x86");
    #[cfg(target_arch = "x86_64")]
    println!("cargo:rustc-link-search=static=lib/x64");
}