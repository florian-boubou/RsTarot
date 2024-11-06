use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct TarotValueError {
  value: u8
}

impl TarotValueError {
  pub fn new(value: u8) -> Self {
    Self {
      value
    }
  }
}

#[derive(Debug)]
pub struct PipValueError {
  inner: TarotValueError
}

impl PipValueError {
  pub fn new(value: u8) -> Self {
    Self {
      inner: TarotValueError::new(value)
    }
  }
}

impl Display for PipValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the value {} doesn't represent any pip card", self.inner.value)
    }
}

impl Error for PipValueError {}

#[derive(Debug)]
pub struct TrumpValueError {
  inner: TarotValueError
}

impl TrumpValueError {
  pub fn new(value: u8) -> Self {
    TrumpValueError {
      inner: TarotValueError::new(value)
    }
  }
}

impl Display for TrumpValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the value {} doesn't represent any pip card", self.inner.value)
    }
}

impl Error for TrumpValueError {

}
