fn main() {
    let player1 = String::from("player 1");
    let result;
    {
        let player2 = String::from("player 1");
        result = first_run(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

fn first_run<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    /*
        we can use &'static str for string slices
        because they live in the program's binary
        meaning that they are valid for the entire duration of the program
    */
    let s: &'static str = "Let's get rusty!";
    /*
        returning s works because we are returning a value
        that lives at least as long as p1 and "s" is a static lifetime
        it would be greater or equal to p1

        the correct way of doing this is:
        fn first_run(p1: &str, p2: &str) -> &'static str {
    */
    s
}
