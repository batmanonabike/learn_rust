// Using Box<T> for allocating on the heap.
// Use Box<T> when:
// 1).  You have a type whose size cannot be known at compile time.
// 2).  There is large amounts of data when transferring ownership.
// 3).  You want to own a value and care only that it's a type that implements a particular trait
//      rather than being of a specific type.
pub fn box_contrived_example() {
    let b = Box::new(5);    // Limited value of this, just a i32 on the heap.
    println!("b = {}", b);  // Box<T> seems analogous too std::unique_ptr in C++.
}

// Recursive types with Boxes.

// Cons List
// A 'construct function' list is a recursive data structure whereby you construct a pair from its 
// two arguments, which usually are a single value and another pair.  These pairs containing pairs
// form a list.
enum MyList {
    Cons(i32, Box<MyList>),
    Nil
}

#[allow(unused_variables)]
pub fn test_con_list() {

    use crate::MyList::{Cons, Nil}; 
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));
}
// Box<T> is a smart pointer because it implements the 'Deref' trait which allows it to be used like
// a reference.  The heap is cleaned up when the containing instance goes out of scope.

pub fn test_standard_references() {
    let x = 5;
    let y=  &x; 
    assert_eq!(5, x);

    //assert_eq!(5, y);
    assert_eq!(5, *y); // Don't forget to dereference the reference.
}

pub fn dereferencing_a_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    //assert_eq!(5, y);
    assert_eq!(5, *y);
}

// Defining our own smart pointer.
struct MyBox<T>(T); // Using a generic tuple struct with one element.

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }    
}

// We need to implement the 'Deref' trait so we can dereference using the * operator.
// Without the deref trait, the compiler can dereference only & references.
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T; // Assoicated type for the Deref trait to use.

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn test_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Uses deref

    // *y 
    // is equivalent to:
    let z = *(y.deref());
    assert_eq!(5, z);
}

// Implicit Deref coercions with functions and methods.
// Deref coercion is added as a convenience to reduce explicit references and dereferences with & 
// and *.
fn deref_coercion(name: & str) { // We take a string slice
    println!("deref coercion: {}", name);
}

pub fn call_deref_coercion() {
    let m = MyBox::new(String::from("Test me"));

    // &m uses the Deref trait and Rust can then turn &MyBox<String> into &String.  The standard 
    // library on Deref on String returns a string slice.
    deref_coercion(&m);

    // Otherwise we would have to call:
    deref_coercion(&(*m)[..]);
    // *m gets is a String from MyBox<String>
    // & and [..] takes a full string slice.
}

// How Deref Coercion interacts with Mutability:
// Rust does deref coercion when it finds types and trait implementations in three cases:
// 1).  From &T to &U when T: Deref<Target=U>
// 2).  From &mut T to &mut U when T: DerefMut<Target=U>
// 3).  From &mut T to &U when T: Deref<Target=U>
// Immutable references will never coerce a mutable reference to an immutable one.

// Running code cleanup with the Drop Trait (like destructors in C++).
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with '{}'", self.data);
    }
}

#[allow(unused_variables)]
pub fn test_drop() {
    let a = CustomSmartPointer { data: String::from("item a") };
    let b = CustomSmartPointer { data: String::from("item b") };
    println!("CustomSmartPointers created");
    // Drop gets called automatically when these a and b go out of scope.
}

#[allow(unused_variables)]
pub fn explicit_dropping_early() {
    let a = CustomSmartPointer { data: String::from("item a") };
    drop(a);    // using std::mem::drop explicitly.
}

// Using Rc<T> for a reference counting type that enables multiple ownership.
// Analogous to shared_ptr in C++.
use std::rc::Rc; // Rc is not in the prelude.
enum MyRcList {
    MyCons(i32, Rc<MyRcList>),
    Nil
}

#[allow(unused_variables)]
pub fn test_rc_list() {
    use crate::MyRcList::{MyCons, Nil};

    let a = Rc::new(MyCons(5, Rc::new(MyCons(10, Rc::new(Nil)))));
    let b = MyCons(3, Rc::clone(&a)); // Rc::clone increments the reference count.
    println!("ref count: {}", Rc::strong_count(&a));
    let c = MyCons(4, Rc::clone(&a)); // rc::clone is NOT taking a deep copy.
}

// RefCell<T> and the interior mutability pattern.
// Interior mutability is a design pattern in Rust that allows you to mutate data even when there 
// are immutable references to that data (this is normally prohibited by the borrowing rules).
// RefCell<T> represents single ownership over the data it holds.
// With Box<T> the borrowing rules invariants are enforced at compile time (most common case).
// With RefCell<T> the invariants are enforced at runtime (beware panics).
// Also note that RefCell<T> is only for use in single threaded scenarios and will cause a compiler
// error if used in a multi-threaded context.
// Mutating the value inside an immutable value is the 'interior mutability' pattern.
// 
// Ref<T> and RefMut<T>, accessed through RefCell<T> to enforce borrowing rules at runtime rather
// than compile time.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max
            }
        }

        // The Messenger trait has one method called 'send' that takes an immutable reference to 
        // self.  This set_value method takes a mutable reference.
        pub fn set_value(&mut self, value: usize) {
            self.value  = value;
            
            // The send method below takes an immutable reference to self.
            // We don't want to change the trait to use &mut self (as suggested by the compiler)
            // error.
            let percent = self.value as f64 / self.max as f64;
            if percent >= 1.0 {
                self.messenger.send("Oops: over quota");
            } else if percent >= 0.9 {
                self.messenger.send("Warning: quota over 90%");
            } else if percent >= 0.75 {
                self.messenger.send("Warning: quota over 75%");
            }
        }
    }

// Creating a Mock to keep track of messages.
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn testing_over_75_percent() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(1, 2);
    }
}


