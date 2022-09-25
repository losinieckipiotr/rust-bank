mod json;
mod sqlite;

use serde::{Deserialize, Serialize};
use error_stack::{Context, Result};

use std::collections::BTreeMap;
use std::fmt;

pub use sqlite::*;
pub use json::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Client {
  pub card_number: String,
  pub pin: String,
  pub balance: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DatabaseData {
  pub clients: BTreeMap<String, Client>,
}

impl DatabaseData {
  pub fn new() -> Self {
    DatabaseData { clients: BTreeMap::new() }
  }
}

#[derive(Debug)]
pub enum JsonDatabaseError {
  Serialization,
  Deserialization,
  SavingDatabaseFile,
  ReadingDatabaseFile,
  ClientNotFound,
  InsufficientFunds,
}

impl fmt::Display for JsonDatabaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      JsonDatabaseError::Serialization => write!(f, "conversion data to json string failed"),
      JsonDatabaseError::Deserialization => write!(f, "json string conversion to data failed"),
      JsonDatabaseError::ReadingDatabaseFile => write!(f, "reading database file failed"),
      JsonDatabaseError::SavingDatabaseFile => write!(f, "saving database file failed"),
      JsonDatabaseError::ClientNotFound => write!(f, "client not found in database"),
      JsonDatabaseError::InsufficientFunds => write!(f, "operation failed due to insufficient funds")
    }
  }
}

impl Context for JsonDatabaseError {}

pub type JsonDataBaseResult<T> = Result<T, JsonDatabaseError>;

pub trait Database {
  fn name(&self) -> &str;
  fn save_client(&mut self, client: Client) -> JsonDataBaseResult<()>;
  fn save_clients(&mut self, clients: &[Client]) -> JsonDataBaseResult<()>;
  fn has_client(&self, card_number: &str) -> bool;
  fn get_client(&self, card_number: &str) -> JsonDataBaseResult<Client>;
  fn remove_client(&mut self, card_number: &str) -> JsonDataBaseResult<Client>;
  fn add_funds(&mut self, funds: u32, card_number: &str) -> JsonDataBaseResult<()>;
  fn transfer_funds(&mut self, funds: u32, sender_card_number: &str, receiver_card_number: &str) -> JsonDataBaseResult<()>;
  fn get_data(&self) -> DatabaseData;
}
