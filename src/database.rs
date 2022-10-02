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
pub enum DatabaseError {
  JSON,
  SQLite,
}

impl fmt::Display for DatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "database operation failed")
  }
}

impl Context for DatabaseError {}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

pub trait Database {
  fn name(&self) -> &str;
  fn save_new_client(&mut self, client: Client) -> DatabaseResult<()>;
  fn has_client(&self, card_number: &str) -> bool;
  fn get_client(&self, card_number: &str) -> DatabaseResult<Client>;
  fn remove_client(&mut self, card_number: &str) -> DatabaseResult<Client>;
  fn add_funds(&mut self, funds: u32, card_number: &str) -> DatabaseResult<()>;
  fn transfer_funds(&mut self, funds: u32, sender_card_number: &str, receiver_card_number: &str) -> DatabaseResult<()>;
  fn get_clients_count(&self) -> u32; // TODO remove, used only in tests
}
