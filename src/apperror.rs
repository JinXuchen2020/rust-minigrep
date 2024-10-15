use std::fs::{read_to_string, File};
use std::fmt::Display;
use std::{io, num};

#[derive(Debug)]
pub struct AppError {
  kind: String,    // 错误类型
  message: String, // 错误信息
}

// 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
// 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
impl From<io::Error> for AppError {
  fn from(error: io::Error) -> Self {
    AppError {
      kind: String::from("io"),
      message: error.to_string(),
    }
  }
}

impl From<num::ParseIntError> for AppError {
  fn from(error: num::ParseIntError) -> Self {
      AppError {
          kind: String::from("parse"),
          message: error.to_string(),
      }
  }
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "AppError: {} - {}", self.kind, self.message)
  }    
}

impl std::error::Error for AppError {}

#[derive(thiserror::Error, Debug)]
pub enum MyError {
  #[error("Environment variable not found")]
  EnvironmentVariableNotFound(#[from] std::env::VarError),
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

pub fn run() -> Result<(), AppError> {
  let _file = File::open("poem.txt")?;

  let _number: usize;
  let content = "not a number";
  _number = content.parse()?;

  Ok(())
}

pub fn render() -> Result<String, MyError> {
  let file = std::env::var("MARKDOWN")?;
  let source = read_to_string(file)?;
  Ok(source)
}