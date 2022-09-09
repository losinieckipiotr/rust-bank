use clap::{Parser, ValueEnum};

mod database;
mod menu;

use database::*;
use menu::*;

// Simple program to benchmark sort algorithms
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

  let db = db_factory(database);

  let mut menu = MainMenu::new(db);
  loop {
    if let CloseApp::Yes = menu.render() {
      break;
    }
  }
}

fn db_factory(database: DataBaseType) -> Box<dyn Database> {
  match database {
    DataBaseType::JSON => Box::new(JsonDb::new()),
    DataBaseType::SQLITE => Box::new(SqliteDb {}),
  }
}
