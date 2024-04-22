fn main() {
    if build_target::target_arch().unwrap().as_str() == "x86_64" {
        // Tell cargo to pass the linker script to the linker..
        println!("cargo:rustc-link-arg=-Tlinker-x64.ld");
        // ..and to re-run if it changes.
        println!("cargo:rerun-if-changed=linker-x64.ld");
    }
    #[cfg(feature = "raspi5")]
    if build_target::target_arch().unwrap().as_str() == "aarch64" {
        // Tell cargo to pass the linker script to the linker..
        println!("cargo:rustc-link-arg=-Tlinker-aarch64-raspi5.ld");
        // ..and to re-run if it changes.
        println!("cargo:rerun-if-changed=linker-aarch64-raspi5.ld");
    }
}
