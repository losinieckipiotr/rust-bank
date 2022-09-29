use crate::menu::cmd::{CreateAccountCmd, LoginCmd, ExitCmd};
use crate::menu::Menu;

pub struct MainMenu;

impl MainMenu {
  pub fn new() -> Menu {
    Menu {
      header: String::from("Main menu"),
      commands: vec![
        CreateAccountCmd::new().into(),
        LoginCmd::new().into(),
        ExitCmd::new().into(),
      ],
    }
  }
}
