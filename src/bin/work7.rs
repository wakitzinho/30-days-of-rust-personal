enum PaymentMethod {
    CreditCard(String),
    DebitCard(String),
    Cash,
    PayPal
}

fn print_payment_method(method: PaymentMethod) {
    match method {
        PaymentMethod::CreditCard(card_number) => println!("Paid with credit card: {}", card_number),
        PaymentMethod::DebitCard(card_number) => println!("Paid with debit card: {}", card_number),
        PaymentMethod::Cash => println!("Paid with cash"),
        PaymentMethod::PayPal => println!("paid with paypal"),
    }
}

use std::io;
fn main() {
    // get payment method
    println!("payment method [credit] [debit] [cash] [paypal]");
    let mut input = String::new().to_lowercase();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    println!(" ");

    if input == "credit" {
        let payment = PaymentMethod::CreditCard(String::from("1234 5309 5721 0853"));
        print_payment_method(payment)
    } else if input == "debit" {
        let payment = PaymentMethod::DebitCard(String::from("1234 5309 5721 0853"));
        print_payment_method(payment)
    } else if input == "cash" {
        let payment = PaymentMethod::Cash;
        print_payment_method(payment)
    } else if input == "paypal" {
        let payment = PaymentMethod::PayPal;
        print_payment_method(payment)
    } else {
        println!("invalid payment method")
    }
}