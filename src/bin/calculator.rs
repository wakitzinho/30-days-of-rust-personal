use std::io;
fn main() {

    // first number
    println!("number 1: ");
    let mut number1 = String::new();
    number1.clear();
    io::stdin().read_line(&mut number1).unwrap();
    let number1 = number1.trim();

    // operator
    println!("operation? [+] [-] [/] [*]: ");
    let mut oper = String::new();
    oper.clear();
    io::stdin().read_line(&mut oper).unwrap();
    let oper = oper.trim();

    // second number
    println!("number: ");
    let mut number2 = String::new();
    number2.clear();
    io::stdin().read_line(&mut number2).unwrap();
    let number2 = number2.trim();

    // handling
    let num1 = number1.parse::<i64>().unwrap();
    let num2 = number2.parse::<i64>().unwrap();

    if oper == "+" {
        let answer = num1 + num2;
        println!("answer is: {}", answer)
    } else if oper == "-" {
        let answer = num1 - num2;
        println!("answer is: {}", answer)
    } else if oper == "/" {
        let answer = num1 / num2;
        println!("answer is: {}", answer)
    } else if oper == "*" {
        let answer = num1 * num2;
        println!("answer is: {}", answer)
    } else {
        println!("invalid operator! ")
    }
}