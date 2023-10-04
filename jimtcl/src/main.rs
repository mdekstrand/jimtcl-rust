use std::process::exit;

use jimtcl::sys;

fn main() {
  unsafe {
    let interp = sys::Jim_CreateInterp();
    sys::Jim_RegisterCoreCommands(interp);
    sys::Jim_SetVariableStrWithStr(interp, "tcl_interactive".as_ptr() as *const i8, "1".as_ptr() as *const i8);

    let rc = sys::Jim_InteractivePrompt(interp) as u32;
    let rc = if rc == sys::JIM_EXIT {
      sys::Jim_GetExitCode(interp)
    } else if rc == sys::JIM_ERR {
      1
    } else {
      0
    };

    exit(rc)
  };
}
