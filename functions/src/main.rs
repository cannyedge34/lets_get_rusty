fn main() {
   let z = my_function(22);
   println!("my_function returned: {}", z);
}

// use snake_case to name functions
// fn my_function(x: i32) {
//     println!("my_function called with: {}", x)
// }

// one important concept: functions return unit type by default
// they return ()

// we can return a value from function in this example with: -> i32
fn my_function(x: i32) -> i32 {
    println!("my_function called with: {}", x);
    let y = 10;
    // in rust the final expression in a function will be use
    // as the returned value, in this case y WITHOUT SEMICOLON
    y
}

// We can return early from a function with return statement
// fn my_function(x: i32) -> i32 {
//     return 5
//     println!("my_function called with: {}", x);
//     let y = 10;
//     // in rust the final expression in a function will be use
//     // as the returned value, in this case y WITHOUT SEMICOLON
//     y
// }

