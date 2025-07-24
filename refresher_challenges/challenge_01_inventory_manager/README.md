# Challenge 1: Inventory Manager

**Difficulty:** 🟢 Beginner  
**Topics:** Structs, Ownership, Basic Error Handling, Methods

## The Challenge

You're building an inventory management system for a small shop. Create a system that can:

1. Track items with their name, price, and quantity
2. Add new items to inventory
3. Update existing item quantities
4. Calculate total inventory value
5. Handle errors gracefully (like trying to remove more items than available)

## Requirements

### Core Structure
```rust
struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    items: Vec<Item>,
}
```

### Required Methods
Implement these methods for `Inventory`:

1. `new()` - Create a new empty inventory
2. `add_item(name: String, price: f64, quantity: u32)` - Add a new item
3. `update_quantity(name: &str, new_quantity: u32) -> Result<(), String>` - Update item quantity
4. `remove_items(name: &str, quantity: u32) -> Result<(), String>` - Remove items from inventory
5. `total_value() -> f64` - Calculate total inventory value
6. `find_item(&self, name: &str) -> Option<&Item>` - Find an item by name

### Example Usage
```rust
fn main() {
    let mut inventory = Inventory::new();
    
    inventory.add_item("Laptop".to_string(), 999.99, 5);
    inventory.add_item("Mouse".to_string(), 25.50, 20);
    
    println!("Total value: ${:.2}", inventory.total_value());
    
    match inventory.remove_items("Laptop", 2) {
        Ok(()) => println!("Removed 2 laptops"),
        Err(e) => println!("Error: {}", e),
    }
    
    match inventory.remove_items("Laptop", 10) {
        Ok(()) => println!("Removed 10 laptops"),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Learning Objectives

- Practice defining structs and implementing methods
- Work with ownership and borrowing (`&str` vs `String`)
- Use `Result<T, E>` for error handling
- Work with `Option<T>` for optional values
- Practice with vectors and iteration

## Hints

<details>
<summary>Click to see hints</summary>

1. **Finding items:** Use `Vec::iter()` and `find()` method
2. **Mutable access:** You'll need `iter_mut()` when modifying items
3. **Error handling:** Return descriptive error messages for edge cases
4. **String handling:** Remember the difference between `&str` and `String`

Example for finding an item:
```rust
self.items.iter().find(|item| item.name == name)
```

</details>

## Bonus Challenges

1. **Display trait:** Implement `Display` for `Item` to pretty-print items
2. **Search:** Add a method to search items by price range
3. **Sorting:** Add a method to sort items by name, price, or quantity
4. **JSON export:** Add a method to export inventory as JSON string (if you remember `serde`)

## Next Steps

Once you complete this challenge, move on to `challenge_02_file_processor` to practice file I/O and error handling!
