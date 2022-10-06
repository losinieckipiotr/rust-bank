mod cmd;

use crate::menu::cmd::*;

pub use cmd::Cmd;

use crate::Database;

pub enum MenuAction {
  Exit,
  Close,
  Render,
  RenderLoginMenu(String),
}

pub struct Menu {
  header: String,
  commands: Vec<Box<dyn Cmd>>,
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
      ]
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
        }
      }
    }
  }

  fn render(&mut self, db: &mut dyn Database) -> MenuAction {
    print_separator();
    println!("{}:", self.header);

    for (i, cmd) in self.commands.iter().enumerate() {
      println!("{} - {}", i, cmd.name());
    }

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let number = match i32::from_str_radix(line.trim_end(), 10) {
      Ok(n) => n,
      _ => return unknown_command(),
    };

    match self.commands.get(number as usize) {
      Some(cmd) =>  cmd.exec(db),
      None => unknown_command(),
    }
  }
}

fn print_separator() {
  println!("------------------------------------------");
}

// make as new menu action ?
fn unknown_command() -> MenuAction {
  print_separator();
  println!("|Unknown command|");

  MenuAction::Render
}
