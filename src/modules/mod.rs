pub struct Module {
    pub name: String,
    pub value: String,
    pub width: usize,      // Total module width including borders
    pub name_width: usize, // Width reserved for the name
}

impl Module {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            width: 45,     // Default total width
            name_width: 8, // Default name width
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn render(&self) -> Vec<String> {
        let content_width = self.width - 4; // Account for "│ " and " │"
        let value_width = content_width - self.name_width - 3; // Account for " | "

        // Format name (centered in its allocated space)
        let name_display = format!("{:^width$}", self.name, width = self.name_width);

        // Format value (truncate if too long)
        let value_display = if self.value.chars().count() > value_width {
            let mut truncated: String = self.value.chars().take(value_width - 3).collect();
            truncated.push_str("...");
            format!("{:<width$}", truncated, width = value_width)
        } else {
            format!("{:<width$}", self.value, width = value_width)
        };

        // Build the lines
        let border = format!("┌{}┐", "─".repeat(self.width - 2));
        let content = format!("│ {} | {} │", name_display, value_display);
        let bottom_border = format!("└{}┘", "─".repeat(self.width - 2));

        vec![border, content, bottom_border]
    }
}

#[cfg(test)]
mod module_tests {
    use std::fs;

    use crate::modules::Module;

    #[test]
    fn test_modules_success() {
        use std::fs::OpenOptions;
        use std::io::Write;

        let cpu_module = Module::new("CPU", "Intel i5-10210U (8) @ 4.200GHz").with_width(45);

        let memory_module = Module::new("MEM", "8.2GB / 15.9GB (51%)").with_width(45);

        let test_destination = "tests/modules_test_inacc.txt";

        // clear previous content and write header
        fs::write(
            test_destination,
            "This information is NOT accurate. It uses hardcoded values.\n\n",
        )
        .unwrap();

        // Open file in append mode
        let mut file = OpenOptions::new()
            .append(true)
            .open(test_destination)
            .unwrap();

        // Write CPU module
        for line in cpu_module.render() {
            writeln!(file, "{}", line).unwrap();
            println!("{line}");
        }

        writeln!(file).unwrap(); // Add spacing between modules

        // Write Memory module
        for line in memory_module.render() {
            writeln!(file, "{}", line).unwrap();
            println!("{line}");
        }

        // Verify the file was written correctly
        let contents = fs::read_to_string(test_destination).unwrap();
        assert!(contents.contains("CPU"));
        assert!(contents.contains("Intel i5-10210U"));
        assert!(contents.contains("MEM"));
        assert!(contents.contains("8.2GB"));
    }
}
