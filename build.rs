extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link fmod dylib files
    println!("cargo:rustc-link=dylib=libfmod");
    println!("cargo:rustc-link=dylib=libfmodL");
    println!("cargo:rustc-link=dylib=libfmodstudio");
    println!("cargo:rustc-link=dylib=libfmodstudioL");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Instantiate bindgen
    let bindings = bindgen::Builder::default()
        // The header file
        .header("wrapper.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Remove the first instance FMOD_DSP
        // because it is a forward declaration
        // (this seems like the wrong way to do this)
        .blacklist_item("FMOD_DSP")
        // Never trust Clang
        .trust_clang_mangling(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("fmod.rs"))
        .expect("Couldn't write bindings!");
}