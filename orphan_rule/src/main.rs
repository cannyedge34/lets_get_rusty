use orphan_rule::Point;

/*
  the trait or the type are not defined within the binary crate
  that's why we see the error: only traits defined in the current crate can be implemented for arbitrary types...

  we can fix the issue creating a wrapper type, we create the struct PointWrapper
  PointWrapper is a tuple struct which simply wraps the Point struct
*/

struct PointWrapper(Point);

impl PartialEq for PointWrapper {
  fn eq(&self, other: &Self) -> bool {
      // self.x == other.x && self.y == other.y;
      self.0.x == other.0.x && self.0.y == other.0.y
  }
}

fn main() {
  let p1 = PointWrapper(Point { x: 1, y: 2 });
  let p2 = PointWrapper(Point { x: 1, y: 2 });

  println!("{}", p1 == p2);
}