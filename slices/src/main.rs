// fn main() {
    /*
        Slices are references to a contiguous sequence
        of elements in a collection
    */

    /*
        String is growable, heap allocated
        // see string_types files for details
    */
    // let sentence = String::from(
    //     "This is my sentence and it's very very long"
    // );

    /*
        str is immutable sequence of UTF-8 bytes
        // see string_types file for details
    */
    // let trimmed_sentence: &str = &sentence[..20]; //string slice
    // println!("{trimmed_sentence}");

    /*
        the use of these kind of string is depending on the use case:
        1. when you need to own the string because you want to mutate it
        then use String
        2. when if you only need and immutable view of the string or
        a subset of the string, we can use string slices
        3. one important thing in rust if we declare a string like:
        let s = "one simple string", all string literals are string slices.
        and the strings themselves are stored in the application binary.
        in this case, s is a string slice pointing to a specific location
        of the programs binary
    */
// }

// slices with functions
fn main() {
    let sentence = String::from(
        "This is my sentence and it's very very long"
    );

    let trimmed_sentence: &str = trim_sentence(&sentence); // we don't have error here because:
    // a rust feature called "deref coercion"

    let sentence2 = "This is second sentence and it's very very long";

    let trimmed_sentence2 = trim_sentence(sentence2);
    println!("{trimmed_sentence}");
    println!("{trimmed_sentence2}");

    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("{:?}", a_slice) // with debug format
}

fn trim_sentence(sentence: &str) -> &str {
    &sentence[..20]
}