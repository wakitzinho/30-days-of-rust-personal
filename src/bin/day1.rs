fn main() {
    let x = 2;
    let y = 3;
    println!("hello world!");

    let mut z = 10;
    z = 15;
    println!("{}", z);

    let mut answer = 0;
    println!("before: {}", answer);
    answer += y;
    println!("+ y: {}", answer);
    answer += x;
    println!("+ x: {}", answer);
    answer += 500;
    print!("{}", answer);
    println!("");

    let int = 5;
    let float = 5.5;
    let bool = true;
    let char = "R";
    let tuple = (10, 12, 14);
    let array = [1, 2, 3];

    println!("int: {} float: {} bool: {} character: {} tuple: {:?} array: {:?}", int, float, bool, char, tuple, array);

    
}
