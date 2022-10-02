use crate::menu::cmd::*;
use crate::menu::Menu;

pub struct LoginMenu;

impl LoginMenu {
  pub fn new(card_number: String) -> Menu {
    Menu {
      header: String::from("Login menu"),
      commands: vec![
        BalanceCmd::new(card_number.clone()).into(),
        AddIncomeCmd::new(card_number.clone()).into(),
        DoTransferCmd::new(card_number.clone()).into(),
        CloseAccountCmd::new(card_number.clone()).into(),
        CloseCmd::new().into(),
        ExitCmd::new().into(),
      ]
    }
  }
}
