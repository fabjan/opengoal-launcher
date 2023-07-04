use serde::{Serialize, Serializer};

pub mod binaries;
pub mod config;
pub mod game;
pub mod logging;
pub mod support;
pub mod versions;
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  NetworkRequest(#[from] reqwest::Error),
  #[error("{0}")]
  Configuration(String),
  #[error(transparent)]
  TauriEvent(#[from] tauri::Error),
  #[error("{0}")]
  Installation(String),
  //#[error("{0}")]
  //VersionManagement(String),
  #[error("{0}")]
  GameManagement(String),
  #[error("{0}")]
  OSOperation(String),
  #[error("{0}")]
  WindowManagement(String),
  #[error("{0}")]
  BinaryExecution(String),
  #[error("{0}")]
  Support(String),
}

impl Serialize for CommandError {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

#[derive(Debug, Serialize)]
pub struct CmdErr(String);

impl std::error::Error for CmdErr {}

impl std::fmt::Display for CmdErr {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<anyhow::Error> for CmdErr {
  fn from(err: anyhow::Error) -> Self {
    match err.source() {
      Some(source) => CmdErr(format!("{}: {}", err, source)),
      None => CmdErr(err.to_string()),
    }
  }
}

impl CmdErr {
  pub fn new(err: impl ToString) -> Self {
    CmdErr(err.to_string())
  }
}
