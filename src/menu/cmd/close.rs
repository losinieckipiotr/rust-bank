use crate::menu::{MenuAction, Cmd};
use crate::Database;

pub struct  CloseCmd;

impl CloseCmd {
  pub fn new() -> Self {
    CloseCmd {}
  }
}

impl Cmd for CloseCmd {
  fn name(&self) -> &str {
    "Close"
  }

  fn exec(&self, _db: &mut dyn Database) -> MenuAction {
    MenuAction::Close
  }
}
