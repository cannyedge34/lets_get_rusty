fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone();

    print_string(s1.clone());
    /*
        the ownership of s1 string is now is p1
        and p1 is moved to the scope of print_string method
        and it's invalidated in line 12
        // we can pass parameter s1.clone() and s1 will not have error in line 12
    */

    println!("s1 is: {s1}");

    let s3 = generate_string();
    println!("s3 is: {s3}");

    let s4 = add_to_string(s2);
    println!("s4 is: {s4}");

    let x = 10;
    let y = x;

    print_integer(x);
    /*
        now we can print x in line 32
        because the vale in line 22 is cloned
        This case works for primitive data-types
        because they are stored in the Stack (integers, floating point numbers, booleans and characters)
    */

    println!("x is: {x}");
}

fn print_integer(i: i32) {
    println!("i is: {i}");
}

fn add_to_string(mut p1: String) -> String {
    /*
        this method takes ownership of the string s2 with p1 in line 18
        mutate the string
        append "is great" to the p1 string (Rust)
        and it moves the ownership of that string p1 out of this method into s4
    */
    p1.push_str(" is great");
    p1
}

fn generate_string() -> String {
    String::from("Ferris") // the ownership of this string is transferred to s3
}

fn print_string(p1: String) {
    println!("{p1}");
} // p1 is dropped
