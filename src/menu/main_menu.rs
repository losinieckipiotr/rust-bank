use crate::menu::cmd::{CreateAccountCmd, LoginCmd, ExitCmd};
use crate::menu::MenuData;

pub struct MainMenu;

impl MainMenu {
  pub fn new() -> MenuData {
    MenuData {
      header: String::from("Main menu"),
      commands: vec![
        Box::new(CreateAccountCmd::new()),
        Box::new(LoginCmd::new()),
        Box::new(ExitCmd::new()),
      ],
    }
  }
}
