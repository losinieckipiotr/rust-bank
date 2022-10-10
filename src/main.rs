mod database;
mod menu;
mod cmd;
mod luhn;

use database::*;
use menu::Menu;

use clap::{Parser, ValueEnum};

// Simple sort of banking program
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  /// Type of database
  #[clap(default_value_t = DataBaseType::JSON, arg_enum, value_parser)]
  database: DataBaseType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DataBaseType {
  JSON,
  SQLITE,
}

fn main() {
  let Cli { database } = Cli::parse();
  let mut db = db_factory(database);
  let mut main_menu = Menu::new();

  main_menu.start(db.as_mut());
}

fn db_factory(database: DataBaseType) -> Box<dyn Database> {
  match database {
    DataBaseType::JSON => Box::new(JsonDb::new()),
    DataBaseType::SQLITE => Box::new(SQLiteDb::new()),
  }
}
