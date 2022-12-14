pub mod json;
pub mod sqlite;

use serde::{Deserialize, Serialize};
use error_stack::{Context, Result};

pub use sqlite::*;
pub use json::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Client {
  pub card_number: String,
  pub pin: String,
  pub balance: i32,
}

#[derive(Debug)]
pub enum DatabaseError {
  JSON,
  SQLite,
}

impl std::fmt::Display for DatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "database operation failed")
  }
}

impl Context for DatabaseError {}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

pub trait Database {
  fn name(&self) -> &str;
  fn save_new_client(&mut self, client: Client) -> DatabaseResult<()>;
  fn has_client(&self, card_number: &str) -> DatabaseResult<bool>;
  fn get_client(&self, card_number: &str) -> DatabaseResult<Client>;
  fn remove_client(&mut self, card_number: &str) -> DatabaseResult<Client>;
  fn add_funds(&mut self, funds: u32, card_number: &str) -> DatabaseResult<()>;
  fn transfer_funds(&mut self, funds: u32, sender_card_number: &str, receiver_card_number: &str) -> DatabaseResult<()>;
  fn get_clients_count(&self) -> DatabaseResult<u32>; // TODO remove, used only in tests
}

#[allow(dead_code)]
pub mod tests {
  use crate::Client;

  pub fn get_mock_client() -> Client {
    Client {
      card_number: String::from("4000000000000000"),
      pin: String::from("1234"),
      balance: 0
    }
  }
}
