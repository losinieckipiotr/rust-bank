use crate::CloseApp;
use crate::menu::Cmd;
use crate::Database;

pub struct  LoginCmd {
}

impl LoginCmd {
  pub fn new() -> Self {
    LoginCmd {}
  }
}

impl Cmd for LoginCmd {
  fn name(&self) -> &str {
    "Login"
  }

  fn exec(&self, db: &mut Box<dyn Database>) -> CloseApp {
    println!("Enter login:");

    // TODO remove unwrap
    let mut login_buf = String::new();
    std::io::stdin().read_line(&mut login_buf).unwrap();
    let login = login_buf.trim_end();

    println!("Enter PIN:");

    let mut pin_buf = String::new();
    std::io::stdin().read_line(&mut pin_buf).unwrap();
    let pin = pin_buf.trim_end();

    if!db.has_client(&login) {
      println!("Invalid login or PIN");
    }

    let client_option = db.get_client(&login);

    if let Ok(client) = client_option {
      if client.pin != pin {
        println!("Invalid login or PIN");
      } else {
        println!("Login successful");
        println!("logged in on client: {:?}", client);
      }
    }

    CloseApp::No
  }
}
