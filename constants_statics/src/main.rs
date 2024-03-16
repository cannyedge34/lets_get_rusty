
//constants don't use a specific location in memory
const MAX_PLAYERS: u8 = 10;

//static use a specific location in memory which mean
// there is only once instance of the value
static CASINO_NAME: &str = "Rusty Casino";

// use constant unless you really need static variables
// use static when you need to store large amount of data
// when you need the single address property of statics
// or when you are using interior mutability

fn main() {
    let a = MAX_PLAYERS;
    let b = MAX_PLAYERS;

    let c = CASINO_NAME;
    let d = CASINO_NAME;
}
