use mlua::{Lua, Table};

use crate::{
    info::{get_cpu_name, get_gpu_name, get_mem_info, get_uptime, get_user_host},
    modules::Module,
};

#[derive(Debug, PartialEq)]
pub enum ModuleMode {
    Fancy,
    Basic,
}

#[derive(Debug)]
// a single Rustfetch instance
// this is started everytime the "rustfetch" command is called
pub struct RsfInstance {
    pub ascii_path: String,
    pub modules: Vec<ModuleConfig>,
    pub mode: ModuleMode,
}

impl RsfInstance {
    pub fn new(modules: Vec<ModuleConfig>, ascii_path: String, mode: ModuleMode) -> Self {
        Self {
            ascii_path,
            modules,
            mode,
        }
    }

    pub fn output(&self) {
        // Print ASCII art
        let ascii_path = std::env::home_dir()
            .unwrap()
            .join(".config/rustfetch/ascii")
            .join(&self.ascii_path);

        let ascii_contents = std::fs::read_to_string(&ascii_path).unwrap_or_else(|e| {
            eprintln!(
                "Failed to read ASCII file ({}) due to error: {e}",
                ascii_path.display()
            );
            std::process::exit(1);
        });

        println!("{ascii_contents}");

        // Process modules based on mode
        if self.mode == ModuleMode::Fancy {
            for module in &self.modules {
                let value = match module.module_type.as_str() {
                    "builtin" => self.get_builtin_module_value(&module.identifier),
                    //"custom" => self.get_custom_module_value(module),
                    _ => "Unknown module type".to_string(),
                };

                let display_name = module.identifier.clone();

                // Actually print the rendered module
                let rendered = Module::new(&display_name, &value).render_as_fancy();
                for line in rendered {
                    println!("{line}");
                }
            }
        } else {
            println!("Custom modules are not implemented yet.\nAborting...");
            std::process::exit(1);
        }
    }

    fn get_builtin_module_value(&self, identifier: &str) -> String {
        match identifier {
            "CPU" => get_cpu_name(),
            "GPU" => get_gpu_name(),
            "Host" => get_user_host(),
            "Uptime" => get_uptime(),
            "Memory" => get_mem_info(),
            _ => {
                eprintln!("Unknown builtin module: {}", identifier);
                "Not available".to_string()
            }
        }
    }
}

#[derive(Debug)]
pub struct ModuleConfig {
    pub identifier: String,
    pub module_type: String,
    pub function_to_call: Option<String>,
    //pub name: Option<String>,
    //pub enabled: bool,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            identifier: "cpu".to_string(),
            module_type: "builtin".to_string(),
            function_to_call: None,
        }
    }
}

pub fn parse_modules_table(modules_table: &Table) -> mlua::Result<Vec<ModuleConfig>> {
    let mut modules = Vec::new();

    for pair in modules_table.pairs::<String, Table>() {
        let (module_name, module_table) = pair?;

        let mut config = ModuleConfig::default();
        config.module_type = module_table.get("type").unwrap_or(config.module_type);
        config.function_to_call = module_table.get("function_to_call").ok();
        config.identifier = module_name;

        modules.push(config);
    }

    Ok(modules)
}

// this is the main entry point of the rustfetch application
// this is what needs to be called when the rustfetch binary is called in the terminal without arguments
pub fn load_lua_config() -> Result<(), Box<dyn std::error::Error>> {
    let lua = Lua::new();

    // Load config file
    let config_path = std::env::home_dir()
        .expect("Home directory not found!")
        .join(".config/rustfetch/config.lua");

    let lua_config = std::fs::read_to_string(&config_path)?;

    // Execute Lua code
    lua.load(&lua_config).exec()?;

    // Extract values
    let globals = lua.globals();

    //let theme: String = globals.get("theme").unwrap_or_else(|_| "default".to_string());
    let module_mode: String = globals.get("mode").unwrap_or_else(|_| "fancy".to_string());
    let ascii_path: String = globals
        .get("ascii")
        .unwrap_or_else(|_| "tux.txt".to_string());
    let modules_table: Table = globals.get("modules").unwrap();
    //let show_ascii: bool = globals.get("show_ascii").unwrap_or(true);

    let actual_mode = match module_mode.as_str() {
        "basic" => ModuleMode::Basic,
        _ => ModuleMode::Fancy,
    };

    let modules = parse_modules_table(&modules_table)?;

    let instance = RsfInstance::new(modules, ascii_path, actual_mode);
    instance.output();
    Ok(())
}
