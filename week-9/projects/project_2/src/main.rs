use std::io::Write;
use std::io;

fn main() {
    let mut student_info: Vec<(String, String, String, u16)> = Vec::new();
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
        definite(&mut student_info);
    } /*else if yn == "indefinite" {
        indefinite(&mut student_info);
    } */
}





fn definite(student_info: &mut Vec<(String, String, String, u16)>) {
    println!("\nHow many students blah blah blah input sha");
    let app_no: u8 = loop {
        let mut no = String::new();
        io::stdin().read_line(&mut no).expect("Failed to read line.");
        match no.trim().parse::<u8>() {
            Ok(num) => break num,
            Err(_) => println!("Please make a valid input."),
        }
    };

    println!("\nBeginning application session...");
    for _ in 0..app_no {
        println!("\nInput Student Name:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line.");
        let student_name = input2.trim();

        println!("Input Student Department:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read line.");
        let student_dept = input3.trim();

        println!("Input Student Matric No:");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read line.");
        let student_matric_no = input4.trim();

        println!("Input Student Level(100, 200, 300, 400, 500):");
        let student_level: u16 = loop {
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("Failed to read line.");
            match input5.trim().parse::<u16>() {
                Ok(num) => if num == 100 || num == 200 || num == 300 || num == 400 || num == 500 {
                    break num;
                }
                else {
                    println!("Please make a valid input (100, 200, 300, 400 or 500).");
                }
                Err(_) => println!("Please make a valid input."),
            }
            
        };

        student_info.push((student_name.to_string(), student_matric_no.to_string(), student_dept.to_string(), student_level));
    }
    for (name,matric,dept,level) in student_info.iter() {
        println!("{} {} {} {}",name,matric,dept,level);
    }
    let mut file = std::fs::File::create("PAU-SMIS Student Records.csv").expect("Create failed");
    file.write_all("Student Name,Matric. Number,Department,Level\n".as_bytes()).expect("write failed");
    for (name,matric,dept,level) in student_info.iter() {
       let row = format!("{},{},{},{}\n", name, matric, dept, level);
        file.write_all(row.as_bytes()).expect("Write failed");
    }

    println!("\nData written to file.");


}
