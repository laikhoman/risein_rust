enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    println!("Welcome to the rust calculator!");

    let num1 = read_input("Input the first number: ");
    let op = read_operation("Input the operation (+, -, *, /): ");
    let num2 = read_input("Input the second number: ");

    let operation = match op.as_str() {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => panic!("Invalid operation!"),
    };

    let result = calculate(operation);
    println!("The result is: {}", result);
}

fn read_input(message: &str) -> f64 {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input!");

    input.trim().parse::<f64>().expect("Invalid number!")
}

fn read_operation(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input!");

    input.trim().to_string()
}