use std::{fmt, fs, io, num};

///////  CUSTOM ERROR ///////
fn custom_error() {
    match read_and_parse("test.txt") {
        Ok(num) => println!("File read is complete..."),
        Err(err) => panic!("Error -- {}", err)
    }
}

enum MyCustomError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Other(String),
}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::Io(err) => write!(f, "I/O error: {}", err),
            MyCustomError::Parse(err) => write!(f, "Parse error: {}", err),
            MyCustomError::Other(message) => write!(f, "Other error: {}", message),
        }
    }
}

fn read_and_parse(filename: &str) -> Result<i32, MyCustomError> {
    let content = fs::read_to_string(filename)
        .map_err(MyCustomError::Io)?;

    let num: i32 = content.trim().parse().map_err(MyCustomError::Parse)?;

    Ok(num)
}

fn errors() {
    test_error(0); // <- zero value exception...
    test_error(1); // <- invalid value exception...
    test_error(2); // <- out of value exception...
}

fn test_error(x: i32) {
    match x {
        0 => handle_error(MyException::ZeroValueException),
        1 => handle_error(MyException::InvalidValueException),
        _ => handle_error(MyException::OutOfException),
    }
}

enum MyException {
    ZeroValueException,
    InvalidValueException,
    OutOfException,
}

fn handle_error(error: MyException) {
    match error {
        MyException::ZeroValueException => {
            println!("Value is not zero!");
        },
        MyException::InvalidValueException => {
            println!("Invalid value!");
        },
        MyException::OutOfException => {
            panic!("Out of value!");
        },
    }
}

///////  ERROR HANDLING ///////

fn error_handling() {
    let my_content = get_file_content("my_file.txt");

    match my_content {
        Ok(content) => println!("{}", content),
        Err(err) => panic!("Error --- : {}", err)
    }
}

fn divide_err(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        return Err("Denominator must be greater than zero!");
        // Err("Denominator must be greater than zero!")
        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `Result<_, &str>`
    }
    Ok(numerator / denominator)
}

fn get_file_content(file_name: &str) -> Result<String, std::io::Error> {

    let mut path: String = {
        let f = std::env::current_dir();
        match f {
            Ok(file) => file.to_str().unwrap().to_string(),
            Err(err) => panic!("{}", err)
        }
    };

    path.push_str("/src/");
    path.push_str(file_name);

    println!("File Path: {:?}", path);


    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        return None;
        // None
        // ^^^^ expected `()`, found `Option<_>`
    }
    Some(numerator / denominator)
}

///////  PANIC MACRO ///////

fn panic_select() {
    // run with `RUST_BACKTRACE=1` environment variable to display a backtrace;
    // some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // export RUST_BACKTRACE=1
    // export RUST_BACKTRACE=full

    let veggies = vec!["carrot", "celery", "tomato", "banana"];

    chop(veggies[0]); // it works...
    chop(veggies[1]); // it works...
    chop(veggies[2]); // it generate an error and halt execution...
    chop(veggies[3]); // program halted...

}


fn chop(vegetable: &str) {
    match vegetable {
        "carrot" => println!("Chopping a carrot."),
        "celery" => println!("Chopping a celery."),
        "tomato" => panic!("Don't know how to handle a tomato!"),
        _ => println!("Chopping some unknown vegetable."),
    }
}

// this function show an error after halt process on runtime,
// if never using anywhere it works don't have error...
fn make_panic() {
    panic!("This is where we hit the wall!");
}

// will this code don't show any error when build,
// but it throw a panic when runtime after build...
fn panic_example() {
    let v = vec![1, 2, 3];
    v[99]; // This line will cause a panic!
}