# Rust StandardError

This is an experiment at more convenient error handling for Rust.

This adds a type called StandardError, which can either wrap another error or
contain a custom error message. This error defines a conversion from all other
types which implement `error::Error`, thus it can soak up all other errors,
while still providing great error messages. This StandardError type would become
the default Error in Rust, with the `Result` type defaulting to it.

The idea is being able to write code like this:

``` rust
fn read_file(path: &Path) -> Result<String> {
  let buffer = try!(File::open(path).read_to_end());
  let result = try!(String::from_utf8(buffer), "cannot read binary file");
  Ok(result)
}
```

This is more ergonomic than the status quo. Due to the following reasons:

- No need to define a custom error type.
- No need to specify the error type of `Result`.
- Automatic conversion from all types that implement `std::error::Error` with good error messages.
- Ability to specify custom error messages where desired.

## Caveats

1. Actually making this work requires a (non-breaking) change to Result, defaulting the
  Err variant's contained type to StandardError. It would change the definition of result
  to something like:

  ``` rust
  pub enum Result<T, E=std::error::StandardError> {
    Ok(T),
    Err(E),
  }
  ```

  The above code will currently have to be written as:

  ``` rust
  fn read_file(path: &Path) -> Result<String, StandardError> {
    let buffer = try!(File::open(path).read_to_end());
    let result = try!(String::from_utf8(buffer), "cannot read binary file");
    Ok(result)
  }
  ```

  This crate contains a typedef which provides `StandardResult<T>`, which is just
  `Result<T, StandardError>`.

2. StandardError does not currently implement `std::error::Error` itself. This is
  because the blanket implementation in `std`, defining `impl<E> FromError<E> for E`
  makes this impossible.

## Macros

The `try!` macro was expanded to be able to take an additional error parameter
which would be used in case of failure. As above:

``` rust
try!(String::from_utf8(buffer), "cannot read binary file");
```

The `fail!` macro that Armin Ronacher [suggested](http://lucumr.pocoo.org/2014/10/16/on-error-handling/) was added which makes "throwing" error results very convenient. For example:

``` rust
fn always_fail() -> Result<String> {
  fail!("Oh no");
  Ok("yes".to_string())
}
```
