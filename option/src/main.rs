fn main() {
    let username = get_username(1);

    // match username {
    //     Some(name) => println!("{name}"),
    //     None => {}
    // }

    /*
        alternative syntax and it works in situations where
        you only care one variant and want to ignore the other variants
    */
    if let Some(name) = username {
        println!("{name}");
    }
}

// enum Option<T> {
//     None,
//     Some(T)
// }
/*
    Option can be used by default
    lets return get_username
*/

fn get_username(user_id: u32) -> Option<String> {
    // let db_result = String::from("Database");
    // if user_id == 1 {
    //     // we return the Some value of Enum Option
    //     Some(db_result)
    // } else {
    //     None
    // }

    let query =  format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);

    /*
        we need to protect against the errors querying the db
        rust has the Result enum to achieve this
    */

    // ok is a short name to represent the ok option of Result enum
    db_result.ok()
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }
/*
    Result can be used by default
    lets return get_username
*/

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        return Err(String::from("Query string is empty"));
    }
    return Ok(String::from("Database"));
}
