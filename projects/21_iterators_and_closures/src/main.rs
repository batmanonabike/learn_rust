// Closures - anonymous functions that capture their environment.
fn main() {
    let simulated_random_number = 7;
    let simulated_user_specified_value = 10;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout_using_closure(simulated_user_specified_value, simulated_random_number);
    generate_workout_with_lazy_evaluation(simulated_user_specified_value, simulated_random_number);
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

// Functions and closure syntax is similiar.
#[allow(dead_code)]
#[allow(unused_variables)]
fn add_one_v1(x: u32) -> u32 { x + 1 }

#[allow(dead_code)]
#[allow(unused_variables)]
fn closures_look_like_functions() {

  //fn  add_one_v1   (x: u32) -> u32 { x + 1 }
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
    where T: Fn(u32) -> u32 
{
    calculation: T,
    value: Option<u32>
}
// Here the trait bounds on T specify that its a closure by using the Fn trait.

// The value before we execute the closure will be None.
// When code using Cacher asks for the result of the closure, the Cacher will execute the closure at
// that time and store the result within a Some variant in the value field.  If the code asked for
// the result of the closure again, the Cacher will return the result but not execute the closure
// again.
impl<T> Cacher<T>
    where T: Fn(u32) -> u32 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
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

    // Now we only execute the expensive call once for each differing input.
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly but only ONCE...");
        thread::sleep(Duration::from_secs(2));
        num
    });

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
