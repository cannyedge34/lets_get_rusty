#![allow(dead_code, unused_variables)]
use rand::prelude::*;

/*
    ####################### this is the code without modules #######################
*/

/*
    we need to run the next command to generate cargo modules:
        cargo install cargo-modules

    we need to run the next command to see the details:
        cargo-modules structure
*/

// pub struct Credentials {
//     username: String,
//     password: String
// }

// enum Status {
//     Connected,
//     Interrupted
// }

// fn connect_to_database() -> Status {
//     return Status::Connected
// }

// fn login(creds: Credentials) {
//     // authenticate
//     get_user()
// }

// fn get_user() {
//     // get user from database
// }

// fn logout() {
//     // log user out...
// }


/*
    all elements are private by default in rust
    we want to expose the authenticate method to the consumers
    we just need to add pub prefix to make it public
*/

// pub fn authenticate(creds: Credentials) {
//     if let Status::Connected = connect_to_database() {
//         login(creds)
//     }
// }

/*
    ####################### this is the code with modules #######################
*/

mod database;
mod auth_utils;

/*
    we have access to same level modules
    for example auth_utils or database, we don't need to use "crate"
    because the execution context is in the same level than these modules
    we can use the declaration "use" to short the long namespaces
*/

/*
    use statements are also useful for reexporting and
    currently our crate module only exports one item, the authenticate method.
    we also need to export the credential struct from our top level model
    To accomplish this, we can reexport the credentials by adding "pub" to the line 75
*/
pub use auth_utils::models::Credentials;
use database::Status;

/*
    we just pass Credentials as argument
    since we are using "use" declaration
    avoiding long names as argument
*/
pub fn authenticate(creds: Credentials) {

    // we see how to install a crate an use it in the next line:
    let timeout = thread_rng().gen_range(100..500);

    // we are simulating a timeout and we can print it
    println!("The timeout is {timeout}");

    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds)
    }
}

/*
    this is the structure of our small program:

    [ auth_service ]$ cargo-modules structure

    crate auth_service
    ├── mod auth_utils: pub(crate)
    │   ├── fn login: pub
    │   ├── fn logout: pub(self)
    │   └── mod models: pub
    │       └── struct Credentials: pub
    ├── fn authenticate: pub
    └── mod database: pub(crate)
        ├── enum Status: pub
        ├── fn connect_to_database: pub
        └── fn get_user: pub
*/