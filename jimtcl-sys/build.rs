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

fn create_bindings(build_dir: &Path) {
  println!("cargo:rerun-if-changed=jim-wrapper.h");
  let out_path = build_dir.to_owned();

  let bindings = bindgen::Builder::default()
    .clang_arg("-Ijimtcl")
    .clang_arg(format!("-I{}", build_dir.to_str().expect("invalid path encoding")))
    .header("jimtcl/jim.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .allowlist_function("Jim_.*")
    .allowlist_var("JIM_.*")
    .generate()
    .expect("Unable to generate bindings");

   // Write the bindings to the $OUT_DIR/bindings.rs file.
   bindings
       .write_to_file(out_path.join("bindings.rs"))
       .expect("Couldn't write bindings!");
}

fn compile_jimtcl(src: &Path, build_dir: &Path) {
  let mut configure = src.canonicalize().expect("could not resolve source path");
  configure.push("configure");
  let target = env::var("TARGET").expect("no TARGET specified");
  let host = env::var("HOST").expect("no HOST specified");

  create_dir_all(build_dir).expect("error creating build directory");

  let mut ccb = cc::Build::new();
  ccb.include("jimtcl");
  ccb.include(build_dir);

  eprintln!("configuring jim");
  eprintln!("path: {:?}", &configure);
  eprintln!("build path: {:?}", build_dir);
  let mut cmd = Command::new("sh");
  cmd.current_dir(build_dir);
  let cc = ccb.get_compiler();
  let ar = ccb.get_archiver();
  let rl = ccb.get_ranlib();
  cmd.env("CC", cc.cc_env());
  cmd.env("CFLAGS", cc.cflags_env());
  cmd.env("AR", ar.get_program());
  cmd.env("RANLIB", rl.get_program());

  cmd.arg(configure);
  if target != host {
    cmd.arg(&format!("--host={}", target));
  }
  cmd.arg("--without-ext=default");
  run_cmd("configure", cmd);

  eprintln!("building jim");
  let mut cmd = Command::new("make");
  cmd.current_dir(build_dir);
  cmd.arg("libjim.a");
  run_cmd("make", cmd);

  println!("cargo:rustc-link-search={}", build_dir.to_str().unwrap());
  println!("cargo:rustc-link-lib=jim");
}

fn main() {
  let build_dir = env::var("OUT_DIR").expect("no OUT specified");
  let build_dir = PathBuf::from_str(&build_dir).expect("invalid OUT path");

  let src = PathBuf::from_str("jimtcl").expect("internal error");

  println!("cargo:rerun-if-changed=build.rs");
  compile_jimtcl(&src, &build_dir);
  create_bindings(&build_dir);
}
