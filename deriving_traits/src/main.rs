#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p1 = Point { x: 3, y: 1} ;
    let p2 = Point { x: 3, y: 1} ;
    let p3 = Point { x: 4, y: 4} ;

    /*
        colon question mark syntax is used to print out
        some type with debug format, but it only works
        if the types implement the Debug trait
        that's why we get an error in line 15 p1
        we can fix it using #[derive(Debug)] for point
    */

    println!("{:?}", p1);
    println!("{:?}", p1 == p2);
    println!("{:?}", p1 == p3);
}
