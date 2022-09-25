use crate::menu::Cmd;
use crate::{Database, Client};
use crate::menu::MenuAction;

use error_stack::{Context, IntoReport, Report, Result, ResultExt};

use std::fmt;

#[derive(Debug)]
pub enum LoginError {
  InvalidLoginOrPin,
  GettingClientFailed,
  ReadFromConsoleFailed,
}

type LoginResult<T> = Result<T, LoginError>;

impl fmt::Display for LoginError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      LoginError::InvalidLoginOrPin => write!(f, "invalid login or PIN"),
      LoginError::GettingClientFailed => write!(f, "failed to get client from database"),
      LoginError::ReadFromConsoleFailed => write!(f, "failed to read from console"),
    }
  }
}

impl Context for LoginError {}

pub struct  LoginCmd {
  read_from_cmd: Box<dyn Fn(&str) -> LoginResult<String>>,
}

impl LoginCmd {
  pub fn new() -> Self {
    LoginCmd {
      read_from_cmd: Box::new(cmd_impl::read),
    }
  }

  fn login_impl(&self, db: &mut dyn Database) -> LoginResult<Client> {
    let read_from_cmd = self.read_from_cmd.as_ref();

    let login = read_from_cmd(LOGIN_PROMPT)?;
    let pin = read_from_cmd(PIN_PROMPT)?;

    if !db.has_client(&login) {
      return Err(Report::new(LoginError::InvalidLoginOrPin));
    }

    let client = db.get_client(&login)
      .attach_printable(format!("login: {login}"))
      .change_context(LoginError::GettingClientFailed)?;

    if client.pin != pin {
      return Err(Report::new(LoginError::InvalidLoginOrPin));
    }

    Ok(client)
  }
}

const LOGIN_PROMPT: &str = "Enter login:";
const PIN_PROMPT: &str = "Enter PIN:";

impl Cmd for LoginCmd {
  fn name(&self) -> &str {
    "Login"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    match self.login_impl(db) {
      Err(report) => {
        print_report(report);

        MenuAction::Render
      },
      Ok(client) => {
        println!("Login successful");
        println!("logged in on client: {:?}", client);

        MenuAction::RenderLoginMenu(client.card_number)
      },
    }
  }
}

fn print_report(error: Report<LoginError>) {
  println!("\nlogin failed: {error:?}");
}

mod cmd_impl {
  use super::*;

  pub fn read(prompt: &str) -> LoginResult<String> {
    println!("{}", prompt);

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)
    .report()
    .attach_printable(format!("{prompt}"))
    .change_context(LoginError::ReadFromConsoleFailed)?;

    let login = buf.trim_end();

    Ok(String::from(login))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_exec_login_and_return_render_login_action() {
    let mut json_db = crate::database::tests::get_mock_json_db();
    let mock_client = crate::database::tests::get_mock_client();
    json_db.save_client(mock_client.clone()).expect("successfully saved mock client");

    let client = mock_client.clone();
    let login_cmd = LoginCmd {
      read_from_cmd: Box::new(move |prompt| {
        match prompt {
          LOGIN_PROMPT => Ok(client.card_number.clone()),
          PIN_PROMPT => Ok(client.pin.clone()),
          _prompt => panic!("unknown prompt: {_prompt}"),
        }
      })
    };

    let menu_action = login_cmd.exec(&mut json_db);

    let success = match menu_action {
      MenuAction::RenderLoginMenu(card_number) => {
        card_number == mock_client.card_number
      },
      _ => false
    };
    assert_eq!(success, true);
  }
}
