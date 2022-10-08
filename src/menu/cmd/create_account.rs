use crate::menu::{MenuAction, Cmd};
use crate::database::{Database, Client};
use crate::luhn::is_valid_card_number;

use rand::prelude::{thread_rng, IteratorRandom};

const DIGITS: &str = "0123456789";

pub struct CreateAccountCmd;

impl CreateAccountCmd {
  pub fn new() -> Self {
    CreateAccountCmd
  }
}

impl Cmd for CreateAccountCmd {
  fn name(&self) -> &str {
    "Create account"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    let mut card_number = String::new();
    loop {
      card_number = generate_card_number();

      let has_client = match db.has_client(&card_number) {
        Err(error) => {
          println!("\ncreating client account failed: {:?}", error);
          return MenuAction::Render
        },
        Ok(has_client) => has_client,
      };

      if !has_client {
        break;
      }
    }

    let pin = generate_pin();
    let new_client = Client {
      card_number: card_number.clone(),
      pin: pin.clone(),
      balance: 0,
    };

    match db.save_new_client(new_client) {
      Err(error) => {
        println!("\ncreating client account failed: {:?}", error);
      }
      Ok(_) => {
        println!("New client created");
        println!("card_number: {}", card_number);
        println!("pin: {}", pin);
      },
    }

    MenuAction::Render
  }
}

fn generate_card_number() -> String {
  loop {
    let mut card_number = String::from("400000");
    let digits =  DIGITS;
    let mut rng = thread_rng();
    for _ in 0..10 {
      let num = digits.chars().choose(&mut rng).unwrap();
      card_number.push(num);
    }
    if is_valid_card_number(&card_number) {
      return card_number
    }
  }
}

fn generate_pin() -> String {
  let mut pin = String::new();
  let digits =  DIGITS;
  let mut rng = thread_rng();

  for _ in 0..4 {
    let num = digits.chars().choose(&mut rng).unwrap();
    pin.push(num);
  }

  pin
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_exec_create_account_cmd_json() {
    exec_create_account_cmd(crate::database::json::tests::get_mock_db());
  }

  #[test]
  fn should_exec_create_account_cmd_sqlite() {
    exec_create_account_cmd(crate::database::sqlite::tests::get_mock_db());
  }

  fn exec_create_account_cmd(mut db: impl Database) {
    let create_account_cmd = CreateAccountCmd::new();

    assert_eq!(db.get_clients_count().unwrap(), 0);

    let menu_action = create_account_cmd.exec(&mut db);

    let matches = matches!(menu_action, MenuAction::Render);
    assert_eq!(matches, true);
    assert_eq!(db.get_clients_count().unwrap(), 1);
  }
}
