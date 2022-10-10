use error_stack::{Context, IntoReport, Result, ResultExt};

use std::fmt;

#[derive(Debug)]
pub struct CommandLineError;

type CommandLineResult<T> = Result<T, CommandLineError>;

impl fmt::Display for CommandLineError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "invalid login or PIN")
  }
}

impl Context for CommandLineError {}

pub fn read_with_prompt(prompt: &str) -> CommandLineResult<String> {
  println!("{}", prompt);

  let mut buf = String::new();
  std::io::stdin()
    .read_line(&mut buf)
    .report()
    .attach_printable(
      format!("failed to read from command line, prompt: {prompt}")
    )
    .change_context(CommandLineError)?;

  let login = buf.trim_end();

  Ok(String::from(login))
}

pub fn read_from_cmd() -> CommandLineResult<String> {
  let mut buf = String::new();
  std::io::stdin()
    .read_line(&mut buf)
    .report()
    .change_context(CommandLineError)?;

  let login = buf.trim_end();

  Ok(String::from(login))
}
