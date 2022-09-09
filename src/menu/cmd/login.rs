use crate::CloseApp;
use crate::menu::Cmd;
use crate::Database;

pub struct  LoginCmd {
}

impl LoginCmd {
  pub fn new() -> Self {
    LoginCmd {}
  }
}

impl Cmd for LoginCmd {
  fn name(&self) -> &str {
    "Login"
  }

  fn exec(&self, _db: &mut Box<dyn Database>) -> CloseApp {
    println!("Executing: {}", self.name());

    CloseApp::No
  }
}
