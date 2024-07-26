// Define a struct to hold the filter condition
struct FilterCondition {
    min_value: i32,
}

// Implement the `is_match` method to check if an item matches the condition
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        item >= &self.min_value
    }
}

// Define the `custom_filter` function
fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_vec = Vec::new();
    for item in collection {
        if condition.is_match(item) {
            filtered_vec.push(*item);
        }
    }
    filtered_vec
}

fn main() {
    // Create a vector and the filter condition
    let numbers = vec![21, 15, 13, 7, 10, 4];
    let filter_condition = FilterCondition { min_value: 13 };

    // Filter the vector and print the result
    let filtered_numbers = custom_filter(&numbers, &filter_condition);
    println!("Filtered numbers: {:?}", filtered_numbers);
}