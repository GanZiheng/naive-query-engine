/*
 * @Author: Veeupup
 * @Date: 2022-05-12 16:45:51
 * @Email: code@tanweime.com
*/
use std::io;
use arrow::error::ArrowError;

pub type Result<T> = std::result::Result<T, VeeError>;

#[derive(Debug)]
pub enum VeeError {
    /// Error return by arrow
    ArrowError(ArrowError),

    IoError(io::Error),

    #[allow(unused)]
    OtherError,
}

impl From<ArrowError> for VeeError {
    fn from(e: ArrowError) -> Self {
        VeeError::ArrowError(e)
    }
}

impl From<io::Error> for VeeError {
  fn from(e: io::Error) -> Self {
      VeeError::IoError(e)
  }
}
