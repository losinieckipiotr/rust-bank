use crate::CloseApp;
use crate::menu::Cmd;
use crate::Database;

pub struct  ExitCmd {
}

impl ExitCmd {
  pub fn new() -> Self {
    ExitCmd {}
  }
}

impl Cmd for ExitCmd {
  fn name(&self) -> &str {
    "Exit"
  }

  fn exec(&self, _db: &mut Box<dyn Database>) -> CloseApp {
    CloseApp::Yes
  }
}
