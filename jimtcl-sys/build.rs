use std::env;
use std::path::PathBuf;

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

fn compile_jimtcl() {
  let mut build = cc::Build::new();
  build.include("jimtcl");
  build.include(".");
  build.define("HAVE_NO_AUTOCONF", None);
  build.flag("-Wno-unused-parameter");
  build.flag("-Wno-sign-compare");
  // base Jim TCL source files
  // build.file("_load-static-exts.c");
  build.file("jimtcl/jim-subcmd.c");
  build.file("jimtcl/jim-interactive.c");
  build.file("jimtcl/jim-format.c");
  build.file("jimtcl/jim.c");
  build.file("jimtcl/utf8.c");
  build.file("jimtcl/jimregexp.c");
  build.file("jimtcl/jimiocompat.c");

  // compile JimTCL
  build.compile("jim");
}

fn main() {
  create_bindings();
  compile_jimtcl();
}
