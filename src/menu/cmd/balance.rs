use crate::menu::{MenuAction, Cmd};
use crate::Database;

pub struct  BalanceCmd {
  card_number: String,
}

impl BalanceCmd {
  pub fn new(card_number: String) -> Self {
    BalanceCmd {
      card_number,
    }
  }
}

impl Cmd for BalanceCmd {
  fn name(&self) -> &str {
    "Balance"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    match db.get_client(&self.card_number) {
      Err(error) => {
        println!("\nfailed to get client data, error:{error:?}");
      },
      Ok(client) => {
        println!("Your balance: {}", client.balance);
      }
    }

    MenuAction::Render
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_exec_balance_cmd_json() {
    exec_balance_cmd(crate::database::json::tests::get_mock_db());
  }

  #[test]
  fn should_exec_balance_cmd_sqlite() {
    exec_balance_cmd(crate::database::sqlite::tests::get_mock_db());
  }


  fn exec_balance_cmd(mut db: impl Database) {
    let mock_client = crate::database::tests::get_mock_client();
    let balance_cmd = BalanceCmd::new(mock_client.card_number.clone());
    db.save_new_client(mock_client).unwrap();

    let menu_action = balance_cmd.exec(&mut db);

    let matches = matches!(menu_action, MenuAction::Render);
    assert_eq!(matches, true);
  }
}
