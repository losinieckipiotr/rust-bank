mod database;
mod menu;

use clap::Parser;

use database::*;
use menu::MainMenu;

// Simple program to benchmark sort algorithms
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  /// Type of database
  #[clap(default_value_t = DataBaseType::JSON, arg_enum, value_parser)]
  database: DataBaseType,
}

fn main() {
  let Cli { database } = Cli::parse();
  let mut main_menu = MainMenu::new();

  main_menu.start(database);
}
