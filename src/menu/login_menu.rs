use crate::menu::cmd::{ExitCmd, BalanceCmd};
use crate::menu::MenuData;

pub struct LoginMenu;

impl LoginMenu {
  pub fn new(card_number: String) -> MenuData {
    MenuData {
      header: String::from("Login menu"),
      commands: vec![
        Box::new(BalanceCmd::new(card_number)),
        Box::new(ExitCmd::new()),
      ]
    }
  }
}
