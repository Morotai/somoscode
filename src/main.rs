use std::{env};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let first: String;
    let operator: char;
    let second: String;

    match args.len() {
        // no arguments passed
        0 => {
            panic!("At least on argument is required. i.e: 1+1");
        }

        // 1 argument passed
        1 => {
        println!("ONE argument passed: {}", args[0]);
        // get operation and process
        let operation_char = get_operation(args[0].to_string());
        println!("Operation: {operation_char}");
        first = args[0].split(operation_char).nth(0).unwrap().to_string();
        operator = operation_char;
        second = args[0].split(operation_char).nth(1).unwrap().to_string();
   

        // Check if digits were passed
        let re = Regex::new(r"^\d[0-9]").unwrap();
        
        if re.is_match(&first) && re.is_match(&second)
        {
            let first_number = first.parse::<f32>().unwrap();
            let second_number = second.parse::<f32>().unwrap();
    
            let result = operate(operator, first_number, second_number);
            println!("{}", output(first_number, operator, second_number, result));
        } else {

            panic!("A valid digit from 0-9 must be provided.");
        }

        }

        // 2 arguments passed
        2 => {
        println!("Total Args: {}", args.len());
        panic!("At least on argument is required. i.e: 1+1");
        }

        // 3 arguments passed
        3 => {
            println!("Total Args: {}", args.len());
            first = args[0].to_string();
            operator = args[1].chars().next().unwrap();
            second = args[2].to_string();
             // Check if digits were passed
             let re = Regex::new(r"^\d[0-9]").unwrap();

             if re.is_match(&first) && re.is_match(&second)
             {
             let first_number = first.parse::<f32>().unwrap();
             let second_number = second.parse::<f32>().unwrap();
     
             println!("Operation: {operator}");
     
             let result = operate(operator, first_number, second_number);
             println!("{}", output(first_number, operator, second_number, result));
             } else {
                 panic!("A valid digit from 0-9 must be provided.");
             }
        }

        // all the other cases
        _ => {
            panic!("At least on argument is required. i.e: 1+1");
        }
    }

}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

fn get_operation(operation: String) -> char {
    if operation.contains("+") {
        '+'
    } else if operation.contains("-") {
        '-'
    } else if operation.contains("/") {
        '/'
    } else if operation.contains("*") {
        '*'
    } else if operation.contains("X") {
        '*'
    } else if operation.contains("x") {
        '*'
    } else {
        // random character
        '@'
    }
  }