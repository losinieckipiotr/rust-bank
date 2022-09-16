use crate::menu::cmd::{ExitCmd};
use crate::menu::MenuData;

pub struct LoginMenu;

impl LoginMenu {
  pub fn new(_card_number: String) -> MenuData {
    MenuData {
      header: String::from("Login menu"),
      commands: vec![
        Box::new(ExitCmd::new()),
      ]
    }
  }
}
