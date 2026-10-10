use std::io;

fn trapezium() {
    println!("\nPlease enter the following parameters:");
    println!("\nBase 1:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let a:f32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    println!("\nBase 2:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    println!("\nHeight:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = match input3.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    let area = c / 2.0 *(a + b);
    println!("\nArea of the given trapezium is {}",area);
}

fn rhombus() {
     println!("\nPlease enter the following parameters:");
    println!("\nDiagonal 1:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let a:f32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    println!("\nDiagonal 2:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    let area = 0.5 * a * b;
    println!("\nArea of the given rhombus is {}",area);
}

fn parallelogram () {
     println!("\nPlease enter the following parameters:");
    println!("\nBase:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let a:f32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    println!("\nAltitude:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    let area = a * b;
    println!("\nArea of the given parallelogram is {}",area);
}

fn cube () {
     println!("\nPlease enter the following parameters:");
    println!("\nSide:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let a:f32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    let surface_area = 6.0 * a * a;
    println!("\nSurface area of the given cube is {}",surface_area);
}

fn cylinder () {
     println!("\nPlease enter the following parameters:");
    println!("\nRadius:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let a:f32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    println!("\nHeight:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid input.");
            return;
        }
    };
    let volume = (22.0/7.0) * a * a * b;
    println!("\nVolume of the given cylinder is {}",volume);
}




fn main() {
    println!("\nWelcome user!");
    println!("\nWhich shape would you like to find the area/volume of?");
    println!("Trapezium (Area),\nRhombus (Area),\nParallelogram (Area),\nCube (Surface Area),\nCylinder (Volume)");
    loop {
        let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input.");
    let ch = choice.trim();

    if ch == "trapezium" || ch == "Trapzium" {
        trapezium();
        break;
    } else if ch == "rhombus" || ch == "Rhombus" {
        rhombus();
        break;
    } else if ch == "parallelogram" || ch == "Parallelogram" {
        parallelogram();
        break;
    } else if ch == "cube" || ch == "Cube" {
        cube();
        break;
    } else if ch == "cylinder" || ch == "Cylinder" {
        cylinder();
        break;
    } else {
        println!("Please input one of the stated shapes.");
        continue;
    }
    }
}
