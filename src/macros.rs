#[macro_export]
macro_rules! fail {
  ($expr:expr) => (
    return Err(::std::error::FromError::from_error($expr));
  )
}

#[macro_export]
macro_rules! try (
    ($expr:expr) => ({
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(::std::error::FromError::from_error(err))
        }
    });

    ($expr:expr, $err:expr) => ({
        match $expr {
            Ok(val) => val,
            Err(_) => return Err(::std::error::FromError::from_error($err))
        }
    });
)
