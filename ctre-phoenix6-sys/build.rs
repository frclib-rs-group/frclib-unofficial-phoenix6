
fn main() {
    //Linking to dynamic lib `libCTRE_PhoenixTools.so`
    // use `bindgen` crate to generate the bindings

    bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    #[cfg(all(target_os = "linux", frc_real))] {
        println!("cargo:rustc-link-search=native={}", env!("FRC_THIRD_PARTY_LIBS"));
        println!("cargo:rustc-link-lib=dylib=CTRE_PhoenixTools");
    }
}