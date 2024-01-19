use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    let future = hello_world(); // creates a future, nothing is printed.

    // `block_on` blocks the current thread until the provided future has run to completion.
    // Effectively this converts an async call to sync. 
    // This stalls this thread!
    block_on(future);
}
