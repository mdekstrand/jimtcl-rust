use thiserror::Error;
use parse_display::Display;
use crate::sys;

#[derive(Debug, Display)]
#[repr(u32)]
pub enum ExitCode {
  Ok = sys::JIM_OK,
  Err = sys::JIM_ERR,
  Return = sys::JIM_RETURN,
  Break = sys::JIM_BREAK,
  Continue = sys::JIM_CONTINUE,
  Signal = sys::JIM_SIGNAL,
  Exit = sys::JIM_EXIT,
  #[display("format")]
  InvalidCode(u32),
}

impl From<u32> for ExitCode {
  fn from(value: u32) -> Self {
    match value {
      sys::JIM_OK => ExitCode::Ok,
      sys::JIM_ERR => ExitCode::Err,
      sys::JIM_RETURN => ExitCode::Return,
      sys::JIM_BREAK => ExitCode::Break,
      sys::JIM_CONTINUE => ExitCode::Continue,
      sys::JIM_SIGNAL => ExitCode::Signal,
      sys::JIM_EXIT => ExitCode::Exit,
      x => ExitCode::InvalidCode(x),
    }
  }
}

#[derive(Error, Debug)]
pub enum JimError {
  #[error("TCL evaluation error: {0}")]
  Error(String),
  #[error("unexpected TCL return code {0}")]
  UnexpectedReturnCode(ExitCode),
}

pub type JimResult<T> = Result<T, JimError>;
