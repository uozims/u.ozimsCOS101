use std::io;

fn main() {
    println!("\nWelcome, User!");

    println!("Which of the following sectors do you work under?");
    println!("\nOffice Admin\nAcademic\nLawyer\nTeacher\n");

    loop {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input.");
        let b = input2.trim().to_lowercase();

        if b == "office admin" {
            office();
            break;
        } else if b == "academic" {
            academic();
            break;
        } else if b == "lawyer" {
            lawyer();
            break;
        } else if b == "teacher" {
            teacher();
            break;
        } else if b == "none" {
            println!("You are ineligible for an APS Level");
            return;
        } else {
            println!("Please make a valid input.");
            continue;
        }
    }
}

// --------------------------------------------------------------------------------------------------

fn office() {
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Director", "Director", "CEO"];

    println!("\n-------------------------\nWhat role best represents the position you hold?\n");
    for element in &office_admin {
        println!("{}", element);
    }
    println!();

    loop {
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read line.");
        let c = input3.trim().to_lowercase();

        if c == "intern" {
            println!("Your APS Level is APS 1-2.");
            return;
        }
        if c == "administrator" {
            println!("How many years have you been working as an administrator?");
            loop {
                let mut input4 = String::new();
                io::stdin().read_line(&mut input4).expect("Failed to read input.");
                let d: u8 = match input4.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if d <= 10 {
                    println!("Your APS Level is APS 3-5");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if c == "senior administrator" {
            println!("How many years have you worked as a senior administrator?");
            loop {
                let mut input5 = String::new();
                io::stdin().read_line(&mut input5).expect("Failed to read input.");
                let e: u8 = match input5.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if e <= 11 {
                    println!("Your APS Level is APS 5-8");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if c == "office director" {
            println!("How many years have you worked as an office director?");
            loop {
                let mut input6 = String::new();
                io::stdin().read_line(&mut input6).expect("Failed to read input.");
                let f: u8 = match input6.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if f <= 11 {
                    println!("Your APS Level is EL1 8-10");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if c == "director" {
            println!("How many years have you worked as a director?");
            loop {
                let mut input7 = String::new();
                io::stdin().read_line(&mut input7).expect("Failed to read input.");
                let g: u8 = match input7.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if g <= 11 {
                    println!("Your APS Level is EL2 10-13");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if c == "ceo" {
            println!("Your APS Level is SES");
            return;
        }

        println!("Please make a valid input.");
    }
}

// --------------------------------------------------------------------------------------------------

fn academic() {
    let academic = vec!["None", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];

    println!("\n-------------------------\nWhat role best represents the position you hold?\n");
    for element in &academic {
        println!("{}", element);
    }
    println!();

    loop {
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("Failed to read line.");
        let h = input8.trim().to_lowercase();

        if h == "none" {
            println!("Your APS Level is APS 1-2.");
            return;
        }
        if h == "research assistant" {
            println!("How many years have you been working as a research assistant?");
            loop {
                let mut input9 = String::new();
                io::stdin().read_line(&mut input9).expect("Failed to read input.");
                let i: u8 = match input9.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if i <= 10 {
                    println!("Your APS Level is APS 3-5");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if h == "phd candidate" {
            println!("How many years have you worked as a PhD Candidate?");
            loop {
                let mut input10 = String::new();
                io::stdin().read_line(&mut input10).expect("Failed to read input.");
                let j: u8 = match input10.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if j <= 11 {
                    println!("Your APS Level is APS 5-8");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if h == "post-doc researcher" {
            println!("How many years have you worked as a Post-Doc Researcher?");
            loop {
                let mut input11 = String::new();
                io::stdin().read_line(&mut input11).expect("Failed to read input.");
                let k: u8 = match input11.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if k <= 11 {
                    println!("Your APS Level is EL1 8-10");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if h == "senior lecturer" {
            println!("How many years have you worked as a Senior Lecturer?");
            loop {
                let mut input12 = String::new();
                io::stdin().read_line(&mut input12).expect("Failed to read input.");
                let l: u8 = match input12.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if l <= 11 {
                    println!("Your APS Level is EL2 10-13");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if h == "dean" {
            println!("Your APS Level is SES");
            return;
        }

        println!("Please make a valid input.");
    }
}

// --------------------------------------------------------------------------------------------------

fn lawyer() {
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    println!("\n-------------------------\nWhat role best represents the position you hold?\n");
    for element in &lawyer {
        println!("{}", element);
    }
    println!();

    loop {
        let mut input13 = String::new();
        io::stdin().read_line(&mut input13).expect("Failed to read line.");
        let m = input13.trim().to_lowercase();

        if m == "paralegal" {
            println!("Your APS Level is APS 1-2.");
            return;
        }
        if m == "junior associate" {
            println!("How many years have you been working as a Junior Associate?");
            loop {
                let mut input14 = String::new();
                io::stdin().read_line(&mut input14).expect("Failed to read input.");
                let n: u8 = match input14.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if n <= 10 {
                    println!("Your APS Level is APS 3-5");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if m == "associate" {
            println!("How many years have you worked as an Associate?");
            loop {
                let mut input15 = String::new();
                io::stdin().read_line(&mut input15).expect("Failed to read input.");
                let o: u8 = match input15.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if o <= 11 {
                    println!("Your APS Level is APS 5-8");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if m == "senior associate 1-2" {
            println!("How many years have you worked as a Senior Associate 1-2?");
            loop {
                let mut input16 = String::new();
                io::stdin().read_line(&mut input16).expect("Failed to read input.");
                let p: u8 = match input16.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if p <= 11 {
                    println!("Your APS Level is EL1 8-10");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if m == "senior associate 3-4" {
            println!("How many years have you worked as a Senior Associate 3-4?");
            loop {
                let mut input17 = String::new();
                io::stdin().read_line(&mut input17).expect("Failed to read input.");
                let q: u8 = match input17.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if q <= 11 {
                    println!("Your APS Level is EL2 10-13");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if m == "partner" {
            println!("Your APS Level is SES");
            return;
        }

        println!("Please make a valid input.");
    }
}

// --------------------------------------------------------------------------------------------------

fn teacher() {
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("\n-------------------------\nWhat role best represents the position you hold?\n");
    for element in &teacher {
        println!("{}", element);
    }
    println!();

    loop {
        let mut input18 = String::new();
        io::stdin().read_line(&mut input18).expect("Failed to read line.");
        let r = input18.trim().to_lowercase();

        if r == "placement" {
            println!("Your APS Level is APS 1-2.");
            return;
        }
        if r == "classroom teacher" {
            println!("How many years have you been working as a Classroom Teacher?");
            loop {
                let mut input19 = String::new();
                io::stdin().read_line(&mut input19).expect("Failed to read input.");
                let s: u8 = match input19.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if s <= 10 {
                    println!("Your APS Level is APS 3-5");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if r == "senior teacher" {
            println!("How many years have you worked as a Senior Teacher?");
            loop {
                let mut input20 = String::new();
                io::stdin().read_line(&mut input20).expect("Failed to read input.");
                let t: u8 = match input20.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if t <= 11 {
                    println!("Your APS Level is APS 5-8");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if r == "leading teacher" {
            println!("How many years have you worked as a Leading Teacher?");
            loop {
                let mut input21 = String::new();
                io::stdin().read_line(&mut input21).expect("Failed to read input.");
                let u: u8 = match input21.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if u <= 11 {
                    println!("Your APS Level is EL1 8-10");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if r == "deputy principal" {
            println!("How many years have you worked as a Deputy Principal?");
            loop {
                let mut input22 = String::new();
                io::stdin().read_line(&mut input22).expect("Failed to read input.");
                let v: u8 = match input22.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please make a valid input.");
                        continue;
                    }
                };

                if v <= 11 {
                    println!("Your APS Level is EL2 10-13");
                    return;
                } else {
                    println!("You are ineligible for APS Level.");
                    return;
                }
            }
        }

        if r == "principal" {
            println!("Your APS Level is SES");
            return;
        }

        println!("Please make a valid input.");
    }
}