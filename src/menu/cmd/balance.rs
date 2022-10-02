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
  fn should_exec_balance_cmd() {
    let mut json_db = crate::database::tests::get_mock_json_db();
    let mock_client = crate::database::tests::get_mock_client();
    let balance_cmd = BalanceCmd::new(mock_client.card_number.clone());
    json_db.save_new_client(mock_client).unwrap();

    let menu_action = balance_cmd.exec(&mut json_db);

    let matches = matches!(menu_action, MenuAction::Render);
    assert_eq!(matches, true);
  }
}
