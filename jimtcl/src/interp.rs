use crate::object::Object;
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

  /// Invoke the standard initialization operations.
  pub fn std_init(&self) -> JimResult<()> {
    self.register_core_commands();
    self.init_static_extensions()
  }

  pub fn register_core_commands(&self) {
    unsafe {
      sys::Jim_RegisterCoreCommands(self.interp);
    }
  }

  pub fn init_static_extensions(&self) -> JimResult<()> {
    let rc = unsafe {
      sys::Jim_InitStaticExtensions(self.interp)
    };
    self.require_ok(rc as u32)
  }

  /// Check a return code and succeed if it is OK, returning an error otherwise.
  pub fn require_ok(&self, code: u32) -> JimResult<()> {
    if code == sys::JIM_OK {
      Ok(())
    } else if code == sys::JIM_ERR {
      let obj = unsafe {
        Object::wrap(self, (*self.interp).result)
      };
      Err(JimError::Error(obj.to_string()))
    } else {
      Err(JimError::UnexpectedReturnCode(code.into()))
    }
  }

  pub fn interactive_prompt(&self) -> JimResult<i32> {
    let rc = unsafe {
      sys::Jim_InteractivePrompt(self.interp) as u32
    };
    if rc == sys::JIM_EXIT {
      unsafe {
        Ok(sys::Jim_GetExitCode(self.interp))
      }
    } else {
      self.require_ok(rc)?;
      Ok(0)
    }
  }

  pub(crate) fn free_obj(&self, obj_ptr: * mut sys::Jim_Obj) {
    unsafe {
      sys::Jim_FreeObj(self.interp, obj_ptr);
    }
  }
}

impl Drop for Interp {
  fn drop(&mut self) {
    unsafe {
      sys::Jim_FreeInterp(self.interp);
    }
  }
}
