use std::io;
fn main() {
    let first_number = loop {
        let mut first_number = String::new();
        println!("Enter First Number");
        io::stdin().read_line(&mut first_number).expect("fail");
        let first_number: f32 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
     };
     break first_number;
    };

    let second_number = loop {
        let mut second_number = String::new();
        println!("Enter Second Number");
        io::stdin().read_line(&mut second_number).expect("fail");
        let second_number: f32 = match second_number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
         };
         break second_number;
        };

    loop {
    let mut operation = String::new();

        println!("Please select operation:");
        println!("Enter 1 for '+'");
        println!("Enter 2 for '*'");
        println!("Enter 3 for '-'");
        println!("Enter 4 for '/'");

        io::stdin().read_line(&mut operation).expect("Invalid Operation");


        if operation.trim() == "1" {
            let answer = first_number + second_number;
            println!("ANSWER: {}", answer);
            break;
        }
        else if operation.trim() == "2" {
            let answer = first_number * second_number;
            println!("ANSWER: {}", answer);
            println!("Multiplication");
            break;
        }
        else if operation.trim() == "3" {
            let answer = first_number - second_number;
            println!("ANSWER: {}", answer);
            println!("Subtraction");
            break;
        }
        else if operation.trim() == "4" {
            let answer = first_number / second_number;
            println!("ANSWER: {}", answer);
            println!("Division");
            break;
        }else{
            println!("Invalid Operation!");
        }
    };
}