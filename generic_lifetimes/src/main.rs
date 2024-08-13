fn main() {
    let player1 = String::from("player 1"); // lifetime of p1 is from line 2 to line 7
    let player2 = String::from("player 1"); // lifetime of p2 is from line 2 to line 7

    /*
        added a generic lifetime to the first_run function,
        the borrow checker knows that the lifetime of result
        is gonna be from line 10 to line 13 which means
        we can print result in line 12
    */
    let result = first_run(player1.as_str(), player2.as_str());
    println!("Player going first is: {}", result);
}

/*
    generic lifetime annotations specify/describe the relationship between lifetimes and references
    in this case we are saying that there is relationship between p1, p2 and the returned value.
    the relationship is this one: the lifetime of the returned value is gonna be equal to the shortest lifetime passed in
    the shortest lifetime is p1 and the lifetime of the returned value will be equal to p1
*/
fn first_run<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    /*
        this is am ambiguity since the borrow checker is not able
        to analyze this code resulting in an error (missing lifetime specifier)
        Lifetime specifier as known as "generic lifetime annotation"

        in rust we define lifetime annotation inside of angle brackets
        after the function name. In this case first_run<a'>ç
        the convention is naming the lifetimes with the alphabet letters
    */
    if rand::random() {
        p1
    } else {
        p2
    }
}
