// Closures - anonymous functions that capture their environment.
fn main() {
    let simulated_random_number = 7;
    let simulated_user_specified_value = 10;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout_using_closure(simulated_user_specified_value, simulated_random_number);
    generate_workout_with_lazy_evaluation(simulated_user_specified_value, simulated_random_number);

    closure_with_forced_move();

    simple_iterator();
    calling_next_on_iterator();
    using_iterators_to_mutate_data();
    taking_ownership_with_iterators();
    methods_that_consume_iterator();
    methods_that_produce_other_iterators();
    using_the_collect_method();

    calling_next_directly_on_our_own_type();
    using_other_iterator_trait_methods();
}

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity : u32) -> u32 {
    println!("Calculating slowly and repeatedly with normal functions...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today do {} situps", simulated_expensive_calculation(intensity));
        println!("Today do {} pushups", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break.");
        } else {
            println!("Today, run from {} minutes", simulated_expensive_calculation(intensity));
        }
    }
}

#[allow(unused_variables)]
fn generate_workout_using_closure(intensity: u32, random_number: u32) {

    // Closure definitions come after the '=' starting with a pair of vertical pipes inside which we 
    // specify parameters to the closure.  These are comma separated if multiples are needed.
    let expensive_closure = |num| {
        println!("Calculating slowly and repeatedly with enclosures...");

        thread::sleep(Duration::from_secs(2));
        num
    };

    // Closure definitions will have one concrete type inferred for each of their parameters and 
    // their return value.
   
    // Closures to not 'require' type annotations or return values like functions do.  Closures are 
    // usually short and relevant within a narrow context, the compiler infers types. 
    // Though we can be explicit if we like:
    let explicit_expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly and repeatedly with enclosures...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today do {} situps", expensive_closure(intensity));
        println!("Today do {} pushups", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break.");
        } else {
            println!("Today, run from {} minutes", expensive_closure(intensity));
        }
    }
}

#[allow(dead_code)]
fn closures_look_like_functions() {
    
    // Functions and closure syntax is similiar.
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    add_one_v1(2);
    add_one_v2(2);
    add_one_v3(2);
    add_one_v4(2);
}

// Storing closures using generic parameters and the 'Fn' traits.
// We can create a struct to hold the closure and resulting value of calling the closure.  
// 
// 'memoization' aka: 'lazy evaluation'
// ------------------------------------
// The struct will execute the closure only if we need the resulting value and will cache the 
// resulting value and it will cache the resulting value. 
//
// For structs to hold a closure we need to specify the type of closure and types of each of its 
// fields.
// Each closure 'instance' has its own unique anonymous type; even if two closures use the same
// signature, their types are considered different.
struct Cacher<T>
    where T: Fn(u32) -> u32 // The trait bounds on T specify that its a closure using the Fn trait.
{
    calculation: T,     // This is like a function pointer, in this case with a signature taking a
                        // u32 and returning a u32...  T: Fn(u32) -> u32                         
    value: Option<u32>
}

// When code using Cacher asks for the result of the closure, the Cacher will execute the closure at
// that time and store the result within a Some variant in the value field.  If the code asked for
// the result of the closure again, the Cacher will return the result but not execute the closure
// again.
impl<T> Cacher<T>
    where T: Fn(u32) -> u32 // Like a function pointer
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None // The value before we execute the closure will be None.
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, // Check if we have already made the calculation.
            None => {
                let v = (self.calculation)(arg); // Fn(u32) -> u32 (a bit like a c++ member fn ptr).
                self.value = Some(v); // Store the result.
                v
            }
        }
    }
}

fn generate_workout_with_lazy_evaluation(intensity: u32, random_number: u32) {

    // Passing a closure of type: Fn(u32) -> u32 to expensive_result
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly with enclosures but only ONCE...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // Now we only execute the expensive call once for each differing input.
    if intensity < 25 {
        println!("Today do {} situps", expensive_result.value(intensity));
        println!("Today do {} pushups", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break.");
        } else {
            println!("Today, run from {} minutes", expensive_result.value(intensity));
        }
    }
}

#[test]
#[allow(unused_variables)]
fn call_with_different_values() {

    // One problem with that above code is that the code assumes it will get the same value for the
    // parameter 'arg' to the 'value' method.
    // The test below will fail.
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2); // See implementation of Cacher::value.  
    // A potential resolution to this would be to store a hash map and return the value if its 
    // present.

    assert_eq!(v2, 2);  // FAILS!
}

// Another problem with the aboce code is that we are tied into one parameter of u32 and a return
// value of u32.
// To address this we can introduce more generic parameters to increase the flexibility of the 
// Cacher functionality.

// Capturing the environment with closures.
// ----------------------------------------
// Closures can capture values from their environments in three ways, analogous to the three ways a
// function can take a parameter: taking ownership, borrowing mutably and borrowing immutably.
// These are encoded in the three Fn traits:
// 'FnOnce': consumes the variables it captures from its enclosing scope, known as the closure's
//           environment.   So the closure must take ownership of the variables and move them into 
//           the closure where its defined.  The 'Once' part of the name indicates that the closure 
//           cant take ownership of the same variables more than once so IT CAN ONLY BE CALLED ONCE.
// 'FnMut':  can change the environment because it miutable borrows values.
// 'Fn':     borrows values from the environment immutably.
// When creating a closure, Rust infers which trait to use based no how the closure uses the value 
// from the environment.  
// All closures implement 'FnOnce' because they can all be called at least once.
// Closures that dont move captured variables also implement 'FnMut' 
// Closures that need mutable access to the captured variables also implement 'Fn'.

// You can FORCE a closure to take ownership
fn closure_with_forced_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x; // move x into return value from this closure.
    // This technique is mostly useful when passing a closure to a new thread to move the data so
    // that its owned by the new thread. 

    // x is now out of scope, this wont compile...
    //println!("We can use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// Iterators.
// ----------
// Iterators are lazy, they take no effect until you call the methods that consumre the iterator
// to use it up.
fn simple_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // Only when this loop is used with the iterator be used.
    for val in v1_iter {
        println!("iterator got: {}", val);
    }
}

// All iterators implement a trait named 'Iterator' that is defined in the standard library.
// It will looks something like this...
pub trait MyIterator { // (Renamed from Iterator to avoid clashes)
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // other stuff...
}
// 'type Item' and 'Self::Item' are defining an 'associated type' with this trait.  These are
// discussed later.  For now, its enough to say that the Iterator trait requires that you also
// define an Item type and this Item type is used as the return type from the 'next' method.
// The Iterator trait only requires implementors to define one method, the 'next' method which 
// returns one item of the iterator at a time wrapped in 'Some' and, when the iteration is over
// return 'None'.

// Its possible to call 'next' on iterator directly..
fn calling_next_on_iterator() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // Note that we need to make the iterator mutable because its internal state changes to keep 
    // track of where the iterator is in the sequence.
    // We did not need that in the 'for' loop in 'simple_iterator' above because the loop took 
    // ownership of the iterator and made it mutable behind the scenes.

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // Methods 
}

fn using_iterators_to_mutate_data() {

    let mut v1 = vec![1, 2, 3];

    for val in v1.iter() {
        println!("before mutating: {}", val);
    }
    // The .iter() method produces immutable references to the contained data.  If we want to 
    // iterate over mutable references we can use .iter_mut()
    for val in v1.iter_mut() {
        *val += 1; // We need to dereference.
    }

    for val in v1.iter() {
        println!("after mutating: {}", val);
    }
}

#[allow(unused_variables)]
fn taking_ownership_with_iterators() {

    let v1 = vec![1, 2, 3];

    println!("len: {}", v1.len());

    // .into_iter() creates an iterator that takes ownership of v1 and returns owned values...
    for mut val in v1.into_iter() {
        val += 1; 
        println!("val: {}", val);
    }

    // v1 is now out of scope, this won't compile...
    //println!("len: {}", v1.len());
}

fn methods_that_consume_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    // Methods that call .next are referred to as 'consuming adaptors' because calling them uses up 
    // the iterator.
    // for val in v1_iter {}

    assert_eq!(total, 6);
}

#[allow(unused_must_use)]
fn methods_that_produce_other_iterators() {

    let v1: Vec<i32> = vec![1, 2, 3];

    // Other methods defined on the Iterator trait, known as 'iterator adaptors' allow you to change
    // iterators into other kinds of iterators.
    // The .map method takes a closure to call on each item to produce a new iterator.  The closure
    // here creates a new iterator in which each item from the vactor has been incremented by 1.
    v1.iter().map(|x| x + 1);
    // However, this produces a warning because the closure never gets calls, iterators are lazy, 
    // and we need to consume the iteartor here.
    // note: `#[warn(unused_must_use)]` on by default
    // note: iterators are lazy and do nothing unless consumed
    // (We are supressing the warning above this functions signature).
}

fn using_the_collect_method() {

    let v1: Vec<i32> = vec![1, 2, 3];

    // Using .collect consumes the iterator and collects the resulting values into a collection
    // data type.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // Collect the results of iterating over the iterator thats returned from the call to map into a
    // vector.
    // Because mpa takes a closure, we can specify any operation we want to perform on each item.

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {

    // The .filter method on an iterator takes a closure that takes each item from the iterator and
    // returns a Boolean.  If the closure returns true, the value will be included in the iterator
    // produced by filter.  If the closure returns false, the value won't be included in the 
    // resulting iterator.

    shoes.into_iter() // create an iterator that takes ownership
        .filter(|s| s.size == shoe_size) // adapt to new iterator to filter contents.
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

// Implementing iterators on our own types by implementing the Iterator trait.
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Associated type.

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn calling_next_directly_on_our_own_type() {

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    
    assert_eq!(18, sum);
}
