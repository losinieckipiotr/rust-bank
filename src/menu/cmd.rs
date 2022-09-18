mod exit;
mod close;
mod create_account;
mod login;
mod balance;
mod add_income;

pub use close::CloseCmd;
pub use exit::ExitCmd;
pub use create_account::CreateAccountCmd;
pub use login::LoginCmd;
pub use balance::BalanceCmd;
pub use add_income::AddIncomeCmd;

use crate::Database;
use crate::menu::MenuAction;

pub trait Cmd {
  fn name(&self) -> &str;
  fn exec(&self, db: &mut dyn Database) -> MenuAction;
}
