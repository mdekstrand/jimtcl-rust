use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct JimError {
  pub message: String,
}

impl fmt::Display for JimError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.message)
  }
}

pub type JimResult<T> = Result<T, JimError>;
