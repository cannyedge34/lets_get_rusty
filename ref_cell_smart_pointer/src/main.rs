use std::rc::Rc;
use std::cell::RefCell;

struct Database {
    max_connections: u32
}

struct AuthService {
    db: Rc<RefCell<Database>>
}

struct ContentService {
    db: Rc<RefCell<Database>>
}

fn main() {
    /*
        let db = Rc::new(Database { max_connections: 100 });
        let auth_service = AuthService { db: Rc::clone(&db) };
        let content_service = ContentService { db: Rc::clone(&db) };
    */

    /*
        error here changing the max connections:
        db.max_connections = 200

        cannot mutate immutable variable `db`

        because rc smart pointer just allow
        immutable shared ownership of the value
    */

    /*
        we can use refcell to allow change the max_connections value
    */

    let db = Rc::new(RefCell::new(Database { max_connections: 100 }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    /*
        borrow_mut method, allows us to mutable borrow value wrapped in ref cell
        db.borrow_mut().max_connections = 200;
    */

    /*
        If we try to borrow the mutable value twice simultaneously, the program will fail:

        let r1 =  db.borrow_mut();
        let r2 =  db.borrow_mut();

        r1.max_connections = 200;
   |     ^^ cannot borrow as mutable

        lend_mut() allows you to get a mutable reference to a value inside a RefCell,
        and while Rust normally handles borrowing and mutability at compile time,
        RefCell and lend_mut() check it at runtime.
        This is useful for cases where mutability is difficult to manage at compile time
        due to complex ownership or borrowing rules.
    */

    /*
        we can add mut variable r1 and we will not get any error in compile time
        but we will have an error in runtime:

        thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:70:18
    */

    let mut r1 =  db.borrow_mut();
    let r2 =  db.borrow_mut();
    r1.max_connections = 200;

}

