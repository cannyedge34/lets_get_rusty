fn main() {
    // creation
   let a: i16 = 5;

   // mutability
   let mut b = 5;
   // error without "mut" since variables in rust are inmutables
    b = 10;

   // shadowing
   let c = 10;
   let c = 20;

   println!("c is: {c}");

   // scope
   let d = 30;

   //inner scope
   {
    let d = 40;
    println!("inner d is {d}")
   }

   println!("d is: {d}")
}
