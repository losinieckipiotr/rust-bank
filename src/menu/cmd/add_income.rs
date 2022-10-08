use crate::menu::{MenuAction, Cmd};
use crate::Database;
use crate::cmd::read_with_prompt;

use error_stack::{Context, IntoReport, Result, ResultExt};

use std::fmt;

#[derive(Debug)]
pub struct AddIncomeError;

type AddIncomeResult<T> = Result<T, AddIncomeError>;

impl fmt::Display for AddIncomeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "failed to add income")
  }
}

impl Context for AddIncomeError {}

pub struct  AddIncomeCmd {
  card_number: String,
  read_from_cmd: Box<dyn Fn(&str) -> AddIncomeResult<String>>,
}

impl AddIncomeCmd {
  pub fn new(card_number: &str) -> Self {

    AddIncomeCmd {
      card_number: card_number.to_owned(),
      read_from_cmd: Box::new(|prompt: &str| {
        read_with_prompt(prompt)
          .change_context(AddIncomeError)
      }),
    }
  }

  fn get_income(&self) -> AddIncomeResult<u32> {
    let read_from_cmd = self.read_from_cmd.as_ref();

    let income_str = read_from_cmd(INCOME_AMOUNT_PROMPT)?;

    let income = income_str.parse::<u32>()
      .report()
      .attach_printable_lazy(|| {
        format!("invalid amount, parsed value: \"{}\"", income_str)
      })
      .change_context(AddIncomeError)?;

    Ok(income)
  }

  fn add_income_impl(&self, db: &mut dyn Database) -> AddIncomeResult<u32> {
    let income = self.get_income()
      .attach_printable("failed to read from console")?;

    db.add_funds(income, &self.card_number)
    .attach_printable_lazy(|| {
      format!("failed to add funds to db, income: {:?}, card_number: {:?}", income, &self.card_number)
    })
    .change_context(AddIncomeError)?;

    Ok(income)
  }
}

const INCOME_AMOUNT_PROMPT: &str = "Enter income amount:";

impl Cmd for AddIncomeCmd {
  fn name(&self) -> &str {
    "Add income"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    match self.add_income_impl(db) {
      Err(report) => {
        println!("\n{report:?}");
      }
      Ok(income) => {
        println!("Added {} to your account", income);
      },
    };

    MenuAction::Render
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_exec_add_income_cmd_json() {
    exec_add_income(crate::database::json::tests::get_mock_db());
  }

  #[test]
  fn should_exec_add_income_cmd_sqlite() {
    exec_add_income(crate::database::sqlite::tests::get_mock_db());
  }

  fn exec_add_income(mut db: impl Database) {
    let mock_client = crate::database::tests::get_mock_client();

    assert_eq!(mock_client.balance, 0);

    let card_number = mock_client.card_number.clone();
    let one_thousand = String::from("1000");

    let add_income_cmd = {
      use std::rc::Rc;

      let mock_income = Rc::new(one_thousand);
      AddIncomeCmd {
        card_number: card_number.clone(),
        read_from_cmd: Box::new(move |prompt| {
          match prompt {
            INCOME_AMOUNT_PROMPT => Ok(mock_income.as_ref().clone()),
            _prompt => panic!("unknown prompt: {_prompt}"),
          }
        }),
      }
    };
    db.save_new_client(mock_client).unwrap();

    let menu_action = add_income_cmd.exec(&mut db);

    let matches = matches!(menu_action, MenuAction::Render);
    assert_eq!(matches, true);

    let client = db.get_client(&card_number).expect("client with new balance");
    assert_eq!(client.balance.to_string(), "1000")
  }
}
