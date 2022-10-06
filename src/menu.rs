mod cmd;
mod main_menu;
mod login_menu;

pub use main_menu::MainMenu;
pub use login_menu::LoginMenu;
pub use cmd::Cmd;

use crate::Database;
use crate::DataBaseType;

use crate::database::*;

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
  pub fn start(&mut self, db_type: DataBaseType) -> bool {
    let mut db = db_factory(db_type);

    loop {
      match self.render(db.as_mut()) {
        MenuAction::Exit => return true,
        MenuAction::Close => return false,
        MenuAction::Render => {},
        MenuAction::RenderLoginMenu(card_number) => {
          let mut login_menu = LoginMenu::new(card_number);

          let exit = login_menu.start(db_type);

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

fn unknown_command() -> MenuAction {
  print_separator();
  println!("|Unknown command|");

  MenuAction::Render
}

