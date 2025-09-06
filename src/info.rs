// a module for retrieving system info
use std::fs::File;
use std::io::{BufRead, BufReader};



pub fn get_cpu_name() -> String {
    let file = match File::open("/proc/cpuinfo") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open /proc/cpuinfo: {}", e);
            std::process::exit(1);
        }
    };
    
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };
        
        if line.starts_with("model name") {
            // Split on colon and take the part after it
            if let Some(cpu_name) = line.split(':').nth(1) {
                return cpu_name.trim().to_string();
            }
            else {
                return line.trim().to_string();
            }
        }
    }
    
    "Not found!".to_string()
}



#[cfg(test)]
mod info_tests {
    use crate::info::get_cpu_name;

    #[test]
    fn get_cpu_data_success() {
        
        std::fs::write("cpu_name.txt", get_cpu_name()).unwrap();
    }
}