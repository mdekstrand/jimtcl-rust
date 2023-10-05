use std::{process, mem};

use jimtcl::{interp::Interp, JimResult};

fn main() -> JimResult<()> {
  let interp = Interp::new()?;
  interp.register_core_commands();
  let rc = interp.interactive_prompt()?;
  mem::drop(interp);
  process::exit(rc);
}
