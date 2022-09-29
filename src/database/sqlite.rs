use crate::Database;
use crate::Client;
use crate::DatabaseResult;
use crate::DatabaseData;

// use error_stack::{Context, Result};

use error_stack::Context;

use std::fmt;

// pub type SQLiteDataBaseResult<T> = Result<T, SQLiteDatabaseError>;

#[derive(Debug)]
pub struct SQLiteDatabaseError;

impl fmt::Display for SQLiteDatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "sqlite operation failed")
  }
}

impl Context for SQLiteDatabaseError {}

pub struct SqliteDb {}

impl Database for SqliteDb {
  fn name(&self) -> &str {
    "sqlite"
  }

  fn save_client(&mut self, _client: Client) -> DatabaseResult<()> {
    panic!("Not implemented!");
  }

  fn save_clients(&mut self, _clients: &[Client]) -> DatabaseResult<()> {
    panic!("Not implemented!");
  }

  fn has_client(&self, _card_number: &str) -> bool {
    panic!("Not implemented!");
  }

  fn get_client(&self, _card_number: &str) -> DatabaseResult<Client> {
    panic!("Not implemented!");
  }

  fn remove_client(&mut self, _card_number: &str) -> DatabaseResult<Client> {
    panic!("Not implemented!");
  }

  fn add_funds(&mut self, _funds: u32, _card_number: &str) -> DatabaseResult<()> {
    panic!("Not implemented")
  }

  fn transfer_funds(&mut self, _funds: u32, _sender_card_number: &str, _receiver_card_number: &str) -> DatabaseResult<()> {
    panic!("not implemented")
  }

  fn get_data(&self) -> DatabaseData {
    panic!("Not implemented!");
  }
}
