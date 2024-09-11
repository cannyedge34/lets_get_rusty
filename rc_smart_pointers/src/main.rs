use std::rc::Rc;

struct Database {}

struct AuthService {
    // error with
    // db: Database
    db: Rc<Database>
}

struct ContentService {
    // error with
    // db: Database
    db: Rc<Database>
}

fn main() {
    /*
        let db = Database {};
        let auth_service = AuthService { db: db };

        we get an error in line 26
        since we are transferring the ownership
        from db into the db field in auth_service

        let content_service = ContentService { db: db };
    */

    /*
        However in some situations we might want to use "share ownership"
        for example: these two services want to share the same database instance
        that's why we can use rc smart pointer
    */

    /*
        Solution:
            Using Rc::clone does not clone the database instance
            it just increment the references count.
            When we create an instance of Rc in line 44 the references count is 1
            then when we create the auth_service instance the references count is 2
            and finally content_services instance increases the references count to 3
    */

    let db = Rc::new(Database {});
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    /*
        at the end of them main function all 3 variables would be dropped
        when content_service variable is dropped Rc smart pointer of db field will be dropped
        when the Rc pointer is dropped, the references count is decremented by 1 (in these case from 3 to 2)
        then, on line 45 the auth service decrement the references count from 2 to 1
        and finally on line 44 the references count go from 1 to 0
        when the references count hits 0 the Rc smart pointer will drop the value its holding
        in this case an instance of the database.
    */

    /*
        Rc smart pointer can only be used in single threaded applications
        for multi-threaded applications it must be used the atomically references counter smart pointer
    */
}
