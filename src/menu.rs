mod cmd;
mod main_menu;
mod login_menu;

pub use main_menu::MainMenu;
pub use login_menu::LoginMenu;
pub use cmd::Cmd;

use crate::Database;

pub enum MenuAction {
  Close,
  Render,
  RenderLoginMenu(String),
}

pub struct MenuData {
  header: String,
  commands: Vec<Box<dyn Cmd>>,
}

impl MenuData {
  pub fn start(&mut self, db: &mut dyn Database) {
    loop {
      match self.render(db) {
        MenuAction::Close => break,
        MenuAction::Render => {},
        MenuAction::RenderLoginMenu(card_number) => {
          let mut login_menu = LoginMenu::new(card_number);

          login_menu.start(db)
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

pub fn print_separator() {
  println!("------------------------------------------");
}

fn unknown_command() -> MenuAction {
  print_separator();
  println!("|Unknown command|");

  MenuAction::Render
}
