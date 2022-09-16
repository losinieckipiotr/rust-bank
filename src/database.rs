use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
  pub card_number: String,
  pub pin: String,
  pub balance: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseJsonData {
  clients: HashMap<String, Client>,
}

pub trait Database {
  fn name(&self) -> &str;
  fn save_client(&mut self, client: Client) -> Result<(), String>;
  fn has_client(&self, card_number: &str) -> bool; // change to result ?
  fn get_client(&self, card_number: &str) -> Result<Client, String>;
}

pub enum JsonDatabaseError {
  CreateDatabaseFile,
  Serialization,
  Deserialization,
  SavingDatabaseFile,
  ClientNotFound,
}

pub struct JsonDb {
  data: Option<DatabaseJsonData>,
  read_json_file: fn() -> std::io::Result<String>,
  write_json_to_file: fn(json: &str) -> Result<(), std::io::Error>,
}

impl JsonDb {
  pub fn new() -> Self {
    let mut db = JsonDb {
      data: None,
      read_json_file: fs_impl::read_json_file,
      write_json_to_file: fs_impl::write_json_to_file
    };

    if let Err(msg) = db.read_data() {
      panic!("Error reading json database: {}", msg);
    }

    db
  }

  fn read_data(&mut self) -> Result<(), String> {
    if let Some(_) = self.data {
      return Ok(());
    }

    let read_json_file = self.read_json_file;

    let data: DatabaseJsonData = match read_json_file() {
      // failed to read data file, try create new
      Err(_) => {
        let empty_data = DatabaseJsonData { clients: HashMap::new() };

        let json = match json_impl::data_to_json_str(&empty_data) {
          // failed to serialize empty_data
          Err(e) => return Err(get_error_str(e)),
          Ok(j) => j,
        };

        let write_json_to_file = self.write_json_to_file;

        match write_json_to_file(&json) {
          // failed to save file
          Err(_) => return Err(get_error_str(JsonDatabaseError::CreateDatabaseFile)),
          Ok(_) => empty_data
        }
      },
      Ok(str) => match json_impl::data_from_json(&str) {
        Ok(d) => d,
        Err(e) => return Err(get_error_str(e)),
      },
    };

    // save in struct
    self.data = Some(data);

    Ok(())
  }
}

impl Database for JsonDb {
  fn name(&self) -> &str {
    "json"
  }

  fn save_client(&mut self, client: Client) -> Result<(), String> {
    // make copy to rollback changes in case of error
    let mut data_copy = self.data.as_mut().expect("initialized data").clone();
    data_copy.clients.insert(client.card_number.clone(), client);

    let write_json_to_file = self.write_json_to_file;

    match json_impl::data_to_json_str(&data_copy) {
      Err(e) => return Err(get_error_str(e)),
      Ok(json) => match write_json_to_file(&json) {
        Err(_) => return Err(get_error_str(JsonDatabaseError::SavingDatabaseFile)),
        Ok(_) => {
          // sync data in struct
          self.data = Some(data_copy);
          Ok(())
        },
      },
    }
  }

  fn has_client(&self, card_number: &str) -> bool {
    match &self.data {
      None => false,
      Some(data) => data.clients.contains_key(card_number),
    }
  }

  fn get_client(&self, card_number: &str) -> Result<Client, String> {
     let client = match &self.data {
      None => None,
      Some(data) => match data.clients.get(card_number) {
        None => None,
        Some(client) => Some(client.clone()),
      },
    };

    match client {
      None => Err(get_error_str(JsonDatabaseError::ClientNotFound)),
      Some(c) => Ok(c),
    }
  }
}

fn get_error_str(err: JsonDatabaseError) -> String {
   match err {
    JsonDatabaseError::CreateDatabaseFile => String::from("creating database file failed"),
    JsonDatabaseError::Serialization => String::from("conversion data to json string failed"),
    JsonDatabaseError::Deserialization => String::from("json string conversion to data failed"),
    JsonDatabaseError::SavingDatabaseFile => String::from("saving database file failed"),
    JsonDatabaseError::ClientNotFound => String::from("client not found in database"),
  }
}

mod json_impl {
  use super::*;

  pub fn data_to_json_str(data: &DatabaseJsonData) -> Result<String, JsonDatabaseError> {
    use serde_json::to_string_pretty;

    match to_string_pretty(&data) {
      Ok(json) => Ok(json),
      Err(_) => return Err(JsonDatabaseError::Serialization),
    }
  }

  pub fn data_from_json(json: &str) -> Result<DatabaseJsonData, JsonDatabaseError> {
    use serde_json::from_str;

    let data: DatabaseJsonData = match from_str(json) {
      Ok(d) => d,
      Err(_) => return Err(JsonDatabaseError::Deserialization),
    };

    Ok(data)
  }
}

mod fs_impl {
  use std::fs::OpenOptions;
  use std::io::prelude::*;

  const FILE_NAME: &str = "data.json";

  pub fn read_json_file() -> std::io::Result<String> {
    std::fs::read_to_string(FILE_NAME)
  }

  pub fn write_json_to_file(json: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(FILE_NAME)?;

    file.write_all(json.as_bytes())?;

    Ok(())
  }
}

pub struct SqliteDb {}

impl Database for SqliteDb {
  fn name(&self) -> &str {
    "sqlite"
  }

  fn save_client(&mut self, _client: Client) -> Result<(), String> {
    panic!("Not implemented!");
  }

  fn has_client(&self, _card_number: &str) -> bool {
    panic!("Not implemented!");
  }

  fn get_client(&self, _card_number: &str) -> Result<Client, String> {
    panic!("Not implemented!");
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub fn get_mock_json_db() -> JsonDb {
    let mut json_db = JsonDb {
      data: None,
      read_json_file: read_file_mock,
      write_json_to_file: |_| { Ok(()) },
    };

    let result = json_db.read_data();
    let read_ok = matches!(result, Ok(()));
    assert_eq!(read_ok, true);

    json_db
  }

  #[test]
  fn should_return_json_db_name() {
    let mut json_db = JsonDb {
      data: None,
      read_json_file: read_file_mock,
      write_json_to_file: |_| { Ok(()) },
    };
    let result = json_db.read_data();
    let read_ok = matches!(result, Ok(()));

    assert_eq!(read_ok, true);
    assert_eq!("json", json_db.name());
  }

  #[test]
  fn should_save_client_to_json() {
    let card_number = String::from("4000000000000000");
    let pin = String::from("1234");
    let balance = 0;

    let mut json_db = get_mock_json_db();
    json_db.write_json_to_file = |json| {
      let card_number = String::from("4000000000000000");
      let pin = String::from("1234");
      let balance = 0;

      let data: DatabaseJsonData = serde_json::from_str(json).unwrap();
      let client = data.clients.get(&card_number).unwrap();

      assert_eq!(client.card_number, card_number);
      assert_eq!(client.pin, pin);
      assert_eq!(client.balance, balance);

      Ok(())
    };

    let result = json_db.save_client(
      Client {
        card_number,
        pin,
        balance,
      }
    );

    assert_eq!(result, Ok(()));
  }

  fn read_file_mock() -> std::io::Result<String> {
    Ok(String::from("{\"clients\":{}}"))
  }
}
