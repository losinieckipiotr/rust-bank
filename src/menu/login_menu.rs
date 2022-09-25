use crate::menu::cmd::*;
use crate::menu::Menu;

pub struct LoginMenu;

impl LoginMenu {
  pub fn new(card_number: String) -> Menu {
    Menu {
      header: String::from("Login menu"),
      commands: vec![
        Box::new(BalanceCmd::new(card_number.clone())),
        Box::new(AddIncomeCmd::new(card_number.clone())),
        Box::new(DoTransferCmd::new(card_number.clone())),
        Box::new(CloseAccountCmd::new(card_number.clone())),
        Box::new(CloseCmd::new()),
        Box::new(ExitCmd::new()),
      ]
    }
  }
}
