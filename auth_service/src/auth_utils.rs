pub mod models;

pub fn login(creds: models::Credentials) {
  // crate is the root namespace, then database and the method name
  crate::database::get_user();
}

fn logout() {
  // log user out...
}