use crate::Database;
use crate::Client;
use crate::JsonDataBaseResult;
use crate::DatabaseData;

pub struct SqliteDb {}

impl Database for SqliteDb {
  fn name(&self) -> &str {
    "sqlite"
  }

  fn save_client(&mut self, _client: Client) -> JsonDataBaseResult<()> {
    panic!("Not implemented!");
  }

  fn save_clients(&mut self, _clients: &[Client]) -> JsonDataBaseResult<()> {
    panic!("Not implemented!");
  }

  fn has_client(&self, _card_number: &str) -> bool {
    panic!("Not implemented!");
  }

  fn get_client(&self, _card_number: &str) -> JsonDataBaseResult<Client> {
    panic!("Not implemented!");
  }

  fn remove_client(&mut self, _card_number: &str) -> JsonDataBaseResult<Client> {
    panic!("Not implemented!");
  }

  fn add_funds(&mut self, _funds: u32, _card_number: &str) -> JsonDataBaseResult<()> {
    panic!("Not implemented")
  }

  fn transfer_funds(&mut self, _funds: u32, _sender_card_number: &str, _receiver_card_number: &str) -> JsonDataBaseResult<()> {
    panic!("not implemented")
  }

  fn get_data(&self) -> DatabaseData {
    panic!("Not implemented!");
  }
}
