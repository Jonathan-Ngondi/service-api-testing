use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub const RANDOM_STRING:&str = "RDR3cThqdmFFV0ZCNVVWL0V3cFlDL3V4R0hFYmFzb3BlSytzMkZIRllzNHZiNWJ2dUZtaXI3eDU=";
pub const RANDOM_STRING2:&str = "TFg4ODQ2SG8wV2EwT1ovbE1IN3V4em9hZGhyck9ySnFOQ0xQdjNLbnF1eXdHWGZoeWFxbXpoUXF0MTBCUEx4ZVNNWTE2VHkyaTNKeG8zWnJaaElWcFE9PQ==";
pub const RANDOM_STRING3: &str = "STNWRk0zSktNTkRKQ0RINUJNQkVFUUFXNktKNk5PRTM=";

// pub fn read_keys(file_name:&str) -> Vec<String> {
    
//     let file = File::open(file_name).expect("file not found!");
//     let reader = BufReader::new(file);
//     let mut vec: Vec<String> = Vec::new();

//     for line in reader.lines() {
//         vec.push(line.unwrap());
//     }

//     vec
// }