use std::env;
use std::fs::create_dir_all;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::str::FromStr;

fn run_cmd(stage: &str, mut cmd: Command) {
  let mut proc = cmd.spawn().expect("error spawning process");
  let res = proc.wait().expect("error waiting for process");
  if !res.success() {
    eprintln!("{}: failed with code {}", stage, res);
    panic!("jim: build stage {} failed", stage);
  }
}

fn create_bindings() {
  println!("cargo:rerun-if-changed=jim-wrapper.h");

  let bindings = bindgen::Builder::default()
    .clang_arg("-Ijimtcl")
    .header("jim-wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .allowlist_function("Jim_.*")
    .allowlist_var("JIM_.*")
    .generate()
    .expect("Unable to generate bindings");

   // Write the bindings to the $OUT_DIR/bindings.rs file.
   let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
   bindings
       .write_to_file(out_path.join("bindings.rs"))
       .expect("Couldn't write bindings!");
}

fn compile_jimtcl(src: &Path, build: &Path) {
  let mut configure = src.canonicalize().expect("could not resolve source path");
  configure.push("configure");
  let target = env::var("TARGET").expect("no TARGET specified");
  let host = env::var("HOST").expect("no HOST specified");

  create_dir_all(build).expect("error creating build directory");

  eprintln!("configuring jim");
  eprintln!("path: {:?}", &configure);
  eprintln!("build path: {:?}", build);
  let mut cmd = Command::new("sh");
  cmd.current_dir(build);
  cmd.arg(configure);
  if target != host {
    cmd.arg(&format!("--host={}", target));
  }
  cmd.arg("--without-ext=default");
  run_cmd("configure", cmd);

  eprintln!("building jim");
  let mut cmd = Command::new("make");
  cmd.current_dir(build);
  run_cmd("make", cmd);

  println!("cargo:rustc-link-search={}", build.to_str().unwrap());
  println!("cargo:rustc-link-lib=jim");
}

fn main() {
  let build = env::var("OUT_DIR").expect("no OUT specified");
  let build = PathBuf::from_str(&build).expect("invalid OUT path");

  let src = PathBuf::from_str("jimtcl").expect("internal error");

  println!("cargo:rerun-if-changed=build.rs");
  compile_jimtcl(&src, &build);
  create_bindings();
}
