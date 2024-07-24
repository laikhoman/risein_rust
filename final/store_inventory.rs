use std::io;
use bcrypt::{hash_password, generate_salt};
// Define the data structures that will represent your products and inventory.
// Define a struct for representing a product.
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

// Define a struct to hold the inventory.
struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    // Function to create a new empty inventory.
    fn new() -> Inventory {
        Inventory {
            products: Vec::new(),
        }
    }

    // Function that adds a new product to the inventory.
    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    // Function that edits an existing product in the inventory.
    fn edit_product(&mut self, index: usize, product: Product) -> Result<(), String> {
        if index < self.products.len() {
            self.products[index] = product;
            Ok(())
        } else {
            Err("Invalid index".to_string())
        }
    }

    // Function that deletes a product from the inventory.
    fn delete_product(&mut self, index: usize) -> Result<Product, String> {
        if index < self.products.len() {
            Ok(self.products.remove(index))
        } else {
            Err("Invalid index".to_string())
        }
    }

    // Function that generates a report of the inventory.
    fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Inventory Report\n");
        report.push_str("Name\tDescription\tPrice\tQuantity\n");
        for product in &self.products {
            report.push_str(&format!(
                "{}\t{}\t{}\t{}\n",
                product.name, product.description, product.price, product.quantity
            ));
        }
        report
    }

    // Function that prompts the user for a product index and calls the delete_product function.
    fn delete_product_prompt(&mut self) -> Result<(), String> {
        println!("Enter the index of the product to delete:");
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            return Err("Failed to read input".to_string());
        }
        let index = match input.trim().parse::<usize>() {
            Ok(idx) => idx,
            Err(_) => return Err("Invalid index".to_string()),
        };
        match self.delete_product(index) {
            Ok(product) => {
                println!("Deleted product: {:?}", product);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

     Function that creates a hashed password from a plain text password.
    fn create_hashed_password(password: &str) -> Result<String, String> {
        let salt = generate_salt()?;
        let hash = hash_password(password, &salt)?;
        Ok(hash)
    }
} 

// Function to read user input.
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Function to parse a floating point number from user input.
fn parse_float(input: &str) -> Result<f64, String> {
    input
        .parse()
        .map_err(|_| "Invalid input. Please enter a valid number.".to_string())
}

// Function to parse an unsigned integer from user input.
fn parse_uint(input: &str) -> Result<u32, String> {
    input
        .parse()
        .map_err(|_| "Invalid input. Please enter a valid number.".to_string())
}

// Function to create a new product from user input.
fn create_product() -> Product {
    let name = read_input("Enter product name:");
    let description = read_input("Enter product description:");
    let price_str = read_input("Enter product price:");
    let price = parse_float(&price_str).expect("Invalid price");
    let quantity_str = read_input("Enter product quantity:");
    let quantity = parse_uint(&quantity_str).expect("Invalid quantity");

    Product {
        name,
        description,
        price,
        quantity,
    }
}

// Main function.
fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("\n===== Rusty Store Inventory Management System =====");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. To Implement Error Handling...");
        println!("6. To Add Security...");
        println!("7. Exit");

        let choice = read_input("Enter your choice:");

        match choice.as_str() {
            "1" => {
                let product = create_product();
                inventory.add_product(product);
            }
            "2" => {
                let index_str = read_input("Enter index of product to edit:");
                let index = match parse_uint(&index_str) {
                    Ok(idx) => idx as usize,
                    Err(err) => {
                        println!("Error: {}", err);
                        continue;
                    }
                };
                let product = create_product();
                if let Err(err) = inventory.edit_product(index, product) {
                    println!("Error: {}", err);
                }
            }
            "3" => {
                let index_str = read_input("Enter index of product to delete:");
                let index = parse_uint(&index_str).expect("Invalid index");
                if let Err(err) = inventory.delete_product(index as usize) {
                    println!("Error: {}", err);
                }
            }
            "4" => {
                println!("{}", inventory.generate_report());
            }
            "5" => {
                inventory
                    .delete_product_prompt()
                    .expect("Failed to delete product");
            }
            "6" => {
                let password = read_input("Enter password to hash:");
                match Inventory::create_hashed_password(&password) {
                    Ok(hash) => println!("Hashed password: {}", hash),
                    Err(err) => println!("Error: {}", err),
                }
            }
            "7" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 7.");
            }
        }
    }
}