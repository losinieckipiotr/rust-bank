use crate::menu::cmd::{CreateAccountCmd, LoginCmd, ExitCmd};
use crate::menu::Menu;

pub struct MainMenu;

impl MainMenu {
  pub fn new() -> Menu {
    Menu {
      header: String::from("Main menu"),
      commands: vec![
        Box::new(CreateAccountCmd::new()),
        Box::new(LoginCmd::new()),
        Box::new(ExitCmd::new()),
      ],
    }
  }
}
