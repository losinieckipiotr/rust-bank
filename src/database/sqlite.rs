use crate::Database;
use crate::Client;
use crate::DatabaseResult;
// use crate::DatabaseData;
use crate::DatabaseError;

use rusqlite::params;

use error_stack::{Context, Result, IntoReport, Report, ResultExt};

use std::fmt;

pub type SQLiteDataBaseResult<T> = Result<T, SQLiteDatabaseError>;

#[derive(Debug)]
pub enum SQLiteDatabaseError {
  QueryFailed,
  PrepareQueryFailed,
  ClientAlreadyExists(Client),
}

impl fmt::Display for SQLiteDatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::QueryFailed => write!(f, "sqlite query failed"),
      Self::PrepareQueryFailed => write!(f, "prepare sqlite query failed"),
      Self::ClientAlreadyExists(client) => write!(f, "client already exists in database, {client:?}"),
    }
  }
}

impl Context for SQLiteDatabaseError {}

pub struct SQLiteDb {
  connection: rusqlite::Connection,
}

fn get_connection_impl() -> rusqlite::Connection {
  rusqlite::Connection::open("clients.db").unwrap()
}

impl SQLiteDb {
  pub fn new() -> Self {
    let db = SQLiteDb {
      connection: get_connection_impl()
    };

    if let Err(error) = db.create_clients_table() {
      println!("\n failed to create clients table, error: {:?}", error);
      panic!("SQLiteDb::new() failed");
    }

    db
  }

  fn create_clients_table(&self) -> SQLiteDataBaseResult<()> {
    self.connection.execute(
      "
        CREATE TABLE IF NOT EXISTS clients(
          id INTEGER PRIMARY KEY,
          cardNumber TEXT UNIQUE,
          pin TEXT,
          balance INTEGER
        )
      ",
      []
    )
      .report()
      .attach_printable(
        format!("failed to execute CREATE TABLE query")
      )
      .change_context(SQLiteDatabaseError::QueryFailed)?;

      Ok(())

  }

  fn insert_client(&self, client: &Client) -> SQLiteDataBaseResult<()> {
    self.connection.execute(
      "
        INSERT INTO clients(cardNumber, pin, balance)
        VALUES(?1, ?2, ?3)
      ",
      params![
        client.card_number,
        client.pin,
        client.balance
      ]
    )
    .report()
    .attach_printable_lazy(|| {
      format!("failed to execute INSERT query for {client:?}")
    })
    .change_context(SQLiteDatabaseError::QueryFailed)?;

    Ok(())
  }

  fn update_client_balance(client: &Client, conn: &rusqlite::Connection) {
    conn.execute(
      "
        UPDATE clients
        SET balance = ?1
        WHERE cardNumber = ?2
      ",
      params![
        client.balance,
        client.card_number
      ]
    ).unwrap();
  }
}

impl Database for SQLiteDb {
  fn name(&self) -> &str {
    "sqlite"
  }

  fn save_new_client(&mut self, client: Client) -> DatabaseResult<()> {
    let has_client = self.has_client(&client.card_number)?;

    if has_client {
      return Err(
        Report::new(
          SQLiteDatabaseError::ClientAlreadyExists(client)
        )
          .change_context(DatabaseError::SQLite)
      );
    }

    self.insert_client(&client)
      .attach_printable_lazy(|| {
        format!("failed to insert client to database")
      })
      .change_context(DatabaseError::SQLite)?;

    Ok(())
  }

  fn has_client(&self, card_number: &str) -> DatabaseResult<bool> {
    let mut stmt = self.connection.prepare(
      "
        SELECT * FROM clients
        WHERE cardNumber = ?
      "
    )
      .report()
      .change_context(SQLiteDatabaseError::PrepareQueryFailed)
      .change_context(DatabaseError::SQLite)?;

    match stmt.exists([&card_number]) {
      Err(error) => Err(error)
        .report()
        .change_context(SQLiteDatabaseError::QueryFailed)
        .attach_printable_lazy( || {
          format!(
            "failed to check if client with card_number: {} exists",
            card_number
          )
        })
        .change_context(DatabaseError::SQLite),
      Ok(exists) => Ok(exists),
    }
  }

  fn get_client(&self, card_number: &str) -> DatabaseResult<Client> {
    let mut stmt = self.connection.prepare(
      "
        SELECT cardNumber, pin, balance
        FROM clients
        WHERE cardNumber = ?
      "
    ).unwrap();
    Ok(stmt.query_row([&card_number], |row| {
      Ok(Client {
        card_number: row.get(0)?,
        pin: row.get(1)?,
        balance: row.get(2)?
      })
    }).unwrap())
  }

  fn remove_client(&mut self, card_number: &str) -> DatabaseResult<Client> {
    let client = self.get_client(card_number).unwrap();

    self.connection.execute(
      "
        DELETE FROM clients
        WHERE cardNumber = ?
      ",
      [&card_number]
    ).unwrap();

    Ok(client)
  }

  fn add_funds(&mut self, funds: u32, card_number: &str) -> DatabaseResult<()> {
    let mut client = self.get_client(card_number)?;

    client.balance += funds as i32;

    self.connection.execute(
      "
        UPDATE clients
        SET balance = ?1
        WHERE cardNumber = ?2
      ",
      params![
        client.balance,
        client.card_number,
      ]
    ).unwrap();

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
      return Err(Report::new(DatabaseError::SQLite))
        .attach_printable_lazy(|| {
          format!(
            "sender's balance before transfer: {}, after transfer: {}",
            sender_original_balance,
            sender_client.balance
          )
        })
    }

    receiver_client.balance += funds as i32;

    let tx = self.connection.transaction().unwrap();

    SQLiteDb::update_client_balance(&sender_client, &tx);
    SQLiteDb::update_client_balance(&receiver_client, &tx);

    tx.commit().unwrap();

    Ok(())
  }

  fn get_clients_count(&self) -> u32 {
    let count: u32 = self.connection.prepare(
      "
        SELECT COUNT(*)
        FROM clients
      "
    )
    .unwrap()
    .query_row([], |row| { row.get(0) })
    .unwrap();

    count
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub fn get_mock_db() -> SQLiteDb {
    let sqlite_db = SQLiteDb {
      connection: get_mock_connection(),
    };

    sqlite_db.create_clients_table().unwrap();

    sqlite_db
  }

  #[test]
  fn should_save_client_to_clients_table() {
    let client_mock = crate::database::tests::get_mock_client();
    let mut sql_db = get_mock_db();

    assert_eq!(sql_db.get_clients_count(), 0);

    sql_db.save_new_client(client_mock.clone()).unwrap();

    assert_eq!(sql_db.get_clients_count(), 1);
    assert_eq!(
      client_mock,
      sql_db.get_client(&client_mock.card_number.clone()).unwrap()
    );
  }

  fn get_mock_connection() -> rusqlite::Connection {
    rusqlite::Connection::open_in_memory().unwrap()
  }
}

