use crate::menu::{MenuAction, Cmd};
use crate::database::{Database, Client};

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
    let card_number = generate_card_number();
    let pin = generate_pin();

    let new_client = Client {
      card_number: card_number.clone(),
      pin: pin.clone(),
      balance: 0,
    };

    match db.save_client(new_client) {
      Ok(_) => {
        println!("New client created");
        println!("card_number: {}", card_number);
        println!("pin: {}", pin);
      },
      Err(error) => {
        println!("\ncreating client account failed: {:?}", error);
      }
    };

    MenuAction::Render
  }
}

fn generate_card_number() -> String {
  let mut card_number = String::from("400000");
    let digits =  DIGITS;
    let mut rng = thread_rng();
    for _ in 0..10 {
      let num = digits.chars().choose(&mut rng).unwrap();
      card_number.push(num);
    }

    card_number
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
  fn should_exec_create_account_cmd() {
    let create_account_cmd = CreateAccountCmd::new();
    let mut json_db = crate::database::tests::get_mock_json_db();

    assert_eq!(json_db.get_data().clients.len(), 0);

    let menu_action = create_account_cmd.exec(&mut json_db);

    let matches = matches!(menu_action, MenuAction::Render);
    assert_eq!(matches, true);
    assert_eq!(json_db.get_data().clients.len(), 1);
  }
}
