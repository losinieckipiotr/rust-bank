use crate::menu::{CloseApp, Cmd};
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

  fn exec(&self, _db: &mut dyn Database) -> CloseApp {
    CloseApp::Yes
  }
}
