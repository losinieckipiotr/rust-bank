mod print;
mod cmd;
mod main_menu;
mod login_menu;

pub use main_menu::MainMenu;
pub use login_menu::LoginMenu;
pub use cmd::Cmd;

use crate::Database;
use crate::menu::print::print_separator;

pub enum CloseApp {
  Yes,
  No
}

pub struct MenuData {
  header: String,
  commands: Vec<Box< dyn Cmd>>,
}

impl MenuData {
  pub fn start(&mut self, db: &mut dyn Database) {
    loop {
      if let CloseApp::Yes = self.render(db) {
        break;
      }
    }
  }

  fn render(&mut self, db: &mut dyn Database) -> CloseApp {
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

fn unknown_command() -> CloseApp {
  print_separator();
  println!("|Unknown command|");

  CloseApp::No
}
