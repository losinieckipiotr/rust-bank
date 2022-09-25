use crate::menu::{MenuAction, Cmd};
use crate::Database;

use error_stack::{Context, IntoReport, Result, ResultExt};

use std::fmt;

#[derive(Debug)]
pub struct DoTransferError;

impl fmt::Display for DoTransferError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "transfer failed")
  }
}

impl Context for DoTransferError {}

type DoTransferResult<T> = Result<T, DoTransferError>;

pub struct  DoTransferCmd {
  card_number: String,
  read_from_cmd: Box<dyn Fn(&str) -> DoTransferResult<String>>,
}

const RECEIVER_CARD_PROMPT: &str = "Enter receiver card number:";
const AMOUNT_PROMPT: &str = "Enter amount:";

impl DoTransferCmd {
  pub fn new(card_number: String) -> Self {
    DoTransferCmd {
      card_number,
      read_from_cmd: Box::new(cmd_impl::read),
    }
  }

  fn do_transfer_impl(&self, db: &mut dyn Database) -> DoTransferResult<u32> {
    let read_from_cmd = &self.read_from_cmd;

    let receiver_card_number = read_from_cmd(RECEIVER_CARD_PROMPT)?;
    let amount_str = read_from_cmd(AMOUNT_PROMPT)?;

    let amount = amount_str.parse::<u32>()
      .report()
      .attach_printable_lazy(|| {
        format!("failed to parse transfer amount, amount_str: \"{}\"", amount_str)
      })
      .change_context(DoTransferError)?;

    db.transfer_funds(amount, &self.card_number, &receiver_card_number)
      .attach_printable_lazy(|| {
        format!(
          "transfer funds failed, amount: {} sender_card_number: {} receiver_card_number: {}",
          amount,
          self.card_number,
          receiver_card_number
        )
      })
      .change_context(DoTransferError)?;

    Ok(amount)
  }
}

impl Cmd for DoTransferCmd {
  fn name(&self) -> &str {
    "Do transfer"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    match self.do_transfer_impl(db) {
      Err(error) => {
        println!("\nerror: {error:?}");
      },
      Ok(amount) => {
        println!("transferred: {}", amount);
      }
    }

    MenuAction::Render
  }
}

mod cmd_impl {
  use super::*;

  pub fn read(prompt: &str) -> DoTransferResult<String> {
    println!("{}", prompt);

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)
    .report()
    .attach_printable(format!("{prompt}"))
    .change_context(DoTransferError)?;

    let login = buf.trim_end();

    Ok(String::from(login))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_exec_do_transfer_cmd() {
    let mut json_db = crate::database::tests::get_mock_json_db();

    let mock_client1 = crate::database::tests::get_mock_client();
    let mut mock_client2 = crate::database::tests::get_mock_client();
    mock_client2.card_number = String::from("4000000000000001");
    mock_client2.balance = 5000;


    let sender_card_number = mock_client2.card_number.clone();
    let receiver_card_number = mock_client1.card_number.clone();
    let one_thousand = String::from("1000");

    let do_transfer_cmd = {
      use std::rc::Rc;

      let receiver_mock = Rc::new(receiver_card_number.clone());
      let amount_mock = Rc::new(one_thousand);
      DoTransferCmd {
        card_number: sender_card_number.clone(),
        read_from_cmd: Box::new(move |prompt| {
          match prompt {
            RECEIVER_CARD_PROMPT => Ok(receiver_mock.as_ref().clone()),
            AMOUNT_PROMPT => Ok(amount_mock.as_ref().clone()),
            _prompt => panic!("unknown prompt: {_prompt}"),
          }
        }),
      }
    };
    let clients = [mock_client1, mock_client2];
    json_db.save_clients(&clients).expect("successfully saved mock clients");

    let menu_action = do_transfer_cmd.exec(&mut json_db);

    let matches = matches!(menu_action, MenuAction::Render);
    assert_eq!(matches, true);

    let sender_client = json_db.get_client(&sender_card_number).unwrap();
    let receiver_client = json_db.get_client(&receiver_card_number).unwrap();

    assert_eq!(sender_client.balance.to_string(), "4000");
    assert_eq!(receiver_client.balance.to_string(), "1000");
  }
}
