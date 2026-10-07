fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is {}",num);
}

fn mutate_num_to_zero(para_num:&mut i32) {
    *para_num = *para_num*0; //de reference
    println!("para_num value is :{}",para_num);
}
