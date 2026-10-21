use std::io::Write;

fn main() {
    
    let mut file = std::fs::File::create("Nigerian Breweries.csv").expect("Create failed");
    file.write_all("Lager,Stout,Non Alcoholic\n".as_bytes()).expect("write failed");
    file.write_all("33 Export,Legend,Maltina\n".as_bytes()).expect("write failed");
    file.write_all("Desperados,Turbo King,Amsel Malta\n".as_bytes()).expect("write failed");
    file.write_all("Goldberg,Williams,Malta Gold\n".as_bytes()).expect("write failed");
    file.write_all("Gulder,,Fayrouz\n".as_bytes()).expect("write failed");
    file.write_all("Heineken,,\n".as_bytes()).expect("write failed");
    file.write_all("Star,,".as_bytes()).expect("write failed");

   

}