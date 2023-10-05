use crate::prelude::*;
use crate::sys;

pub struct Interp {
  interp: * mut sys::Jim_Interp,
}

impl Interp {
  pub fn new() -> JimResult<Interp> {
    let interp = unsafe {
      sys::Jim_CreateInterp()
    };
    if interp.is_null() {
      panic!("could not allocate Jim interpreter");
    }
    Ok(Interp { interp })
  }

  pub fn register_core_commands(&self) {
    unsafe {
      sys::Jim_RegisterCoreCommands(self.interp);
    }
  }
}
