use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, from_str};

use std::fs;
use std::fs::OpenOptions;
// use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;

const FILE_NAME: &str = "data.json";

pub trait Database {
  fn name(&self) -> &str;
  fn save_client(&mut self, client: Client) -> Result<(), String>;
}

pub struct JsonDb {
  data: Option<DatabaseJsonData>,
}

impl JsonDb {
  pub fn new() -> Self {
    JsonDb { data: None }
  }
}

impl Database for JsonDb {
  fn name(&self) -> &str {
    "json"
  }

  fn save_client(&mut self, client: Client) -> Result<(), String> {
    if let None = self.data {
      match get_data() {
        Ok(data) => self.data = Some(data),
        Err(e) => return get_error(e),
      }
    }
    let data = self.data.as_mut().expect("initialized data");

    data.clients.insert(client.card_number.clone(), client);

    match data_to_json_str(&data) {
      Ok(json) => match write_json_to_file(&json) {
        Ok(_) => Ok(()),
        Err(_) => return get_error(JsonDatabaseError::SavingDatabaseFile)
      },
      Err(e) => return get_error(e),
    }
  }
}

fn get_data() -> Result<DatabaseJsonData, JsonDatabaseError> {
  let data: DatabaseJsonData = match fs::read_to_string(FILE_NAME) {
    Ok(str) => match from_str(&str) {
      Ok(d) => d,
      Err(_) => return Err(JsonDatabaseError::Deserialization),
    },
    Err(_) => match create_empty_data_file() {
      Ok(data) => data,
      Err(err) => return Err(err),
    }
  };

  Ok(data)
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

fn create_empty_data_file() -> Result<DatabaseJsonData, JsonDatabaseError> {
  let empty_data = DatabaseJsonData { clients: HashMap::new() };

  let json = data_to_json_str(&empty_data)?;

  if let Err(_) = write_json_to_file(&json) {
    return Err(JsonDatabaseError::CreateDatabaseFile);
  }

  Ok(empty_data)
}

fn data_to_json_str(data: &DatabaseJsonData) -> Result<String, JsonDatabaseError> {
  match to_string_pretty(&data) {
    Ok(json) => Ok(json),
    Err(_) => return Err(JsonDatabaseError::Serialization),
  }
}

fn write_json_to_file(json: &str) -> Result<(), std::io::Error> {
  let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(FILE_NAME)?;

  println!("Saving data:");
  println!("{}", json);

  file.write_all(json.as_bytes())?;

  Ok(())
}

pub enum JsonDatabaseError {
  CreateDatabaseFile,
  Serialization,
  Deserialization,
  SavingDatabaseFile,
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
