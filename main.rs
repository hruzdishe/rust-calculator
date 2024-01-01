use std::io::stdin;

fn get_input() -> String {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Error in reading line");

    input.trim().to_string()
}

fn get_number() -> i32 {
    let input = get_input();

    input.parse().expect("Parsing error. Enter a valid number!")
}

struct InvalidOperatorError {
    details: String,
}

impl InvalidOperatorError {
    fn new(msg: &str) -> InvalidOperatorError {
        InvalidOperatorError { details: msg.to_string() }
    }
}

fn calculate(first_num: i32, second_num: i32, operator: String) -> Result<i32, InvalidOperatorError> {
    match operator.as_str() {
        "+" => Ok(first_num + second_num),
        "-" => Ok(first_num - second_num),
        "*" => Ok(first_num * second_num),
        "/" => Ok(first_num / second_num),
        operator => Err(InvalidOperatorError::new(format!("{operator} is not a valid operator").as_str()))
    }
}

fn main() {
    let first_num = get_number();
    let operator = get_input();
    let second_num = get_number();

    let result = calculate(first_num, second_num, operator.clone());
    match result {
        Ok(result) => {
            print!("{first_num} {operator} {second_num} = {result}");
        }
        Err(msg) => {
            print!("{}", msg.details);
        }
    }
}
