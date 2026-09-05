use std::io;

fn main() {
    let number = 5;

    // if / else if  / else statements
    if number > 5 {
        println!("number is bigger then 5.")
    } else if 5 > number {
        println!("number is smaller then 5.")
    } else {
        println!("number is 5.")
    }

    let mut count = 0;

    // normal loop
    loop {
        count += 1;
        println!("number is: {}", count);
        if count == 10 {
            println!("Breaking the loop at {}", count);
            break;
        }
    }
    println!(" ");

    // while loop
    let mut num = 1;

    while 10 > num {
        println!("number is: {}", num);
        num += 1;
    }

    println!(" ");

    // for loop
    for number in 1..4 {
        println!("number is {}", number)
    }
    println!(" ");
    let color = "orange";

    match color {
        "green" => println!("you can go."),
        "yellow" => println!("you can go but speed up."),
        "red" => println!("you cant go."),
        _ => println!("light is buggin out"),
    }


    // work work work wok work workrworkwork wor
    let mut counter = 1;

    while 10 > counter {
        println!("counter: {}", counter);
        counter += 1
    }

    let colors = ["violet", "red", "drake", "rick ross"];
    let mut countt = 0;
    for color in colors {
        println!("color: {}", colors[countt]);
        countt += 1
    }

    // match case
    let mut greeting = String::new();

    let greeting = greeting.to_string();




    println!("number: ");
    let mut input = String::new();

    while input != "x" {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        println!("you wrote: {}", input);

        let int = input.parse::<i32>().unwrap();

        if int % 2 == 0 {
            println!("the number {} is even", int)
        } else {
            println!("the number {} is odd", int)
        }
    }

}