// fn main() {
    /*
    s1 is the owner of the string stored on the heap
    and when s1 goes out of scope, the heap will be cleared.
    see stack_heap_1 file
    */

    // let s1 = String::from("Rust"); // heap allocated string
    // println!("s1 is: {s1}");
// }  s1 is dropped

// ##########################################################################

// what about if we move s1 to an inner scope:
// fn main() {
//     {
        // let s1 = String::from("Rust"); // heap allocated string
    // } // s1 is dropped here

    // println!("s1 is: {s1}"); error here
// }

// ##########################################################################

// fn main() {
    // let s1 = String::from("Rust");

    /*
        in rust the values are moved by default,
        in this case, the value is moved to s2 (this is not true for some primitive types)
        and s2 is the owner of the string and s1 is invalidated
        see stack_heap_2 picture
    */
    // let s2 = s1;

    // println!("s1 is: {s1}"); // it's invalid since the owner is s2 now
    // println!("s2 is: {s2}");
// }

// ##########################################################################

// what about if we want to clone the value instead of moving it?
fn main() {
    let s1 = String::from("Rust");
    /*
        we just need to use the clone() method.
        // see stack_heap_3 picture
    */

    let s2 = s1.clone(); // s2 has a copy of "Rust" string
    println!("s1 is: {s1}"); // we don't have any error using clone()

    // we don't need to use clone for some primitive types (it is cloned by default):
    let x = 10;
    let y = x;

    println!("x is: {x}"); // our code is compiling without errors
    /*
        this is because in rust primitives are store in the STACK (heap is not used)
        such as integers, floating numbers, booleans or characters and they are cloned by default
        this types are cheap to clone.
     */
}
