mod exit;
mod create_account;
mod login;
mod balance;

pub use exit::ExitCmd;
pub use create_account::CreateAccountCmd;
pub use login::LoginCmd;
pub use balance::Balance;

use crate::Database;
use crate::menu::MenuAction;

pub trait Cmd {
  fn name(&self) -> &str;
  fn exec(&self, db: &mut dyn Database) -> MenuAction;
}
