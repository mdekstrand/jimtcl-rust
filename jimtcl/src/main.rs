use std::{process, mem};

use jimtcl::{interp::Interp, JimResult};

fn main() -> JimResult<()> {
  let interp = Interp::new()?;
  interp.register_core_commands();
  interp.init_static_extensions()?;
  let rc = interp.interactive_prompt()?;
  mem::drop(interp);
  process::exit(rc);
}
