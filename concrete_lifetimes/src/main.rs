fn main() {
    /*
        we have two types of lifetimes: concrete/generic
        This chapter is about concrete lifetime

        a concrete lifetime is the time during which
        a value exists at a particular memory location

        a lifetime starts when the value is created or moved
        into a particular memory location and ends when the value is dropped/moved
        out a particular memory location
    */

    /*
        in this particular example the lifetime of s1 starts in 20 and
        ends on line 21 when the value in s1 is moved into s2
        that's why we have the error in line 22 (value borrowed here after move)
    */

    // let s1 = String::from("Let's get Rusty!");
    // println!("s1: {s1}");
    // let s2 = s1;
    // println!("s1: {s1}");

    let r1;

    {
        let s1 = String::from("Let's get Rusty!");
        r1 = &s1;
    }

    /*
        we have an error in line 36 because s1 is dropped inline 30
        and rust does not allow dangling references and
        the borrow checker checks at compile time and s1 lifetime ends
        on line 30 at the end of the inner scope

        we can fix the error "r1 = &s1; ^^^ borrowed value does not live long enough"
        moving the println sentence inside the inner scope.
    */
    println!("r1: {r1}");
}
