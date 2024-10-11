//calculator
use std::io;

fn operator(operation: &str) -> i32 {
    let (mut num1, mut num2) = (String::new(), String::new());
    
    println!("enter num1");
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1 = num1.trim().parse::<i32>().unwrap();
    
    println!("enter num2");
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2 = num2.trim().parse::<i32>().unwrap();
    
    match operation {
        "+" => return num1 + num2,
        "-" => return num1 - num2,
        "*" => return num1 * num2,
        "/" => if num2 == 0 { panic!("Can't divide by zero") } else { return num1 / num2 },
        _ => panic!("wrong operator called"),
    }
}

fn main() {
    let mut input = String::new();

    println!("Choose the operation from +, -, *, /");

    io::stdin().read_line(&mut input).expect("invalid input");
    match input.trim() {
        "+" => println!("Sum is {}", operator("+")),
        "-" => println!("Difference is {}", operator("-")),
        "*" => println!("result is {}", operator("*")),
        "/" => println!("result is {}", operator("/")),
        _ => println!("invalid operation {}", input),
    }
}
