use std::io;
use std::io::Write;

fn main() {
    let mut student_info: Vec<(String, String, String, u16)> = Vec::new();
    println!("\nWelcome, User!\nIs there a *definite* or *indefinite* number of student entries today?");
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
    } else if yn == "indefinite" {
        indefinite(&mut student_info);
    } 
}

fn definite(student_info: &mut Vec<(String, String, String, u16)>) {
    println!("\nHow many students entries are being made today?");
    let app_no: u8 = loop {
        let mut no = String::new();
        io::stdin().read_line(&mut no).expect("Failed to read line.");
        match no.trim().parse::<u8>() {
            Ok(num) => break num,
            Err(_) => println!("Please make a valid input."),
        }
    };

    println!("\nBeginning entry session...");
    for _ in 0..app_no {
        println!("\nInput Student Name:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line.");
        let student_name = input2.trim();

        println!("Input Student Department:");
        let mut student_dept = String::new();
        let mut dept_acronym = "";

        loop {
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("Failed to read line.");
            let dept = input3.trim();

            let acronym = match dept.to_lowercase().as_str() {
                "accounting" => "ACC",
                "computer" | "computer science" => "CSC",
                "economics" => "ECO",
                "electrical" | "electrical engineering" | "electronic engineering" | "electrical and electronic engineering" | "electronic and electrical engineering" => "EEE",
                "mechanical" | "mechanical engineering" => "MEE",
                "software" | "software engineering" => "SEN",
                "mechatronics" | "mechatronics engineering" => "MCT",
                "data" | "data science" => "DS",
                "business" | "business administration" | "business admin" => "BUA",
                "finance" => "FIN",
                "film" | "film studies" | "film and multimedia" | "film and multimedia studies" => "FMM",
                "mass comm" | "mass comms" | "mass communication" | "mass communications" => "MCM",
                "strategic comm" | "strategic communication" | "strategic communications" | "strategic comms" => "STC",
                "information science and media studies" | "information and media" | "info science" | "information" | "information and media studies" | "information science and media" => "ISMS",
                _ => {
                    println!("Please input a valid department");
                    continue; 
                }
            };

            student_dept = dept.to_string();
            dept_acronym = acronym;
            break;
        }

        println!("Input Student Matric No:");
        let matric_no: u64 = loop {
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("Failed to read line.");
            match input4.trim().parse::<u64>() {
                Ok(num) => break num,
                Err(_) => println!("Please make a valid input."),
            }
        };
        let compl_matric_no = dept_acronym.to_string() + &matric_no.to_string();

        println!("Input Student Level(100, 200, 300, 400, 500):");
        let student_level: u16 = loop {
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("Failed to read line.");
            match input5.trim().parse::<u16>() {
                Ok(num) => if num == 100 || num == 200 || num == 300 || num == 400 || num == 500 {
                    break num;
                } else {
                    println!("Please make a valid input (100, 200, 300, 400 or 500).");
                },
                Err(_) => println!("Please make a valid input."),
            }
        };

        student_info.push((
            student_name.to_string(),
            compl_matric_no.to_string(),
            student_dept.to_string(),
            student_level,
        ));
    }

    let mut file = std::fs::File::create("PAU-SMIS Student Records.csv").expect("Create failed");
    file.write_all("Student Name,Matric. Number,Department,Level\n".as_bytes()).expect("write failed");

    println!("\nStudent Index:");
    for (name, matric, dept, level) in student_info {
        println!("\nName: {} , Matric Number: {} , Department: {} , Level: {}", name, matric, dept, level);

        file.write_all(name.as_bytes()).expect("Write failed");
        file.write_all(",".as_bytes()).expect("Write failed");
        file.write_all(matric.as_bytes()).expect("Write failed");
        file.write_all(",".as_bytes()).expect("Write failed");
        file.write_all(dept.as_bytes()).expect("Write failed");
        file.write_all(",".as_bytes()).expect("Write failed");
        file.write_all(level.to_string().as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }

    println!("\nData written to file.");
}

fn indefinite(student_info: &mut Vec<(String, String, String, u16)>) {
    loop {
        println!("\nBeginning entry session...");

        println!("\nInput Student Name:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line.");
        let student_name = input2.trim();

        println!("Input Student Department:");
        let mut student_dept = String::new();
        let mut dept_acronym = "";

        loop {
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("Failed to read line.");
            let dept = input3.trim();

            let acronym = match dept.to_lowercase().as_str() {
                "accounting" => "ACC",
                "computer" | "computer science" => "CSC",
                "economics" => "ECO",
                "electrical" | "electrical engineering" | "electronic engineering" | "electrical and electronic engineering" | "electronic and electrical engineering" => "EEE",
                "mechanical" | "mechanical engineering" => "MEE",
                "software" | "software engineering" => "SEN",
                "mechatronics" | "mechatronics engineering" => "MCT",
                "data" | "data science" => "DS",
                "business" | "business administration" | "business admin" => "BUA",
                "finance" => "FIN",
                "film" | "film studies" | "film and multimedia" | "film and multimedia studies" => "FMM",
                "mass comm" | "mass comms" | "mass communication" | "mass communications" => "MCM",
                "strategic comm" | "strategic communication" | "strategic communications" | "strategic comms" => "STC",
                "information science and media studies" | "information and media" | "info science" | "information" | "information and media studies" | "information science and media" => "ISMS",
                _ => {
                    println!("Please input a valid department");
                    continue; 
                }
            };

            student_dept = dept.to_string();
            dept_acronym = acronym;
            break;
        }

        println!("Input Student Matric No:");
        let matric_no: u64 = loop {
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("Failed to read line.");
            match input4.trim().parse::<u64>() {
                Ok(num) => break num,
                Err(_) => println!("Please make a valid input."),
            }
        };
        let compl_matric_no = dept_acronym.to_string() + &matric_no.to_string();

        println!("Input Student Level(100, 200, 300, 400, 500):");
        let student_level: u16 = loop {
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("Failed to read line.");
            match input5.trim().parse::<u16>() {
                Ok(num) => if num == 100 || num == 200 || num == 300 || num == 400 || num == 500 {
                    break num;
                } else {
                    println!("Please make a valid input (100, 200, 300, 400 or 500).");
                },
                Err(_) => println!("Please make a valid input."),
            }
        };

        student_info.push((
            student_name.to_string(),
            compl_matric_no.to_string(),
            student_dept.to_string(),
            student_level,
        ));

        println!("Student information has been collected! *End* entry session, or *Continue* session for more entries?");
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

    let mut file = std::fs::File::create("PAU-SMIS Student Records.csv").expect("Create failed");
    file.write_all("Student Name,Matric. Number,Department,Level\n".as_bytes()).expect("write failed");

    println!("\nStudent Index:");
    for (name, matric, dept, level) in student_info {
        println!("\nName: {} , Matric Number: {} , Department: {} , Level: {}", name, matric, dept, level);

        file.write_all(name.as_bytes()).expect("Write failed");
        file.write_all(",".as_bytes()).expect("Write failed");
        file.write_all(matric.as_bytes()).expect("Write failed");
        file.write_all(",".as_bytes()).expect("Write failed");
        file.write_all(dept.as_bytes()).expect("Write failed");
        file.write_all(",".as_bytes()).expect("Write failed");
        file.write_all(level.to_string().as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }

    println!("\nData written to file.");
}