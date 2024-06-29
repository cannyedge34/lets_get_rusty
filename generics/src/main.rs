/*
    Conventions suggest use T (which represents Type)
    and then moving down the alphabet
    if we have more than one Generics, for example <T, U>...
*/
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

/*
    implement new method for any type
*/
impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload
        }
    }

    /*
        get_payload method takes an immutable reference to self
        and return and reference to the payload
        we return T, because the payload is generic
    */
    fn get_payload(&self) -> &T {
        &self.payload
    }
}

/*
    implement print_payload when T is a string type
*/
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload)
    }
}

fn main() {
    // let cmd1 = BrowserCommand {
    //     name: "navigate".to_owned(),
    //     payload: "https:://google.com".to_owned(),
    // };

    // let cmd1 = BrowserCommand {
    //     name: "zoom".to_owned(),
    //     payload: 200
    // };

    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://google.com".to_owned()
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );
    cmd1.print_payload();
    // cmd2.print_payload() // we will have an error here since payload here is an integer

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);
}

fn serialize_payload<T>(payload: &T) -> String {
    // convert payload to JSON string...
    "placeholder".to_owned()
}

// pub enum Option<T> {
//     None,
//     Some(T),
// }

// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

/*
    Notes: Using generics in rust there is no runtime performance costs
    serialize_payload function is not slower than creating two function
    one for integer payload and other one for a string payload,
    because this is exactly what rust under the hood at compile time
    it's called Monomorphization https://rustc-dev-guide.rust-lang.org/backend/monomorph.html
*/