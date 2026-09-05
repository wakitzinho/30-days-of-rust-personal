fn greet() {
    println!("hello there")
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_odd(number: i32) -> bool {
    if number % 2 == 0 {
        println!("the number is even");
        false
    } else {
        println!("the number is odd");
        true
    }
}

fn main() {
    greet();
    let sum = add(53, 5);
    println!("the sum is {}", sum);

    let number = is_odd(10);
}