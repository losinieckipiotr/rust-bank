mod print;
mod close_app;
mod cmd;

use print::print_separator;
use cmd::*;
use crate::Database;
pub use close_app::*;

pub trait Menu {
  fn render(&mut self) -> CloseApp;
}

pub struct MainMenu {
  db: Box<dyn Database>,
  commands: Vec<Box< dyn Cmd>>,
}

impl MainMenu {
  pub fn new(db: Box<dyn Database>) -> Self {
    MainMenu {
      db,
      commands: vec![
        Box::new(CreateAccountCmd::new()),
        Box::new(LoginCmd::new()),
        Box::new(ExitCmd::new()),
      ],
    }
  }
}

impl Menu for MainMenu {
  fn render(&mut self) -> CloseApp {
    print_separator();
    println!("{}:", "Main menu");

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
      Some(cmd) =>  cmd.exec(&mut self.db),
      None => unknown_command(),
    }
  }
}

fn unknown_command() -> CloseApp {
  print_separator();
  println!("|Unknown command|");

  CloseApp::No
}
