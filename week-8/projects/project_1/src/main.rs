use std::io;

fn main() {
    let _office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let _academic = vec!["None","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let _lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let _teacher = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];


    println!("\nWelcome, User!\nAre you an applicant or admin?\n");
    loop{
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let a = input1.trim();

    if a == "admin" || a == "Admin" {
        return;
    }
    if a == "applicant" || a == "Applicant" {
        break;
    }
    else {
        println!("Please make a valid input.");
        continue;
    }
    
}   
    println!("\nWhich of the following sectors do you work under?");
    println!("Office Admin,\nAcademic,\nLawyer,\nTeacher\n");
    
    loop {
        let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input.");
    let b = input2.trim();

    if b == "Office Admin" || b == "office admin" {
        office()
    }
    else if b == "academic" || b == "Academic" {
        academic()
    }
    else if b == "lawyer" || b == "Lawyer" {
        lawyer()
    }
    else if b == "Teacher" || b == "teacher" {
        teacher()
    }
    else if b == "none" || b == "None" {
        println!("You are ineligible for an APS Level");
        return;
    }
    else {
        println!("Please make a valid input.");
        continue;
    }
    }



}



// --------------------------------------------------------------------------------------------------------------------------------------------------------




fn office() {
    
    let _office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];

    println!("\n-------------------------\nWhat ur power lvl\n");
    for element in &_office_admin {
        println!("{}",element);
    }
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line.");
    let c = input3.trim();

    if c == "intern" || c == "Intern" {
        println!("Your APS Level is APS 1-2.");
    }
    if c == "administrator" || c == "Administrator" {
        println!("How many years have you been working as an administrator?");
        
        'admin_loop: loop {
            let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input.");
        let d:u8 = match input4.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if d >= 0 && d <=10 {
            println!("Your APS Level is APS 3-5");
            return;
        }
        if d > 11 || d < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

        

    }

    if c == "Senior Administartor" || c == "senior administrator" {
        println!("How many years have you worked as a senior administrator?");
        'senior_admin_loop: loop {
            let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("Failed to read input.");
        let e:u8 = match input5.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if e >= 0 && e <=11 {
            println!("Your APS Level is APS 5-8");
            return;
        }
        if e > 12 || e < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

    }

    if c == "office director" || c == "Office Director" || c == "Office director" {
          println!("How many years have you worked as an office director?");
        'office_director_loop: loop {
            let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("Failed to read input.");
        let f:u8 = match input6.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if f >= 0 && f <=11 {
            println!("Your APS Level is EL1 8-10");
            return;
        }
        if f > 12 || f < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if c == "Director" || c == "director" {
          println!("How many years have you worked as a director?");
        'director_loop: loop {
            let mut input7 = String::new();
        io::stdin().read_line(&mut input7).expect("Failed to read input.");
        let g:u8 = match input7.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if g >= 0 && g <=11 {
            println!("Your APS Level is EL2 10-13");
            return;
        }
        if g > 12 || g < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if c == "CEO" || c == "ceo" {
        println!("Your APS Level is SES");
    }
}





// -------------------------------------------------------------------------------------------------------------------------------------------------------





fn academic() {

    let _academic = vec!["None","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    println!("\n-------------------------\nWhat ur power lvl\n");
    for element in &_academic {
        println!("{}",element);
    }
    let mut input8 = String::new();
    io::stdin().read_line(&mut input8).expect("Failed to read line.");
    let h = input8.trim();

    if h == "none" || h == "None" {
        println!("Your APS Level is APS 1-2.");
    }
    if h == "Research Assistant" || h == "research assistant" || h == "Research assistant" {
        println!("How many years have you been working as a research assistant?");
        
        'research_assistant_loop: loop {
            let mut input9 = String::new();
        io::stdin().read_line(&mut input9).expect("Failed to read input.");
        let i:u8 = match input9.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if i >= 0 && i <=10 {
            println!("Your APS Level is APS 3-5");
            return;
        }
        if i > 11 || i < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

        

    }

    if h == "PhD Candidate" || h == "PhD candidate" {
        println!("How many years have you worked as a PhD Candidate?");
        'senior_admin_loop: loop {
            let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("Failed to read input.");
        let j:u8 = match input10.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if j >= 0 && j <=11 {
            println!("Your APS Level is APS 5-8");
            return;
        }
        if j > 12 || j < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

    }

    if h == "Post-Doc Researcher" || h == "Post-doc researcher" || h == "Post-doc researcher" || h == "post-doc researcher" || h == "Post-Doc researcher" {
          println!("How many years have you worked as a Post-Doc Researcher?");
        'post_doc_loop: loop {
            let mut input11 = String::new();
        io::stdin().read_line(&mut input11).expect("Failed to read input.");
        let k:u8 = match input11.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if k >= 0 && k <=11 {
            println!("Your APS Level is EL1 8-10");
            return;
        }
        if k > 12 || k < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if h == "Senior Lecturer" || h == "senior lecturer" || h == "Senior lecturer" {
          println!("How many years have you worked as a Senior Lecturer?");
        'senior_lecturer_loop: loop {
            let mut input12 = String::new();
        io::stdin().read_line(&mut input12).expect("Failed to read input.");
        let l:u8 = match input12.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if l >= 0 && l <=11 {
            println!("Your APS Level is EL2 10-13");
            return;
        }
        if l > 12 || l < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if h == "Dean" || h == "dean" {
        println!("Your APS Level is SES");
    }
}





// ----------------------------------------------------------------------------------------------------------------------------------------------------------------






fn lawyer() {

    let _lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    println!("\n-------------------------\nWhat ur power lvl\n");
    for element in &_lawyer {
        println!("{}",element);
    }
    let mut input13 = String::new();
    io::stdin().read_line(&mut input13).expect("Failed to read line.");
    let m = input13.trim();


    if m == "paralegal" || m == "Paralegal" {
        println!("Your APS Level is APS 1-2.");
    }
    if m == "Junior Associate" || m == "Junior associate" || m == "junior associate" {
        println!("How many years have you been working as a Junior Associate?");
        
        'junior_loop: loop {
            let mut input14 = String::new();
        io::stdin().read_line(&mut input14).expect("Failed to read input.");
        let n:u8 = match input14.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if n >= 0 && n <=10 {
            println!("Your APS Level is APS 3-5");
            return;
        }
        if n > 11 || n < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

        

    }

    if m == "Associate" || m == "associate" {
        println!("How many years have you worked as an Associate?");
        'associate_loop: loop {
            let mut input15 = String::new();
        io::stdin().read_line(&mut input15).expect("Failed to read input.");
        let o:u8 = match input15.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if o >= 0 && o <=11 {
            println!("Your APS Level is APS 5-8");
            return;
        }
        if o > 12 || o < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

    }

    if m == "Senior Associate 1-2" || m == "Senior associate 1-2" || m == "senior associate 1-2" {
          println!("How many years have you worked as a Senior Associate 1-2?");
        'senior_associate_1_2_loop: loop {
            let mut input16 = String::new();
        io::stdin().read_line(&mut input16).expect("Failed to read input.");
        let p:u8 = match input16.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if p >= 0 && p <=11 {
            println!("Your APS Level is EL1 8-10");
            return;
        }
        if p > 12 || p < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if m == "Senior Associate 3-4" || m == "Senior associate 3-4" || m == "senior associate 3-4" {
          println!("How many years have you worked as a Senior Associate 3-4?");
        'director_loop: loop {
            let mut input17 = String::new();
        io::stdin().read_line(&mut input17).expect("Failed to read input.");
        let q:u8 = match input17.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if q >= 0 && q <=11 {
            println!("Your APS Level is EL2 10-13");
            return;
        }
        if q > 12 || q < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if m == "Partner" || m == "partner" {
        println!("Your APS Level is SES");
    }
}







// ---------------------------------------------------------------------------------------------------------------------------------------------------------------









fn teacher() {
    let _teacher = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];
    println!("\n-------------------------\nWhat ur power lvl\n");
    for element in &_teacher {
        println!("{}",element);
    }
    let mut input18 = String::new();
    io::stdin().read_line(&mut input18).expect("Failed to read line.");
    let r = input18.trim();


    if r == "Placement" || r == "placement" {
        println!("Your APS Level is APS 1-2.");
    }
    if r == "Classroom Teacher" || r == "Classroom teacher" || r == "classroom teacher" {
        println!("How many years have you been working as a Classroom Teacher?");
        
        'classroom_teacher_loop: loop {
            let mut input19 = String::new();
        io::stdin().read_line(&mut input19).expect("Failed to read input.");
        let s:u8 = match input19.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if s >= 0 && s <=10 {
            println!("Your APS Level is APS 3-5");
            return;
        }
        if s > 11 || s < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

        

    }

    if r == "Senior Teacher" || r == "Senior teacher" || r == "senior teacher" {
        println!("How many years have you worked as a Senior Teacher?");
        'associate_loop: loop {
            let mut input20 = String::new();
        io::stdin().read_line(&mut input20).expect("Failed to read input.");
        let t:u8 = match input20.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if t >= 0 && t <=11 {
            println!("Your APS Level is APS 5-8");
            return;
        }
        if t > 12 || t < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }

    }

    if r == "Leading Teacher" || r == "Leading teacher" || r == "leading teacher" {
          println!("How many years have you worked as a Leading Teacher?");
        'leading_teacher_loop: loop {
            let mut input21 = String::new();
        io::stdin().read_line(&mut input21).expect("Failed to read input.");
        let u:u8 = match input21.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if u >= 0 && u <=11 {
            println!("Your APS Level is EL1 8-10");
            return;
        }
        if u > 12 || u < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if r == "Deputy Principal" || r == "Deputy principal" || r == "deputy principal" {
          println!("How many years have you worked as a Senior Associate 3-4?");
        'deputy_principal_loop: loop {
            let mut input22 = String::new();
        io::stdin().read_line(&mut input22).expect("Failed to read input.");
        let v:u8 = match input22.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid input.");
                continue;
            }
        };

        if v >= 0 && v <=11 {
            println!("Your APS Level is EL2 10-13");
            return;
        }
        if v > 12 || v < 0 {
            println!("You are ineligible for APS blah blah blah");
        }
        }
    }

    if r == "Principal" || r == "principal" {
        println!("Your APS Level is SES");
    }
}
