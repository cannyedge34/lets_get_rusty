use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}
impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
 
impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPointer::new(
        Box::new("let's get rusty".to_owned())
    );
    // print(&s);
    /*
        this works because a rust feature
        called implicit deref coercion
        https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods

        MySmartPointer implements the deref trait, a reference to MySmartPointer
        can be coerce to the reference to the value inside the smart pointer.
        in this case is gonna be a reference to Box smart pointer, which is then coerce
        to a reference to String which is then coerce to string slice

        &MySmartPointer -> &Box -> &String -> &str
    */

    /*
        we can do this operation manually:
        let s = &(s);
        is a reference to MySmartPointer
    */

    /*
        lets use the dereference operator on s:
        let s = &(*s); is a reference to &Box<String>
        let s = &(**s); is a reference to &String
        let s = &(***s); is a reference to &str

        the number of calls to the deref method is result at compile time, so
        there is no runtime cost for using deref coercion
    */
    let s = &(***s);

    print(s)

}

fn print(s: &str) {
    println!("{s}")
}


