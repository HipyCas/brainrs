use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BrainrsConfig {
  #[serde(default)]
  tape: TapeConfig,
  #[serde(default)]
  edition: Edition,
}

#[derive(Debug, Deserialize, Serialize)]
enum Edition {
  Classic,
  Extended,
}

impl Default for Edition {
  fn default() -> Self {
    Self::Extended
  }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TapeConfig {
  #[serde(default)]
  fixed: bool,
  #[serde(default)]
  size: usize,
  #[serde(default)]
  item_size: ItemSize,
}

// TODO This won't work, instead create a ItemSize.rs file and include!() it in the generic for the vec, people can customize this at build-time only
#[derive(Debug, Serialize, Deserialize)]
enum ItemSize {
  U8,
  U16,
  U32,
  U64,
  U128,
  F32,
  F64,
}

impl Default for ItemSize {
  fn default() -> Self {
    Self::U8
  }
}
