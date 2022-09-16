use crate::menu::{MenuAction, Cmd};
use crate::Database;

pub struct  Balance {
  card_number: String,
}

impl Balance {
  pub fn new(card_number: String) -> Self {
    Balance {
      card_number,
    }
  }
}

impl Cmd for Balance {
  fn name(&self) -> &str {
    "Balance"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    let client = db.get_client(&self.card_number).expect("logged in client data");

    println!("Your balance: {}", client.balance);

    MenuAction::Render
  }
}
