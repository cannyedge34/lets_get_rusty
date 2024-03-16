fn main() {
    let a = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    // same line
    // we need to return the same type of data
    let b = if a > 5 { 1 } else { -1 };

    // loops
    loop  {
        loop {
            // this inner loop break will not break
            // the parent loop
            break;
        }
        println!("loop forever");
        break
    }

    // label for loop, the name needs to start with a tick symbol
    'outer: loop {
        // now in the inner loop we can break the outer loop
        // with break 'outer;
        loop {
            break 'outer;
        }
    }

    // also we can return values from a loop
    let x = loop {
        break 5;
    };

    // while loop
    let mut a = 0;

    while a < 5 {
        println!("a is {a}");
        // we would have an error here because in rust
        // variables are inmutable by default
        // we need to declare a as mutable in line 42
        a = a + 1;
    }

    let a = [1,2,3,4,5];

    for element in a {
        println!("{}", element)
    }

}
