use crate::Database;
use crate::Client;
use crate::{DatabaseError, DatabaseResult};
use crate::DatabaseData;

use error_stack::{Context, Result, IntoReport, Report, ResultExt};

use std::fmt;

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
    match self {
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

pub struct JsonDb {
  data: DatabaseData, // TODO do not store data in RAM
  read_json_file: Box<dyn Fn() -> JsonDataBaseResult<String>>,
  write_json_to_file: Box<dyn Fn(&str) -> JsonDataBaseResult<()>>,
}

impl JsonDb {
  pub fn new() -> Self {
    let mut db = JsonDb {
      data: DatabaseData::new(),
      read_json_file: Box::new(fs_impl::read_json_file),
      write_json_to_file: Box::new(fs_impl::write_json_to_file),
    };

    if let Err(error) = db.read_data() {
      println!("\n failed to read json database, error: {:?}", error);
      panic!("JsonDb::new() failed");
    }

    db
  }

  fn read_data(&mut self) -> JsonDataBaseResult<()> {
    let read_json_file = self.read_json_file.as_ref();

    match read_json_file() {
      Err(e) => {
        println!("try to create new database file because: {:?}", e);

        // self.data must be empty
        assert_eq!(self.data,  DatabaseData::new());

        self.save_data()
          .attach_printable("creating new database file failed")?;
      },
      Ok(str) => {
        let red_data = json_impl::data_from_json(&str)?;
        // sync data
        self.data = red_data;
      }
    };

    Ok(())
  }

  fn save_data(&self) -> JsonDataBaseResult<()> {
    let write_json_to_file = self.write_json_to_file.as_ref();

    let json = json_impl::data_to_json_str(&self.data)?;

    write_json_to_file(&json)
      .attach_printable_lazy(|| {
        format!("failed to save data")
      })?;

    Ok(())
  }

  fn save_clients(&mut self, clients: &[Client]) -> DatabaseResult<()> {
    // make copy to rollback changes in case of error
    let data_copy = self.data.clone();

    for client in clients {
      self.data.clients.insert(client.card_number.clone(), client.clone());
    }

    self.save_data()
      .attach_printable_lazy(|| {
        format!("failed to save {} clients: {:?}", clients.len(), clients)
      })
      .or_else(|err| {
        // rollback
        self.data = data_copy;
        Err(err)
      })
      .change_context(DatabaseError::JSON)?;

    Ok(())
  }
}

impl Database for JsonDb {
  fn name(&self) -> &str {
    "json"
  }

  fn save_new_client(&mut self, client: Client) -> DatabaseResult<()> {
    // TODO check if already exists
    self.data.clients.insert(
      client.card_number.clone(),
      client.clone()
    );

    self.save_data()
      .attach_printable_lazy(|| {
        format!("failed insert new client, client: {:?}", client)
      })
      .change_context(DatabaseError::JSON)?;

    Ok(())
  }

  fn has_client(&self, card_number: &str) -> DatabaseResult<bool> {
    Ok(self.data.clients.contains_key(card_number))
  }

  fn get_client(&self, card_number: &str) -> DatabaseResult<Client> {
    match self.data.clients.get(card_number) {
      None => Err(Report::new(JsonDatabaseError::ClientNotFound))
        .attach_printable_lazy(|| {
          format!("client with card_number: {} not found", card_number)
        })
        .change_context(DatabaseError::JSON),
      Some(client) => Ok(client.clone()),
    }
  }

  fn remove_client(&mut self, card_number: &str) -> DatabaseResult<Client> {
    let client = match self.data.clients.remove(card_number) {
      None => Err(Report::new(JsonDatabaseError::ClientNotFound))
        .attach_printable_lazy(|| {
          format!("client with card_number: {} is not present in database", card_number)
        })
        .change_context(DatabaseError::JSON),
      Some(client) => Ok(client)
    };

    self.save_data().change_context(DatabaseError::JSON)?;

    client
  }

  fn add_funds(&mut self, funds: u32, card_number: &str) -> DatabaseResult<()> {
    let mut client = self.get_client(card_number)?;

    client.balance += funds as i32;

    self.save_clients(&[client])?;

    Ok(())
  }

  fn transfer_funds(&mut self, funds: u32, sender_card_number: &str, receiver_card_number: &str) -> DatabaseResult<()> {
    let mut sender_client = self.get_client(sender_card_number)
      .attach_printable_lazy(|| {
        format!("sender client not found, sender_card_number: {}", sender_card_number)
      })?;

    let mut receiver_client = self.get_client(receiver_card_number)
      .attach_printable_lazy(|| {
        format!("receiver client not found, receiver_card_number: {}", receiver_card_number)
      })?;

    let sender_original_balance = sender_client.balance;
    sender_client.balance -= funds as i32;

    if sender_client.balance < 0 {
      return Err(Report::new(JsonDatabaseError::InsufficientFunds))
        .attach_printable_lazy(|| {
          format!(
            "sender's balance before transfer: {}, after transfer: {}",
            sender_original_balance,
            sender_client.balance
          )
        })
        .change_context(DatabaseError::JSON)
    }

    receiver_client.balance += funds as i32;

    let clients = [sender_client, receiver_client];

    self.save_clients(&clients)
      .attach_printable_lazy(|| {
        format!("failed to save clients data in database")
      })?;

    Ok(())
  }

  fn get_clients_count(&self) -> u32 {
    self.data.clients.len() as u32
  }
}

mod json_impl {
  use super::*;

  pub fn data_to_json_str(data: &DatabaseData) -> JsonDataBaseResult<String> {
    use serde_json::to_string_pretty;

    to_string_pretty(&data)
    .report()
    .attach_printable(format!("data: {data:?}"))
    .change_context(JsonDatabaseError::Serialization)
  }

  pub fn data_from_json(json: &str) -> JsonDataBaseResult<DatabaseData> {
    use serde_json::from_str;

    from_str(json)
    .report()
    .attach_printable_lazy(|| {
      let mut s = String::from("json:\n");
      s.push_str(json);
      s
    })
    .change_context(JsonDatabaseError::Deserialization)
  }
}

mod fs_impl {
  use super::*;
  use std::fs::OpenOptions;
  use std::io::prelude::*;

  const FILE_NAME: &str = "data.json";

  pub fn read_json_file() -> JsonDataBaseResult<String> {
    std::fs::read_to_string(FILE_NAME)
    .report()
    .attach_printable(format!("failed to read file {}, file not exists?", FILE_NAME))
    .change_context(JsonDatabaseError::ReadingDatabaseFile)
  }

  pub fn write_json_to_file(json: &str) -> JsonDataBaseResult<()> {
    let mut file = OpenOptions::new()
      .create(true)
      .truncate(true)
      .write(true)
      .open(FILE_NAME)
      .report()
      .attach_printable(format!("failed to open file: {}", FILE_NAME))
      .change_context(JsonDatabaseError::SavingDatabaseFile)?;

    let mut json_copy = String::from(json.clone());
    json_copy.push('\n');
    let json_bytes = json_copy.as_bytes();

    file.write_all(json_bytes)
    .report()
    .attach_printable(format!("failed write to file: {}", FILE_NAME))
    .change_context(JsonDatabaseError::SavingDatabaseFile)?;

    Ok(())
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub fn get_mock_db() -> JsonDb {
    let mut json_db = JsonDb {
      data: DatabaseData::new(),
      read_json_file: get_empty_data_mock(),
      write_json_to_file: write_success_mock(),
    };

    let result = json_db.read_data();
    let read_ok = matches!(result, Ok(()));
    assert_eq!(read_ok, true);

    json_db
  }

  #[test]
  fn should_save_client_to_json() {
    let client_mock = crate::database::tests::get_mock_client();

    let card_number = client_mock.card_number.clone();
    let pin = client_mock.pin.clone();
    let balance = client_mock.balance;

    let mut json_db = get_mock_db();

    json_db.write_json_to_file = Box::new(move |json| {
      let data: DatabaseData = serde_json::from_str(json).unwrap();
      let client = data.clients.get(&card_number).unwrap();

      assert_eq!(client.card_number, card_number);
      assert_eq!(client.pin, pin);
      assert_eq!(client.balance, balance);

      Ok(())
    });

    json_db.save_new_client(client_mock).unwrap();
  }

  fn get_empty_data_mock() -> Box<dyn Fn() -> JsonDataBaseResult<String>> {
    Box::new(|| Ok(String::from("{\"clients\":{}}")))
  }

  fn write_success_mock() -> Box<dyn Fn(&str) -> JsonDataBaseResult<()>> {
    Box::new(|_| {
      // Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permissions error test"))
      // .report()
      // .change_context(JsonDatabaseError::SavingDatabaseFile)
      Ok(())
    })
  }
}
