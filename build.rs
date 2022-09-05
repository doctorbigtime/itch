use std::process::Command;
// use std::env;
use std::fs::File;
// use std::os::unix::io::IntoRawFd;
// use std::path::Path;

fn main() {
  // let out_dir = env::var("OUT_DIR").unwrap();
  let output = File::create("src/itch.rs").unwrap();
  Command::new("python").args(&["src/codegen.py", "src/nasdaq_totalview_itch.xml"])
    .stdout(output)
    .status().unwrap();

  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/codegen.py");
  println!("cargo:rerun-if-changed=src/nasdaq_totalview_itch.xml");
}
