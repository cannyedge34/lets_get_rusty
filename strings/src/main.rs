// fn main() {
//     let s1 = "Привет, мир";
//     let s2 = String::from("Привет, мир");
//     let s3 = "Привет, мир".to_string();
//     let s4 = "Привет, мир".to_owned();
//     let s5 = &s4[..]; // reference for the entire string

//     println!("{}", s5)
// }

// fn main() {
//     let mut s = String::from("foo");
//     s.push_str("bar");
//     println!("{}", s);
//     s.replace_range(.., "baz");
//     println!("{}", s);
// }

// fn main() {
//     // + operator
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     // s1 is moved to s3 and append the content of s2 to s3
//     let s3 = s1 + &s2;

//     println!("{}", s3);
// }

// fn main() {
//     // other way of concat string is with format macro:
//     let s1 = String::from("one");
//     let s2 = String::from("two");
//     let s3 = String::from("three");

//     let s = format!("{}-{}-{}", s1, s2, s3);
//     println!("{}", s)
// }

// fn main() {
//     //more ways of using strings
//     let s1 = ["first", "second"].concat();
//     let s2 = format!("{}{}", "first", "second");
//     let s3 = concat!("first", "second");

//     let s4 = String::from("test");
//     let s5 = s4 + "okokok";
// }

// fn main() {
//     let s1 = "😀😀😀😀";
    /*
        one icon is 4 bytes long that's why we get an error when we call s1[0]
        coz 0 is calling for the first byte instead of the first icon
        rust does not let you index strings using icons
    */
    // let s2 = s1[0];

    /*
        However we can create the string with the first icon like this
        we don't get any error with let s2 = &s1[0..4];
    */
//     let s2 = &s1[0..4];
//     println!("{}", s2)
// }

// iterating strings
// add the dependency to the Cargo.toml file and run "cargo build"
// use unicode_segmentation::UnicodeSegmentation;

// fn main() {
    // for b in "होला मुंडो".bytes() {
    //     println!("{}", b)
    // }

    // other option is using .chars()
    // for c in "होला मुंडो".chars() {
    //     println!("{}", c)
    // }

    // for c in "होला मुंडो".graphemes(true) {
    //     println!("{}", c);
    // }
// }

// string and functions
fn main() {
    let s1 = "Hello World!"; // string slice
    let s2 = String::from("Hello World!"); // string

    my_function(s1);

    /*
        we can pass to the function both (string slice and string)
        this is because of a rust feature called deref coercion
        https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/deref-coercions.html
    */
    /*
        when we pass a reference to a string, rust
        automatically coerce to a string slice
    */
    my_function(&s2);


    fn my_function(a: &str) -> String {
        return format!("{}", a);
    }
}