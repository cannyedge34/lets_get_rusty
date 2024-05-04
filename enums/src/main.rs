// enum ProductCategory {
//   Books,
//   Clothing,
//   Electronic
// }

// struct Product {
//   name: String,
//   category: ProductCategory,
//   price: f32,
//   in_stock: bool
// }

// fn main() {
//   let category = ProductCategory::Electronic;
//   let product = Product {
//     name: String::from("Car"),
//     category,
//     price: 32.0,
//     in_stock: true
//   };
// }


// ############################################################
// this is a more complex use of enums
enum Command {
  Undo,
  Redo,
  AddText(String),
  MoveCursor(i32, i32),
  Replace {
    from: String,
    to: String
  }
}

impl Command {
  fn serialize(&self) -> String {
    // dummy string
    return String::from("JSON string");
  }
}

fn main() {
  let cmd = Command::Undo;
  let cmd = Command::AddText(String::from("test"));
  let cmd = Command::MoveCursor(22, 0);
  let cmd = Command::Replace { from: String::from("here"), to: String::from("there") };

  let json_string = cmd.serialize();
  print!("{}", json_string);
}

// ############################################################