// fn main() {
//     let s1 = String::from("Rust"); // heap allocated string
//     print_string(s1);

    /*
        we have error in s1 because the ownership
        of s1 is transferred to p1 into the method
        print_string
    */

//     println!("s1 is: {s1}");
// }

// fn print_string(p1: String) {
//     println!("{p1}");
// }

//###################################################################

fn main() {
    let mut s1 = String::from("Rust"); // heap allocated string
    let r1 = &s1; // this is a immutable reference to s1, (we need to use & symbol) // in rust references are immutable by default
    print_string(r1);
    let r2 = &mut s1; // this is a mutable reference (&mut)
    add_to_string(r2); // it's taking a mutable reference (r2) instead of ownership of the string

    /*
        first borrowing rule: there can be only one mutable reference
        or many immutable references at the same time. We can fix it
        moving print_string(r1) to line 23 and r2 to line 24
        because these two references are not used at the same time
    */

    /*
        we have error in s1 (line 33) because the ownership
        of s1 is transferred to p1 into the method
        add_to_string. We can solve it creating the s1 again
        let s1 = add_to_string(s1)
        but this code is not efficient, we can fix it creating references r2
    */
    println!("s1 is: {s1}");

    let s2 = generate_string();
}

/*
    we need to put as a parameter p1: &String to make it work
    because it's a reference to a String
*/
fn print_string(p1: &String) {
    println!("{p1}");
}

fn generate_string() -> &String {
    /*
        let's see what it happens returning a reference to a string
        instead of returning a string like this: String::from("Ferris");
     */
    let s = String::from("Ferris");
    &s
} /*
    s is dropped and the string is cleaned up
    which means the the &s is a dangling reference
    because it's pointing to an invalid memory
  */

/*
    it takes a mutable reference of a string
*/
fn add_to_string(p1: &mut String) {
    /*
        we can use push_str method because
        rust has a feature call, automatic dereferencing
        we don't need to explicitly dereference p1 on line 61
        however if we want to do it manually it would be something like this:
        (*p1).push_str(" is great")
        * symbol is the dereference operator.
     */
    p1.push_str(" is great");
}

