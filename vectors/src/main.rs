// fn main() {
    /*
        vectors are growable and always allocate memory on the heap
        vectors are sequence of elements of the same type.
        however arrays are not growable and they allocate memory on the stack
    */
    // let mut v = Vec::new();
    // v.push(String::from("One"));
    // v.push(String::from("Two"));
    // v.push(String::from("Three"));

    // the vector has ownership of the elements
// } v is dropped and all of its elements are dropped as well

// second way to create vectors
fn main() {
    /*
        vec macro defines a similar how we would define an array
        the benefit of this approach is we can initialize a vector
        with elements without pushing elements
    */

    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    let _v2 = vec![1, 2, 3];

    // we can not move an element out of the vector
    // using the syntax because that would leave the vector in a invalid state
    let _s = &v[0]; // can panic

    /*
        with the "remove" method, return an element at the given index
        moving it out of the vector but it will also shift all the elements after it
        to the left such that the vector stays in a valid state
    */
    // let s = v.remove(0);

    /*
        the second way to index into a vector is to use a "get" method
        it's safer than using the bracket syntax because calling the get method will not panic
        instead the get method returns an optional, if the index is valid we will get some variant
        with the reference to the element, and if it's invalid we will get the None variant
    */
    let s = v.get(0);

    if let Some(e) = s {
        println!("{e}");
    }

    // we need to use &mut because we want to mutate the strings
    // but we don't want to take ownership of v
    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}")
    }

    /*
        for loop consuming a vector
    */

    let mut v3: Vec<String> = vec![];

    /*
        moving to from v to the v3 vector
        this time we are taking v by value instead of a reference to v
        this allow us to move the elements inside v (loop),
        but after the for loop call, v is not longer valid
    */
    for s in v.into_iter() {
        v3.push(s);
    }

    // error borrow of moved value: `v` if we try:
    // let i = v.get(0);
}
