use crate::menu::{MenuAction, Cmd};
use crate::database::Database;

pub struct CloseAccountCmd {
  card_number: String,
}

impl CloseAccountCmd {
  pub fn new(card_number: String) -> Self {
    CloseAccountCmd {
      card_number
    }
  }
}

impl Cmd for CloseAccountCmd {
  fn name(&self) -> &str {
    "Close account"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    match db.remove_client(&self.card_number) {
      Err(error) => {
        println!("\nclose account failed: {:?}", error);
        MenuAction::Render
      },
      Ok(client) => {
        println!(
          "Client with card_number: {} removed successfully",
          client.card_number
        );
        MenuAction::Close
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_exec_create_account_cmd_json() {
    let mut db = crate::database::json::tests::get_mock_db();

    exec_create_account_cmd(&mut db);
  }

  #[test]
  fn should_exec_create_account_cmd_sqlite() {
    let mut db = crate::database::sqlite::tests::get_mock_db();

    exec_create_account_cmd(&mut db);
  }

  fn exec_create_account_cmd(db: &mut dyn Database) {
    assert_eq!(db.get_clients_count(), 0);

    let mock_client = crate::database::tests::get_mock_client();
    let card_number = mock_client.card_number.clone();

    db.save_new_client(mock_client).unwrap();

    assert_eq!(db.get_clients_count(), 1);

    let close_account_cmd = CloseAccountCmd::new(card_number);

    let menu_action = close_account_cmd.exec(db);

    let matches = matches!(menu_action, MenuAction::Close);
    assert_eq!(matches, true);
    assert_eq!(db.get_clients_count(), 0);
  }
}
