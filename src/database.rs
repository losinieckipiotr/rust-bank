use serde::{Deserialize, Serialize};
use error_stack::{Context, IntoReport, Report, Result, ResultExt};

use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Client {
  pub card_number: String,
  pub pin: String,
  pub balance: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DatabaseData {
  pub clients: HashMap<String, Client>,
}

impl DatabaseData {
  pub fn new() -> Self {
    DatabaseData { clients: HashMap::new() }
  }
}

#[derive(Debug)]
pub enum JsonDatabaseError {
  Serialization,
  Deserialization,
  SavingDatabaseFile,
  ReadingDatabaseFile,
  ClientNotFound,
}

impl fmt::Display for JsonDatabaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      JsonDatabaseError::Serialization => write!(f, "conversion data to json string failed"),
      JsonDatabaseError::Deserialization => write!(f, "json string conversion to data failed"),
      JsonDatabaseError::ReadingDatabaseFile => write!(f, "reading database file failed"),
      JsonDatabaseError::SavingDatabaseFile => write!(f, "saving database file failed"),
      JsonDatabaseError::ClientNotFound => write!(f, "client not found in database"),
    }
  }
}

impl Context for JsonDatabaseError {}

type JsonDataBaseResult<T> = Result<T, JsonDatabaseError>;

pub trait Database {
  fn name(&self) -> &str;
  fn save_client(&mut self, client: Client) -> JsonDataBaseResult<()>;
  fn has_client(&self, card_number: &str) -> bool;
  fn get_client(&self, card_number: &str) -> JsonDataBaseResult<Client>;
  fn get_data(&self) -> DatabaseData;
}

pub struct JsonDb {
  data: DatabaseData,
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
      println!("error: {:?}", error);
      panic!("Failed to read database");
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

        let json = json_impl::data_to_json_str(&self.data)?;

        let write_json_to_file = self.write_json_to_file.as_ref();

        write_json_to_file(&json)
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
}

impl Database for JsonDb {
  fn name(&self) -> &str {
    "json"
  }

  fn save_client(&mut self, client: Client) -> JsonDataBaseResult<()> {
    // make copy to rollback changes in case of error
    let mut data_copy = self.data.clone();
    data_copy.clients.insert(client.card_number.clone(), client.clone());

    let write_json_to_file = self.write_json_to_file.as_ref();

    let json = json_impl::data_to_json_str(&data_copy)?;

    write_json_to_file(&json).or_else(|e| {
      Err(e.attach_printable(format!("failed to save client: {:?}", client)))
    })?;

    // sync data in struct
    self.data = data_copy;

    Ok(())
  }

  fn has_client(&self, card_number: &str) -> bool {
    self.data.clients.contains_key(card_number)
  }

  fn get_client(&self, card_number: &str) -> JsonDataBaseResult<Client> {
    match self.data.clients.get(card_number) {
      None => Err(Report::new(JsonDatabaseError::ClientNotFound)),
      Some(client) => Ok(client.clone()),
    }
  }

  fn get_data(&self) -> DatabaseData {
    self.data.clone()
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

    from_str(json).or_else(|err| {
      Err(err)
      .report()
      .attach_printable_lazy(|| {
        let mut s = String::from("json:\n");
        s.push_str(json);
        s
      })
      .change_context(JsonDatabaseError::Deserialization)
    })
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
      .read(true)
      .write(true)
      .create(true)
      .open(FILE_NAME)
      .report()
      .attach_printable(format!("failed to open file: {}", FILE_NAME))
      .change_context(JsonDatabaseError::SavingDatabaseFile)?;

    file.write_all(json.as_bytes())
    .report()
    .attach_printable(format!("failed write to file: {}", FILE_NAME))
    .change_context(JsonDatabaseError::SavingDatabaseFile)?;

    Ok(())
  }
}

pub struct SqliteDb {}

impl Database for SqliteDb {
  fn name(&self) -> &str {
    "sqlite"
  }

  fn save_client(&mut self, _client: Client) -> JsonDataBaseResult<()> {
    panic!("Not implemented!");
  }

  fn has_client(&self, _card_number: &str) -> bool {
    panic!("Not implemented!");
  }

  fn get_client(&self, _card_number: &str) -> JsonDataBaseResult<Client> {
    panic!("Not implemented!");
  }

  fn get_data(&self) -> DatabaseData {
    panic!("Not implemented!");
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub fn get_mock_json_db() -> JsonDb {
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

  pub fn get_mock_client() -> Client {
    Client {
      card_number: String::from("4000000000000000"),
      pin: String::from("1234"),
      balance: 0
    }
  }

  #[test]
  fn should_save_client_to_json() {
    let client_mock = get_mock_client();

    let card_number = client_mock.card_number.clone();
    let pin = client_mock.pin.clone();
    let balance = client_mock.balance;

    let mut json_db = get_mock_json_db();

    json_db.write_json_to_file = Box::new(move |json| {
      let data: DatabaseData = serde_json::from_str(json).unwrap();
      let client = data.clients.get(&card_number).unwrap();

      assert_eq!(client.card_number, card_number);
      assert_eq!(client.pin, pin);
      assert_eq!(client.balance, balance);

      Ok(())
    });

    json_db.save_client(client_mock).expect("client saved");
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
