fn main() {
  // Initialize two String variables
  let first_string = String::from("Hello");
  let second_string2 = String::from(" World!");

  // Concatenate strings using slices and push_str
  let concatenated_string = concatenate_strings(&first_string, &second_string2);

  // Print the concatenated string
  println!("Concatenated string: {}", concatenated_string);
}

fn concatenate_strings(str1: &str, str2: &str) -> String {
  let mut result = String::new();
  result.push_str(str1);
  result.push_str(str2);
  result
}