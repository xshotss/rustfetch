use std::env;
// a module for retrieving system info
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

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
            } else {
                return line.trim().to_string();
            }
        }
    }

    "Not found!".to_string()
}

pub fn get_gpu_name() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i \"VGA\"")
        .output()
        .unwrap_or_else(|e| {
            eprintln!("Could not run lspci command to get GPU info!\nIs pciutils installed?");
            eprintln!("Error generated: {e}");
            std::process::exit(1);
        });

    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        if line.contains("VGA") {
            // Split at the first colon to remove PCI address
            if let Some((_, after_first_colon)) = line.split_once(':') {
                // Split at the second colon to get just the description
                if let Some((_, description)) = after_first_colon.split_once(':') {
                    return description.trim().to_string();
                }
            }
        }
    }

    "GPU not found".to_string()
}

pub fn get_user_host() -> String {
    let username = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let hostname = get_hostname().unwrap_or_else(|_| "localhost".to_string());

    format!("{}@{}", username, hostname)
}

pub fn get_hostname() -> Result<String, Box<dyn std::error::Error>> {
    let hostname = std::fs::read_to_string("/etc/hostname")?.trim().to_string();

    if hostname.is_empty() {
        Err("Empty hostname".into())
    } else {
        Ok(hostname)
    }
}

pub fn get_uptime() -> String {
    let uptime_content = std::fs::read_to_string("/proc/uptime").unwrap_or_else(|e| e.to_string());
    let uptime_seconds: f64 = uptime_content
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse()
        .expect("Failed to get uptime!");

    let days = (uptime_seconds / 86400.0) as u64;
    let hours = ((uptime_seconds % 86400.0) / 3600.0) as u64;
    let minutes = ((uptime_seconds % 3600.0) / 60.0) as u64;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn get_mem_info() -> String {
    let uptime_content = std::fs::read_to_string("/proc/meminfo").unwrap_or_else(|e| e.to_string());

    let mut total = String::new();
    let mut used = String::new();

    for line in uptime_content.trim().lines() {
        if line.starts_with("MemTotal:") {
            total = line
                .split(':')
                .nth(1)
                .unwrap()
                .replace("kB", "")
                .trim()
                .to_string();
        } else if line.starts_with("MemAvailable:") {
            used = line
                .split(':')
                .nth(1)
                .unwrap()
                .replace("kB", "")
                .trim()
                .to_string();
        }
    }

    // convert total and used to gigabytes instead of kilobytes
    let total_as_gb = total.parse::<f64>().unwrap_or(0.0) / 1024.0 / 1024.0; // KB → MB → GB

    let used_as_gb = used.parse::<f64>().unwrap_or(0.0) / 1024.0 / 1024.0; // KB → MB → GB

    format!(
        "{:.3}GB / {:.3}GB ({:.1}%)",
        total_as_gb - used_as_gb,
        total_as_gb,
        // convert it to percentage
        ((total_as_gb - used_as_gb) / total_as_gb) * 100.0
    )
}

#[cfg(test)]
mod info_tests {
    use crate::info::{get_cpu_name, get_gpu_name, get_mem_info, get_uptime, get_user_host};

    #[test]
    fn get_cpu_data_success() {
        std::fs::write("tests/cpu_name.txt", get_cpu_name()).unwrap();
    }

    #[test]
    fn get_gpu_data_success() {
        std::fs::write("tests/gpu_name.txt", get_gpu_name()).unwrap();
    }

    #[test]
    fn get_hostname_success() {
        std::fs::write("tests/hostname.txt", format!("{}", get_user_host())).unwrap();
    }

    #[test]
    fn get_uptime_success() {
        std::fs::write("tests/uptime.txt", format!("{}", get_uptime())).unwrap();
    }

    #[test]
    fn get_mem_info_success() {
        std::fs::write("tests/mem.txt", get_mem_info()).unwrap();
    }
}
