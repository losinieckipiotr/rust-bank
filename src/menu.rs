mod cmd;
mod main_menu;
mod login_menu;

pub use main_menu::MainMenu;
pub use login_menu::LoginMenu;
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
  pub fn start(&mut self, db: &mut dyn Database) -> bool {
    loop {
      match self.render(db) {
        MenuAction::Exit => return true,
        MenuAction::Close => return false,
        MenuAction::Render => {},
        MenuAction::RenderLoginMenu(card_number) => {
          let mut login_menu = LoginMenu::new(card_number);

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

pub fn print_separator() {
  println!("------------------------------------------");
}

fn unknown_command() -> MenuAction {
  print_separator();
  println!("|Unknown command|");

  MenuAction::Render
}
