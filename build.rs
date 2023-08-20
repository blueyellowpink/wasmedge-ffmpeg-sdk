use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");

    // witc plugin wasmedge_opencvmini.wit > src/generated.rs
    let output = Command::new("witc")
        .arg("plugin")
        .arg("wasmedge_ffmpeg.wit")
        .output()
        .expect("Failed to execute command");
    println!("status: {}", output.status);

    fs::write(&dest_path, String::from_utf8_lossy(&output.stdout).as_ref()).unwrap();
}
