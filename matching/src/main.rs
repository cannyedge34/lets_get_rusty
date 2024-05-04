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
      // return String::from("JSON string");

      let json_match = match self {
        Command::Undo => String::from(
          "{ \"cmd\": \"undo\" }"
        ),
        Command::Redo => String::from(
          "{ \"cmd\": \"redo\" }"
        ),
        Command::AddText(s) => {
          format!(
            "{{\
               \"cmd\": \"add_text\", \
               \"text\": \"{s}\"
            }}"
          )
        },
        Command::MoveCursor(line, column) =>
          format!(
            "{{\
              \"cmd\": \"move_cursor\", \
              \"line\": {line}, \
              \"column\": {column} \
            }}"
          ),
        Command::Replace { from, to } => {
          format!(
            "{{\
              \"cmd\": \"move_cursor\", \
              \"from\": {from}, \
              \"to\": {to} \
            }}"
          )
        }
      };

      json_match
    }
  }

  fn main() {
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace { from: String::from("here"), to: String::from("there") };

    print!("{}", cmd1.serialize());
    print!("{}", cmd2.serialize());
    print!("{}", cmd3.serialize());
    print!("{}", cmd4.serialize());


    // let age = 35;

    // match age {
    //     1 => println!("Happy 1st Birthday!"),
    //     13..=19 => println!("You are a teenager!"),
    //     x => println!("You are {x} years old!"),
    // }


  }
