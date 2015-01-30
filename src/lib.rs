#![feature(core)]

use std::error::{Error, FromError};

#[macro_use]
mod macros;

pub enum StandardError {
  Actual { description: &'static str },
  Wrapped(Box<Error+'static>)
}

impl StandardError {
  pub fn description(&self) -> &str {
    match *self {
      StandardError::Actual { description } => description,
      StandardError::Wrapped(ref err) => err.description()
    }
  }

  pub fn cause(&self) -> Option<&Error> {
    match *self {
      StandardError::Actual { description: _ } => None,
      StandardError::Wrapped(ref err) => err.cause()
    }
  }
}

impl<T> FromError<T> for StandardError where T: Error+'static {
  fn from_error(err: T) -> StandardError {
    StandardError::Wrapped(Box::new(err))
  }
}

impl FromError<&'static str> for StandardError {
  fn from_error(err: &'static str) -> StandardError {
    StandardError::Actual { description: err }
  }
}

impl std::fmt::Debug for StandardError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    formatter.write_str(self.description())
  }
}

pub type StandardResult<T> = Result<T, StandardError>;

#[cfg(test)]
mod test {

  use std::old_io::File;
  use super::{StandardResult};

  fn success() -> StandardResult<String> {
    Ok("Hello".to_string())
  }

  fn read_file(path: &Path) -> StandardResult<String> {
    let buffer = try!(File::open(path).read_to_end());
    let result = try!(String::from_utf8(buffer), "cannot read binary file");
    Ok(result)
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
        assert_eq!(err.description(), "couldn't open path as file");
      }
    }
  }

  #[test]
  fn test_try_with_message() {
    let result = read_file(&Path::new("./binary_file"));
    match result {
      Ok(_) => panic!("should fail!"),
      Err(err) => assert_eq!(err.description(), "cannot read binary file")
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
