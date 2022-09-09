mod exit;
mod create_account;
mod login;

pub use exit::ExitCmd;
pub use create_account::CreateAccountCmd;
pub use login::LoginCmd;

use crate::Database;
use crate::CloseApp;

pub trait Cmd {
  fn name(&self) -> &str;
  fn exec(&self, db: &mut Box<dyn Database>) -> CloseApp;
}
