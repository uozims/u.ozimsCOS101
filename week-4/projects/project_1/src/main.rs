//Rust program to find the roots of a quadratic equation
use std::io;

fn main() {
    println!("\nFinding the roots of a quadratic equation: ");

    //input a
    println!("\nEnter your 'a' value: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Invalid input: a must be a real number");
     if a == 0.0 {
        println!("Invalid input: a cannot be 0");
        return;
    }
    println!("Value of a is {}",a); 

    //input b
    println!("\nEnter your 'b' value: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Invalid input: b must be a real number");
    println!("Value of b is {}",b);

    //input c
    println!("\nEnter your 'c' value: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Invalid input: c must be a real number");
    println!("Value of c is {}",c);

    //the equation
    println!("\nTherefore, the equation is {}x^2 {}x {} = 0",a,b,c);

    //discriminant
    let d:f32 = b*b - (4.0*a*c);
    println!("\nDiscriminant (D) is {}",d);
    if d > 0.0 {
        println!("\nThe quadratic equation has two distinct roots");
        let root1 = (-b + d.sqrt()) / 2.0*a;
        let root2 = (-b - d.sqrt()) / 2.0*a;
        println!("\nTherefore,");
        println!("First Root is {}",root1);
        println!("Second Root is {}",root2);
    }
    else if d == 0.0 {
        println!("\nThe quadratic equation has two real and equal roots");
        let root1 = (-b + d.sqrt()) / 2.0*a;
        println!("\nTherefore,");
        println!("Root 1 is equal to Root 2 which is {}",root1);
    }
    else {
        println!("\nThe quadratic equation has no real roots");
        let real = -b / (2.0*a);
        let imaginary = d.abs().sqrt() / (2.0*a);
        println!("\nTherefore,");
        println!("First complex root is {} + {}i",real,imaginary);
        println!("Second complex root is {} - {}i",real,imaginary);
    }

}
