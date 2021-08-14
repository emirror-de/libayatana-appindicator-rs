extern crate bindgen;
extern crate pkg_config;

use std::path::PathBuf;

const LIBRARY_NAME: &'static str = "ayatana-appindicator3";
const VERSION: &'static str = "0.1";

fn write_bindings(library: pkg_config::Library) {
    let mut bindings = bindgen::Builder::default()
        .header(&format!("{}.h", LIBRARY_NAME))
        // Hide Gtk types, as these will be filled in via gtk-sys
        .blocklist_type("Gtk.*")
        .allowlist_type(".*AppIndicator.*")
        .allowlist_function("app_indicator_.*");

    for p in library.include_paths {
        bindings = bindings
            .clang_arg("-I")
            .clang_arg(format!("{}", p.as_path().display()));
    }

    let gen_bindings =
        bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("./build");
    gen_bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rustc-link-lib={}", LIBRARY_NAME);

    match pkg_config::probe_library(&format!("{}-{}", LIBRARY_NAME, VERSION)) {
        Ok(library) => write_bindings(library),
        Err(_) => panic!("{} library not found!", LIBRARY_NAME),
    };
}
