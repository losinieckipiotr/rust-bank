mod cmd;

use crate::menu::cmd::*;
use crate::Database;
use crate::command_line::read_from_cmd;

use error_stack::{Context, Result, ResultExt};

use std::fmt;

#[derive(Debug)]
pub struct MenuError;

type MenuResult<T> = Result<T, MenuError>;

impl fmt::Display for MenuError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "menu operation failed")
  }
}

impl Context for MenuError {}

pub enum MenuAction {
  Exit,
  Close,
  Render,
  RenderLoginMenu(String),
  UnknownCommand
}

pub struct Menu {
  header: String,
  commands: Vec<Box<dyn Cmd>>,
  read_from_cmd: Box<dyn Fn() -> MenuResult<String>>,
}

impl Menu {
  pub fn new() -> Self {
    Menu {
      header: String::from("Main menu"),
      commands: vec![
        CreateAccountCmd::new().into(),
        LoginCmd::new().into(),
        ExitCmd::new().into(),
      ],
      read_from_cmd: Box::new(Menu::prompt_impl),
    }
  }

  fn new_login_menu(card_number: &str) -> Self {
    Menu {
      header: String::from("Login menu"),
      commands: vec![
        BalanceCmd::new(card_number).into(),
        AddIncomeCmd::new(card_number).into(),
        DoTransferCmd::new(card_number).into(),
        CloseAccountCmd::new(card_number).into(),
        CloseCmd::new().into(),
        ExitCmd::new().into(),
      ],
      read_from_cmd: Box::new(Menu::prompt_impl),
    }
  }

  pub fn start(&mut self, db: &mut dyn Database) -> bool {
    loop {
      match self.render(db) {
        MenuAction::Exit => return true,
        MenuAction::Close => return false,
        MenuAction::Render => {},
        MenuAction::RenderLoginMenu(card_number) => {
          let mut login_menu = Menu::new_login_menu(&card_number);

          let exit = login_menu.start(db);

          if exit {
            return  true;
          }
        },
        MenuAction::UnknownCommand => {
          Menu::print_separator();
          println!("|Unknown command|");
        }
      }
    }
  }

  fn render(&mut self, db: &mut dyn Database) -> MenuAction {
    Menu::print_separator();
    println!("{}:", self.header);

    for (i, cmd) in self.commands.iter().enumerate() {
      println!("{} - {}", i, cmd.name());
    }

    let read_from_cmd = self.read_from_cmd.as_ref();
    let line = match read_from_cmd() {
      Err(report) => {
        println!("\n{report:?}");

        return MenuAction::Render;
      },
      Ok(line) => line,
    };

    let number = match i32::from_str_radix(&line, 10) {
      Ok(n) => n,
      _ => return MenuAction::UnknownCommand,
    };

    match self.commands.get(number as usize) {
      Some(cmd) =>  cmd.exec(db),
      None => MenuAction::UnknownCommand,
    }
  }

  fn prompt_impl() -> MenuResult<String> {
    read_from_cmd()
      .change_context(MenuError)
  }

  fn print_separator() {
    println!("------------------------------------------");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod json_tests {
    use super::*;
    use crate::database::json::tests::get_mock_db;

    // TODO make a macro ?

    #[test]
    fn should_exit_with_true_json() {
      exit_with_true(get_mock_db());
    }

    #[test]
    fn should_close_with_false_json() {
      close_with_false(get_mock_db());
    }

    #[test]
    fn should_create_account_then_exit_json() {
      create_account_then_exit(get_mock_db())
    }
  }

  mod sqlite_tests {
    use super::*;
    use crate::database::sqlite::tests::get_mock_db;

    #[test]
    fn should_exit_with_true_sqlite() {
      exit_with_true(get_mock_db());
    }

    #[test]
    fn should_close_with_false_sqlite() {
      close_with_false(get_mock_db());
    }

    #[test]
    fn should_create_account_then_exit_sqlite() {
      create_account_then_exit(get_mock_db())
    }
  }

  fn exit_with_true(mut db: impl Database) {
    let mut menu = Menu {
      header: String::from("Test menu"),
      commands: vec![
        ExitCmd::new().into(),
      ],
      read_from_cmd: Box::new(|| {
        Ok(0.to_string())
      }),
    };

    let result = menu.start(&mut db);

    assert_eq!(result, true);
  }

  fn close_with_false(mut db: impl Database) {
    let mut menu = Menu {
      header: String::from("Test menu"),
      commands: vec![
        CloseCmd::new().into(),
      ],
      read_from_cmd: Box::new(|| {
        Ok(0.to_string())
      }),
    };

    let result = menu.start(&mut db);

    assert_eq!(result, false);
  }

  fn create_account_then_exit(mut db: impl Database) {
    use std::cell::RefCell;

    let menu_read_from_cmd_ctr = RefCell::new(0);

    let mut menu = Menu {
      header: String::from("Test menu"),
      commands: vec![
        CreateAccountCmd::new().into(), // 0
        ExitCmd::new().into(), // 1
      ],
      read_from_cmd: Box::new(move || {
        let ctr = *menu_read_from_cmd_ctr.borrow();
        menu_read_from_cmd_ctr.replace(ctr + 1);

        match ctr {
          0 => Ok(0.to_string()), // CreateAccountCmd
          1 => Ok(1.to_string()), // ExitCmd
          _ => panic!("read_from_cmd called to many times"),
        }
      }),
    };

    let result = menu.start(&mut db);
    assert_eq!(result, true);
  }
}
