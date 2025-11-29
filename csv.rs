use std::fs::File; 
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn read_financial_csv(file_path: &str) -> impl Iterator<Item = io::Result<HashMap<String, String>>> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader.lines().map(|line| {
        line.map(|l| {
            let mut map = HashMap::new();
            for pair in l.split(';') {
                if let Some((key, value)) = pair.split_once(':') {
                    map.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            map
        })
    })
}

fn main() {
    for entry in read_financial_csv("financial.csv") {
        match entry {
            Ok(record) => {
                for (key, value) in &record {
                    println!("{} : {}", key, value);
                }
                println!("---");
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}