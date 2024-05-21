use auth_service::{authenticate, Credentials};

fn main() {
  let credentials = Credentials {
    username: "username".to_owned(),
    password: "password".to_owned()
  };

  authenticate(credentials);
}