use std::env;

pub fn init() -> bool {
    let mut status = false;

    if std::path::Path::new(config_path().as_str()).exists() {
        
    } else {
        // Config file not found
    }

    status
}

pub fn app_path() -> String {
    
    let mut PATH: String = String::from("NULL");

    match env::current_exe() {
        Ok(exe_path) => PATH = exe_path.display().to_string().replace("\\torch.exe", "").replace("\\?\\", ""),
        Err(e) => println!("failed to get current exe path: {}", e),
    };
    PATH.remove(0);
    PATH
}

pub fn config_path() -> String {
    String::from(app_path() + "\\torch.config.json")
}