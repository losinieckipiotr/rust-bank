use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, from_str};

use std::collections::HashMap;

pub trait Database {
  fn name(&self) -> &str;
  fn save_client(&mut self, client: Client) -> Result<(), String>;
}

pub struct JsonDb {
  data: Option<DatabaseJsonData>,
  read_json_file: fn() -> std::io::Result<String>,
  write_json_to_file: fn(json: &str) -> Result<(), std::io::Error>,
}

impl JsonDb {
  pub fn new() -> Self {
    JsonDb {
      data: None,
      read_json_file: fs_impl::read_json_file,
      write_json_to_file: fs_impl::write_json_to_file
    }
  }
}

impl JsonDb {
  fn get_data(&self) -> Result<DatabaseJsonData, JsonDatabaseError> {
    let read_json_file = self.read_json_file;
    let data: DatabaseJsonData = match read_json_file() {
      Ok(str) => match from_str(&str) {
        Ok(d) => d,
        Err(_) => return Err(JsonDatabaseError::Deserialization),
      },
      Err(_) => match self.create_empty_data_file() {
        Ok(data) => data,
        Err(err) => return Err(err),
      }
    };

    Ok(data)
  }

  fn create_empty_data_file(&self) -> Result<DatabaseJsonData, JsonDatabaseError> {
    let empty_data = DatabaseJsonData { clients: HashMap::new() };

    let json = data_to_json_str(&empty_data)?;
    let write_json_to_file = self.write_json_to_file;
    if let Err(_) = write_json_to_file(&json) {
      return Err(JsonDatabaseError::CreateDatabaseFile);
    }

    Ok(empty_data)
  }
}

impl Database for JsonDb {
  fn name(&self) -> &str {
    "json"
  }

  fn save_client(&mut self, client: Client) -> Result<(), String> {
    if let None = self.data {
      let data = match self.get_data() {
        Ok(data) => data,
        Err(e) => return get_error(e),
      };
      self.data = Some(data);
    }
    let data = self.data.as_mut().expect("initialized data");

    data.clients.insert(client.card_number.clone(), client);

    let write_json_to_file = self.write_json_to_file;
    match data_to_json_str(&data) {
      Err(e) => return get_error(e),
      Ok(json) => match write_json_to_file(&json) {
        Err(_) => return get_error(JsonDatabaseError::SavingDatabaseFile),
        Ok(_) => Ok(()),
      },
    }
  }
}

fn get_error(err: JsonDatabaseError) -> Result<(), String> {
  let msg = match err {
    // nice user friendly error messages
    JsonDatabaseError::CreateDatabaseFile => String::from("creating database file failed"),
    JsonDatabaseError::Serialization => String::from("conversion data to json string failed"),
    JsonDatabaseError::Deserialization => String::from("json string conversion to data failed"),
    JsonDatabaseError::SavingDatabaseFile => String::from("saving database file failed"),
  };

  Err(msg)
}

fn data_to_json_str(data: &DatabaseJsonData) -> Result<String, JsonDatabaseError> {
  match to_string_pretty(&data) {
    Ok(json) => Ok(json),
    Err(_) => return Err(JsonDatabaseError::Serialization),
  }
}

enum JsonDatabaseError {
  CreateDatabaseFile,
  Serialization,
  Deserialization,
  SavingDatabaseFile,
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
}

#[derive(Serialize, Deserialize)]
pub struct Client {
  pub card_number: String,
  pub pin: String,
  pub balance: i32,
}

#[derive(Serialize, Deserialize)]
struct DatabaseJsonData {
  clients: HashMap<String, Client>,
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub fn get_mock_json_db() -> JsonDb {
    JsonDb {
      data: None,
      read_json_file: || Ok(String::from("{\"clients\":{}}")),
      write_json_to_file: |_| { Ok(()) },
    }
  }

  #[test]
  fn should_return_json_db_name() {
    let json_db = JsonDb {
      data: None,
      read_json_file: read_file_mock,
      write_json_to_file: |_| { Ok(()) },
    };
    assert_eq!("json", json_db.name());
  }

  #[test]
  fn should_save_client_to_json() {
    let card_number = String::from("4000006256474728");
    let pin = String::from("1234");
    let balance = 0;

    let mut json_db = JsonDb {
      data: None,
      read_json_file: read_file_mock,
      write_json_to_file: |json| {
        let card_number = String::from("4000006256474728");
        let pin = String::from("1234");
        let balance = 0;

        let data: DatabaseJsonData = from_str(json).unwrap();
        let client = data.clients.get(&card_number).unwrap();

        assert_eq!(client.card_number, card_number);
        assert_eq!(client.pin, pin);
        assert_eq!(client.balance, balance);
        Ok(())
      },
    };
    let client = Client {
      card_number,
      pin,
      balance,
    };

    let result = json_db.save_client(client);
    assert_eq!(result, Ok(()));
  }

  fn read_file_mock() -> std::io::Result<String> {
    Ok(String::from("{\"clients\":{}}"))
  }
}
