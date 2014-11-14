#![feature(struct_variant)]
#![feature(macro_rules)]

use std::error::{Error, FromError};
use std::fmt::{Show, Formatter, FormatError};

#[macro_escape]
mod macros;

pub enum StandardError {
  StandardErrorActual { description: &'static str },
  StandardErrorWrapped(Box<Error+Send+'static>)
}

impl StandardError {
  pub fn description(&self) -> &str {
    match *self {
      StandardErrorActual { description } => description,
      StandardErrorWrapped(ref err) => err.description()
    }
  }

  pub fn detail(&self) -> Option<String> {
    match *self {
      StandardErrorActual { description: _ } => None,
      StandardErrorWrapped(ref err) => err.detail()
    }
  }

  pub fn cause(&self) -> Option<&Error> {
    match *self {
      StandardErrorActual { description: _ } => None,
      StandardErrorWrapped(ref err) => err.cause()
    }
  }
}

impl<T> FromError<T> for StandardError where T: Error {
  fn from_error(err: T) -> StandardError {
    StandardErrorWrapped(box err)
  }
}

impl FromError<&'static str> for StandardError {
  fn from_error(err: &'static str) -> StandardError {
    StandardErrorActual { description: err }
  }
}

impl Show for StandardError {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
    formatter.write(self.description().as_bytes())
  }
}

pub type StandardResult<T> = Result<T, StandardError>;

#[cfg(test)]
mod test {
  use std::io::File;
  use super::{StandardResult};

  fn success() -> StandardResult<String> {
    Ok("Hello".to_string())
  }

  fn read_file(path: &Path) -> StandardResult<String> {
    let buffer = try!(File::open(path).read_to_end());
    let string = buffer.into_ascii().into_string();
    Ok(string)
  }

  fn fail() -> StandardResult<String> {
    fail!("OMG!")
  }

  #[test]
  fn test_success() {
    assert_eq!("Hello", success().unwrap().as_slice());
  }

  #[test]
  fn test_io_error() {
    let result = read_file(&Path::new("./Hello"));
    match result {
      Ok(_) => panic!("should fail!"),
      Err(err) => {
        assert_eq!(err.description(), "couldn't open file");
        assert!(err.detail().unwrap().as_slice().contains("no such file or directory"))
      }
    }
  }

  #[test]
  fn test_fail_macro_error() {
    let result = fail();
    match result {
      Ok(_) => panic!("should fail!"),
      Err(err) => assert_eq!(err.description(), "OMG!")
    }
  }
}
