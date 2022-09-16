use crate::menu::cmd::{ExitCmd, Balance};
use crate::menu::MenuData;

pub struct LoginMenu;

impl LoginMenu {
  pub fn new(card_number: String) -> MenuData {
    MenuData {
      header: String::from("Login menu"),
      commands: vec![
        Box::new(Balance::new(card_number)),
        Box::new(ExitCmd::new()),
      ]
    }
  }
}
