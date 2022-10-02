use crate::Database;
use crate::Client;
use crate::DatabaseResult;
// use crate::DatabaseData;
use crate::DatabaseError;

use rusqlite::{Connection, params};

use error_stack::{Context, Report, ResultExt};

use std::fmt;

// pub type SQLiteDataBaseResult<T> = Result<T, SQLiteDatabaseError>;

// TODO error handling, tests

#[derive(Debug)]
pub struct SQLiteDatabaseError;

impl fmt::Display for SQLiteDatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "sqlite operation failed")
  }
}

impl Context for SQLiteDatabaseError {}

pub struct SQLiteDb;

impl SQLiteDb {
  pub fn new() -> Self {
    SQLiteDb::create_table();

    SQLiteDb
  }

  fn create_table() {
    let conn = Connection::open("clients.db").unwrap();
    conn.execute(
      "
        CREATE TABLE IF NOT EXISTS clients(
          id INTEGER PRIMARY KEY,
          cardNumber TEXT UNIQUE,
          pin TEXT,
          balance INTEGER
        )
      ",
      []
    ).unwrap();
  }

  fn insert_client(client: &Client, conn: &Connection) {
    conn.execute(
      "
        INSERT INTO clients(cardNumber, pin, balance)
        VALUES(?1, ?2, ?3)
      ",
      params![
        client.card_number,
        client.pin,
        client.balance
      ]
    ).unwrap();
  }

  fn update_balance(client: &Client, conn: &Connection) {
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
    // TODO check if already exists
    let conn = Connection::open("clients.db").unwrap();
    SQLiteDb::insert_client(&client, &conn);

    Ok(())
  }

  fn has_client(&self, card_number: &str) -> bool {
    let conn = Connection::open("clients.db").unwrap();
    let mut stmt = conn.prepare(
      "
        SELECT * FROM clients
        WHERE cardNumber = ?
      "
    ).unwrap();
    stmt.exists([&card_number]).unwrap()
  }

  fn get_client(&self, card_number: &str) -> DatabaseResult<Client> {
    let conn = Connection::open("clients.db").unwrap();

    let mut stmt = conn.prepare(
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

    let conn = Connection::open("clients.db").unwrap();
    conn.execute(
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

    let conn = Connection::open("clients.db").unwrap();
    conn.execute(
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
        .change_context(DatabaseError::JSON)
    }

    receiver_client.balance += funds as i32;

    let mut conn = Connection::open("clients.db").unwrap();
    let tx = conn.transaction().unwrap();

    SQLiteDb::update_balance(&sender_client, &tx);
    SQLiteDb::update_balance(&receiver_client, &tx);

    tx.commit().unwrap();

    Ok(())
  }

  fn get_clients_count(&self) -> u32 {
    let conn = Connection::open("clients.db").unwrap();
    let count: u32 = conn
      .prepare(
        "
          SELECT COUNT(*)
          FROM clients
        "
      )
      .unwrap()
      .query_row([], |row| {
        row.get(0)
      })
      .unwrap();

    count
  }
}
