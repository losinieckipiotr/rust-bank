use crate::menu::Cmd;
use crate::Database;
use crate::menu::MenuAction;

pub struct  LoginCmd {
  read_from_cmd: Box<dyn Fn(&str) -> Result<String, ()>>,
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

    // TODO remove unwrap ?
    let login = read_from_cmd("Enter login:").unwrap();
    let pin = read_from_cmd("Enter PIN:").unwrap();

    if!db.has_client(&login) {
      println!("Invalid login or PIN");
    }

    let client_option = db.get_client(&login);

    if let Ok(client) = client_option {
      if client.pin != pin {
        println!("Invalid login or PIN");
      } else {
        println!("Login successful");
        println!("logged in on client: {:?}", client);

        return  MenuAction::RenderLoginMenu(client.card_number);
      }
    }

    MenuAction::Render
  }
}

mod cmd_impl {
  pub fn read(prompt: &str) -> Result<String, ()> {
    println!("{}", prompt);
    // TODO remove unwrap ?
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let login = buf.trim_end();

    Ok(String::from(login))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_return_login_cmd_name() {
    let create_account_cmd = LoginCmd::new();
    assert_eq!(create_account_cmd.name(), "Login");
  }

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
