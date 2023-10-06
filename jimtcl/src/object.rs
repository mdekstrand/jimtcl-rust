use std::fmt;
use std::os::raw::c_int;
use std::slice;
use std::str;

use crate::sys;
use crate::interp::Interp;

pub struct Object<'a> {
  interp: &'a Interp,
  obj: * mut sys::Jim_Obj,
}

impl <'a> Object<'a> {
  pub(crate) fn wrap(interp: &'a Interp, obj: * mut sys::Jim_Obj) -> Object<'a> {
    Object { interp, obj }
  }
}

impl <'a> fmt::Display for Object<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    unsafe {
      let mut len: c_int = 0;
      let ptr = sys::Jim_GetString(self.obj, &mut len) as *const u8;
      let slice = slice::from_raw_parts(ptr, len as usize);
      let repr = str::from_utf8(slice).expect("invalid string repr");
      f.write_str(repr)
    }
  }
}

impl <'a> Drop for Object<'a> {
  fn drop(&mut self) {
    unsafe {
      (*self.obj).refCount -= 1;
      if (*self.obj).refCount <= 0 {
        self.interp.free_obj(self.obj);
      }
    }
  }
    
}
