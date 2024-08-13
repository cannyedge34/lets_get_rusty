fn main() {
    let player1 = String::from("player 1"); // lifetime of p1 is from line 2 to line 11

    {
        let player2 = String::from("player 1"); // lifetime of p2 is from line 5 to line 9
        // lifetime of result is gonna be until line 9 because is the shortest lifetime passed in
        let result = first_run(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    }
}

fn first_run<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

