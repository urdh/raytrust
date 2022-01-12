#[rustversion::nightly]
fn nightly() -> bool {
    true
}

#[rustversion::not(nightly)]
fn nightly() -> bool {
    false
}

fn main() {
    if nightly() {
        println!("cargo:rustc-cfg=nightly");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
