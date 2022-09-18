use crate::menu::Cmd;
use crate::Database;
use crate::menu::MenuAction;

use error_stack::{Context, IntoReport, Report, Result, ResultExt};

use std::fmt;

#[derive(Debug)]
pub struct LoginError;

type LoginResult<T> = Result<T, LoginError>;

impl fmt::Display for LoginError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "conversion data to json string failed")
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
}

impl Cmd for LoginCmd {
  fn name(&self) -> &str {
    "Login"
  }

  fn exec(&self, db: &mut dyn Database) -> MenuAction {
    let read_from_cmd = self.read_from_cmd.as_ref();

    let login = match read_from_cmd("Enter login:") {
      Err(error) => {
        print_error(error);

        return MenuAction::Render;
      },
      Ok(login) => login,
    };

    let pin = match read_from_cmd("Enter PIN:") {
      Err(error) => {
        print_error(error);
        return MenuAction::Render;
      },
      Ok(pin) => pin,
    };

    if !db.has_client(&login) {
      println!("Invalid login or PIN");

      return MenuAction::Render;
    }

    let client = match db.get_client(&login) {
      Err(error) => {
        print_error(
          error
          .attach_printable("failed to ged client from database")
          .change_context(LoginError)
        );

        return MenuAction::Render;
      },
      Ok(client) => client
    };

    if client.pin != pin {
      println!("Invalid login or PIN");

      return MenuAction::Render;
    }

    println!("Login successful");
    println!("logged in on client: {:?}", client);

    MenuAction::RenderLoginMenu(client.card_number)
  }
}

fn print_error(error: Report<LoginError>) {
  println!("Login failed: {:?}", error);
}

mod cmd_impl {
  use super::*;

  pub fn read(prompt: &str) -> LoginResult<String> {
    println!("{}", prompt);

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)
    .report()
    .attach_printable(format!("failed to read from console, prompt: {prompt:?}"))
    .change_context(LoginError)?;

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
          "Enter login:" => Ok(client.card_number.clone()),
          "Enter PIN:" => Ok(client.pin.clone()),
          _ => panic!("prompt changed ?"),
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
