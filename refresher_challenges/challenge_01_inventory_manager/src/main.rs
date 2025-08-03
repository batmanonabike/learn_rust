// Challenge 1: Inventory Manager
// Your task: implement the missing methods for the Inventory system

struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, name: String, price: f64, quantity: u32) {
        match self.items.iter_mut().find(|item| item.name == name) {
            Some(inv) => {
                inv.price = price;
                inv.quantity += quantity;
            }
            None => {
                let new_item = Item {
                    name,
                    price,
                    quantity,
                };
                self.items.push(new_item);
            }
        }
    }

    fn find_item(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.name == name)
    }

    fn update_quantity(&mut self, name: &str, new_quantity: u32) -> Result<(), String> {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            item.quantity = new_quantity;
            Ok(())
        } else {
            Err(format!("Item {} not found in inventory", name))
        }
    }

    fn remove_items(&mut self, name: &str, quantity: u32) -> Result<(), String> {

        if let Some(index) = self.items.iter().position(|item| item.name == name) {          
            let mut item = &mut self.items[index];  
            if item.quantity < quantity {
                Err(format!("Insufficient quantity"))
            } else {
                item.quantity -= quantity;
                if (item.quantity == 0) {
                    self.items.remove(index);
                } 
                Ok(())
            } 

        } else {
            Err(format!("Item {} not found in inventory", name))
        }
    }

    fn total_value(&self) -> f64 {
        let mut total = -0.0;
        for item in self.items.iter() {
            total += item.quantity as f64 * item.price;
        }
        total
    }
}

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
        Ok(()) => println!("Removed 10 laptops", ),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_inventory() {
        let inventory = Inventory::new();
        assert_eq!(inventory.total_value(), 0.0);
    }

    #[test]
    fn test_add_and_find_item() {
        let mut inventory = Inventory::new();
        inventory.add_item("Test Item".to_string(), 10.0, 5);
        
        let item = inventory.find_item("Test Item");
        assert!(item.is_some());
        assert_eq!(item.unwrap().price, 10.0);
    }

    #[test]
    fn test_total_value() {
        let mut inventory = Inventory::new();
        inventory.add_item("Item1".to_string(), 10.0, 2);
        inventory.add_item("Item2".to_string(), 5.0, 4);
        
        assert_eq!(inventory.total_value(), 40.0);
    }

    #[test]
    fn test_remove_items_success() {
        let mut inventory = Inventory::new();
        inventory.add_item("Item".to_string(), 10.0, 5);
        
        let result = inventory.remove_items("Item", 3);
        assert!(result.is_ok());
        assert_eq!(inventory.find_item("Item").unwrap().quantity, 2);
    }

    #[test]
    fn test_remove_items_insufficient() {
        let mut inventory = Inventory::new();
        inventory.add_item("Item".to_string(), 10.0, 3);
        
        let result = inventory.remove_items("Item", 5);
        assert!(result.is_err());
    }
}
