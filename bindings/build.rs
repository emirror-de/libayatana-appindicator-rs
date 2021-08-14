#[cfg(not(feature = "dynamic"))]
const LIBRARY_NAME: &'static str = "ayatana-appindicator3";

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("cargo:rustc-link-lib={}", LIBRARY_NAME);
}

#[cfg(feature = "dynamic")]
fn main() {}
