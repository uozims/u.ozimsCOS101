//Rust program
use std::io;

fn main() {
    println!("\nEmployee Incentive Calculator");

    //employee info
    println!("\nPlease input employee's age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let a:u8 = age.trim().parse().expect("Invalid input: Age must be a positive whole number");
    if a < 18 {
        println!("\nEmployee must be atleast 18 years old");
        return;
    } else if a >=80 {
        println!("\nEmployee cannot be up to 80 years old");
        return;
    }

    //employee experience
    println!("\nIs the employee experienced? Answer true or false");
    let mut exp = String::new();
    io::stdin().read_line(&mut exp).expect("Failed to read input");
    let e:bool = exp.trim().parse().expect("Invalid input: Must answer true or false");
    if e == false && a >= 18 && a < 28 {
        println!("\nEmployee incentive is ₦100,000");
    } else if e == true && a >18 && a < 28 {
        println!("\nEmployee incentive is ₦1,300,000");
    } else if e == true && a >= 28 && a <= 29 {
        println!("\nNo incentive");//what happened to 28 and 29... 
    } else if e == true && a >= 30 && a <= 39 {
        println!("\nEmployee incentive is ₦1,480,000");
    } else if e == true && a >= 40 && a < 80 {
        println!("\nEmployee incentive is ₦1,560,000");
    }
    else if e == false && a >=28 && a < 80 {
        println!("Employee must be experienced if atleast 28 years old");
        return;
    } 
}
