struct Laptop {
    price:u32, 
    quantity:u32,
    brand:String,
}

fn main() {

    let hp = Laptop {
        price:650_000,
        quantity:10,
        brand:String::from("HP"),
    };
    let ibm = Laptop {
        price:755_000,
        quantity:6,
        brand:String::from("IBM"),
    };
    let toshiba = Laptop {
        price:550_000,
        quantity:10,
        brand:String::from("Toshiba"),
    };
    let dell = Laptop {
        price:850_000,
        quantity:4,
        brand:String::from("Dell"),
    };

    println!("\nLaptop Stock:\n");

    println!("Brand: {}, Quantity: {}, Price: ₦{}\n",hp.brand,hp.quantity,hp.price);
    println!("Brand: {}, Quantity: {}, Price: ₦{}\n",ibm.brand,ibm.quantity,ibm.price);
    println!("Brand: {}, Quantity: {}, Price: ₦{}\n",toshiba.brand,toshiba.quantity,toshiba.price);
    println!("Brand: {}, Quantity: {}, Price: ₦{}\n",dell.brand,dell.quantity,dell.price);

    println!("-----------------------\n");

    println!("Customer purchases 3 of each:\n");

    let hp_cost = calculate_cost(hp.price);
    let ibm_cost = calculate_cost(ibm.price);
    let toshiba_cost = calculate_cost(toshiba.price);
    let dell_cost = calculate_cost(dell.price);

    println!("HP Cost = ₦{}\n",hp_cost);
    println!("IBM Cost = ₦{}\n",ibm_cost);
    println!("Toshiba Cost = ₦{}\n",toshiba_cost);
    println!("Dell Cost = ₦{}\n",dell_cost);

    let total_cost = total(hp_cost,ibm_cost,toshiba_cost,dell_cost);

    println!("-----------------------\n");


    println!("Total Cost = ₦{}",total_cost);
}

fn calculate_cost(a:u32) -> u32 {
    a * 3
}

fn total(a:u32,b:u32,c:u32,d:u32) -> u32 {
    a + b + c + d
}