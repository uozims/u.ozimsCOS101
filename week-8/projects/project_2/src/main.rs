use std::io;

fn main() {
    let mut applicants: Vec<(String, u8, u8)> = Vec::new();
    println!("\nWelcome, User!\nIs there a *definite* or *indefinite* number of applicants today?");
    let yn: String = loop {
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read line.");
        let a = input1.trim().to_lowercase();
        if a == "definite" {
            break a.to_string();
        } else if a == "indefinite" {
            break a.to_string();
        } else {
            println!("Please answer Definite or Indefinite.");
            continue;
        }
    };

    if yn == "definite" {
        definite(&mut applicants);
    } else if yn == "indefinite" {
        indefinite(&mut applicants);
    }
}

fn definite(applicants: &mut Vec<(String, u8, u8)>) {
    println!("\nHow many applicants are being interviewed today?");
    let app_no: u8 = loop {
        let mut no = String::new();
        io::stdin().read_line(&mut no).expect("Failed to read line.");
        match no.trim().parse::<u8>() {
            Ok(num) => break num,
            Err(_) => println!("Please make a valid input."),
        }
    };

    println!("\nBeginning interview session...");
    for _ in 0..app_no {
        println!("\nHi! What is your name?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line.");
        let applicant_name = input2.trim();

        println!("What's your age?");
        let applicant_age: u8 = loop {
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("Failed to read line.");
            match input3.trim().parse::<u8>() {
                Ok(num) => break num,
                Err(_) => println!("Please make a valid input."),
            }
        };

        println!("How many years have you worked as a developer?");
        let applicant_exp: u8 = loop {
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("Failed to read line.");
            match input4.trim().parse::<u8>() {
                Ok(num) => break num,
                Err(_) => println!("Please make a valid input."),
            }
        };

        applicants.push((applicant_name.to_string(), applicant_age, applicant_exp));
    }

    let mut sorted = applicants.clone();
    sorted.sort_by(|a, b| b.2.cmp(&a.2));

    println!("\n--- Applicants in Order of Experience ---");
    for (name, age, exp) in sorted {
        println!("Name: {}, Age: {}, Experience: {} years", name, age, exp);
    }
}

fn indefinite(applicants: &mut Vec<(String, u8, u8)>) {
    loop {
        println!("\nHi! What is your name?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line.");
        let applicant_name = input2.trim();

        println!("What's your age?");
        let applicant_age: u8 = loop {
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("Failed to read line.");
            match input3.trim().parse::<u8>() {
                Ok(num) => break num,
                Err(_) => println!("Please make a valid input."),
            }
        };

        println!("How many years have you worked as a developer?");
        let applicant_exp: u8 = loop {
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("Failed to read line.");
            match input4.trim().parse::<u8>() {
                Ok(num) => break num,
                Err(_) => println!("Please make a valid input."),
            }
        };

        applicants.push((applicant_name.to_string(), applicant_age, applicant_exp));

        println!("Your information has been collected! *End* interview session, or *Continue* session for more applicants?");
        let session: String = loop {
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("Failed to read line.");
            let choice = input5.trim().to_lowercase();
            if choice == "end" || choice == "continue" {
                break choice.to_string();
            } else {
                println!("Please answer with End or Continue.");
                continue;
            }
        };

        if session == "end" {
            break;
        } else if session == "continue" {
            continue;
        }
    }

    let mut sorted = applicants.clone();
    sorted.sort_by(|a, b| b.2.cmp(&a.2));

    println!("\n--- Applicants in Order of Experience ---");
    for (name, age, exp) in sorted {
        println!("Name: {}, Age: {}, Experience: {} years", name, age, exp);
    }
}