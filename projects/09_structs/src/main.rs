fn main() {
    test_user1();
    tuple_structs_wthout_named_fields();
}

#[derive(Debug)] // Can print debug infomation
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // Member functions, generally take a immutable self.  
    fn send_email(&self, content: &str) {

        if self.is_active() {
            println!("\"{}\" sent to {:?}", content, self);
        } else {
            println!("\"{}\" is not active", self.username);
        } 
    }
}

impl User {
    // Member functions implementations can be scattered.
    fn is_active(&self) -> bool {
        self.active
    }

    // 'Associated functions' are a bit like static member functions.
    // They dont take a self.
    fn are_both_active(first: &User, second: &User) -> bool {
        return first.active && second.active
    }

    #[allow(dead_code)]
    fn create(username: String, email: String) -> User {
        User {
            username: username,
            email: email,
            sign_in_count: 1,
            active: true
        }
    }

    // Does the same as the other create but is shorthand.
    fn create_shorthand(username: String, email: String) -> User {
        User {
            email, // shorthand, reusing name 'email'
            username, // shorthand, reusing name 'username'
            sign_in_count: 1,
            active: true
        }
    }
}

fn display_user_terse(user: &User) {
    println!("user1 (terse): {:?}", user); // {:?} - needs #[derive(Debug)]
}

fn display_user_verbose(user: &User) {
    println!("user1 (terse): {:#?}", user); // {:?} - needs #[derive(Debug)]
}

fn test_user1() {
    let user1 = User {
        email: String::from("martynandjasper@mail.com"),
        username: String::from("martyn"),
        active: true,
        sign_in_count: 1,
    };
    display_user_terse(&user1);

    // Create instance from with explicit fields from another instance.
    let user2 = User {
        email: user1.email,   // DONT FORGET BORROW/MOVE!!
        active: user1.active, // DONT FORGET BORROW/MOVE!!
        username: String::from("blah"),
        sign_in_count: user1.sign_in_count,
    };
    user2.send_email("blah blah");

    // BAD!  user1 'moved' from!  This wont compile.
    //display_user_verbose(&user1); // BAD

    // Create instance partially from another instance.
    let user3 = User {
        email: String::from("somewhere@blah.com"),
        active: false,
        ..user2 // Use everything else from user2.  DONT FORGET BORROW/MOVE!!
    };
    display_user_verbose(&user3);

    //user1.send_email("blah blah");
    user3.send_email("blah blah");

    let user4 = User::create_shorthand(
        String::from("another"), 
        String::from("another@me.com"));
    println!("Are both active: {}", User::are_both_active(&user3, &user4));
}

// Unit structs can be used to provide different types with different traits.
#[allow(dead_code)]
#[allow(unused_variables)]
struct UnitStruct {

}

fn tuple_structs_wthout_named_fields() {

    #[derive(Debug)] struct Colour(i32, i32, i32); // Note that these are... 
    #[derive(Debug)] struct Point(i32, i32, i32);  // ... different types

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
}
