use std::io;
fn main() {
    println!("Welcome to PAU Cafeteria!");

    //Menu items
    let p1 = "Poundo Yam/Edinkaiko Soup";
    let f1 = "Fried Rice & Chicken";
    let a1 = "Amala & Ewedu Soup";
    let e1 = "Eba & Egusi Soup";
    let w1 = "White Rice & Stew";

    //Menu prices
    let p2 = 3200;
    let f2 = 3000;
    let a2 = 2500;
    let e2 = 2000;
    let w2 = 2500;

    let mut order: Vec<(&str, u32)> = Vec::new(); //storing valuues sorta ig I think

    'main_loop: loop {   //main loop

        println!("\n---Menu--- ");
        println!("{} - ₦{}",p1,p2);
        println!("{} - ₦{}",f1,f2);
        println!("{} - ₦{}",a1,a2);
        println!("{} - ₦{}",e1,e2);
        println!("{} - ₦{}",w1,w2);

        println!("\nKindly make a selection between 1 - 5.");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid input.");
        let c:u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please select a number between 1 - 5");
                continue;
            }
        };

        if c < 1 || c > 5 {
            println!("Please make a valid selection.");
            println!("\n ------------------------------");
            continue;
        }
        match c {
            1 => order.push((p1,p2)),
            2 => order.push((f1,f2)),
            3 => order.push((a1,a2)),
            4 => order.push((e1,e2)),
            5 => order.push((w1,w2)),
            _ => (),
        }
        println!("Added to your order!");

        'inner_loop: loop { //inner loop held together by a prayer
            println!("\nWould you like to add another selection to your order?");
        println!("Kindly answer Y/N");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Invalid answer");
        let a = again.trim();
        if a == "N" || a == "n" {
            break 'inner_loop;
        } 
        //invalid answer
        else if a != "y" && a != "Y" && a != "n" && a != "N" { 
            println!("Please input a valid answer (Y/N)");
            continue;
        }
        
        else if a == "y" || a == "Y" {
            continue 'main_loop;
        }
        }
        println!("\n ---Your Receipt---");
        let mut total:u32 = 0;
        for(item, price) in &order {
            println!("{} - ₦{}",item,price);
            total += price;
        } let mut discount = 0;
        if total > 10_000 {
            discount = (total * 10) / 100 ;
            total = total - discount;
        }
        println!("------------------------------");
        if discount > 1 {
            println!("Total is over ₦10,000 so you've been given a 10% discount!");
        }
        println!("Total: ₦{}",total);
        println!("\nThank you for your patronage!");
        break;
    }

    
}
