use std::env;
use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::time::Instant;


fn main() -> io::Result<()> {
    let before = Instant::now();
    let path = env::args().nth(1).expect("No path found");
    let contents = read_file(path).expect("Invalid path");
    let netcl = find_netcl(&contents);
    let build_id = find_build_id(&contents);
    println!("NetCL found: {:?}", netcl);
    println!("BuildID found: {:?}", build_id);
    println!("Completed in {:?}", before.elapsed());
    Ok(())
}

fn read_file(path: String) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = vec!();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn find_build_id(content: &Vec<u8>) -> String {
    let re = regex::Regex::
        new("2B2B466F72746E6974652B..................................................").unwrap();
    let hex = hex::encode_upper(content);
    let netcl_text: String = re.captures(&hex).unwrap().get(0).map_or("", |m| m.as_str()).into();
    let netcl_text: String = netcl_text.chars().take(72).collect();
    let netcl_bytes = hex::decode(netcl_text).unwrap();
    String::from_utf8(netcl_bytes).expect("Failed to convert build to utf8")
}

fn find_netcl(content: &Vec<u8>) -> u64 {
    let re = regex::Regex::new("9A9999999999C93F........000000").unwrap();
    let hex = hex::encode_upper(content);
    let netcl_text: String = re.captures(&hex).unwrap().get(0).map_or("", |m| m.as_str()).into();
    let netcl_text: String = netcl_text.chars().skip(16).take(20).collect();
    let mut netcl_bytes = hex::decode(netcl_text).unwrap();
    netcl_bytes.push(0);
    netcl_bytes.iter().rev().fold(0u64, |r, x| r << 8 | *x as u64)
}