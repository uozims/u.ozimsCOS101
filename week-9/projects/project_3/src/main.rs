use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye",];
    let ministries = ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum",];
    let zones = ["South West","North East","South South","South West","South East",];

    let mut comm_file = File::create("Commissioners.csv").expect("Create failed");
    comm_file.write_all("Commissioner\n".as_bytes()).expect("Write failed");
    for name in commissioners {
        comm_file.write_all(name.as_bytes()).expect("Write failed");
        comm_file.write_all("\n".as_bytes()).expect("Write failed");
    }

    let mut min_file = File::create("Ministries.csv").expect("Create failed");
    min_file.write_all("Ministry\n".as_bytes()).expect("Write failed");
    for ministry in ministries {
        min_file.write_all(ministry.as_bytes()).expect("Write failed");
        min_file.write_all("\n".as_bytes()).expect("Write failed");
    }

    let mut zone_file = File::create("Zones.csv").expect("Create failed");
    zone_file.write_all("Zone\n".as_bytes()).expect("Write failed");
    for zone in zones {
        zone_file.write_all(zone.as_bytes()).expect("Write failed");
        zone_file.write_all("\n".as_bytes()).expect("Write failed");
    }

    let mut merged_file = File::create("EFCC Merged Records.csv").expect("Create failed");
    merged_file.write_all("Commissioner,Ministry,Zone\n".as_bytes()).expect("Write failed");

    for i in 0..5 {
        merged_file.write_all(commissioners[i].as_bytes()).expect("Write failed");
        merged_file.write_all(",".as_bytes()).expect("Write failed");
        merged_file.write_all(ministries[i].as_bytes()).expect("Write failed");
        merged_file.write_all(",".as_bytes()).expect("Write failed");
        merged_file.write_all(zones[i].as_bytes()).expect("Write failed");
        merged_file.write_all("\n".as_bytes()).expect("Write failed");
    }

    println!("\nData written to file.");

}