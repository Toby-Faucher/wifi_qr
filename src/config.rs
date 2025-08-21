use schematic::Config;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Config, Serialize, Deserialize)]
pub struct WifiQrConfig {
    // [ Core Settings ]

    #[setting(default = 256)]
    default_size: usize,

    //TODO: put this in a enum
    #[setting(default = "wpa2")]
    default_security: String,

    #[setting(default = "png")]
    default_output_format: String,

    #[setting(default = "/")]
    default_output_directory: String,

    // [ Display Settings ]

    #[setting(default = false)]
    auto_terminal: bool,

    //TODO: make a enum for these
    #[setting(default = "low")]
    qr_error_correction_level: String,

    #[setting(default = "None")]
    background_color: String,

    #[setting(default = "256")]
    svg_viewbox_size: String,

    //TODO: Make sure that this works automaticaly with the indexer for file outputs.
    #[setting(default = "qr_code")]
    file_naming_pattern: String,

    //TODO: Make Promting system
    #[setting(default = true)]
    confirm_overwrite: bool,

    //TODO This should only work when wanting to output to a file. 
    #[setting(default = true)]
    auto_open_file: bool,

    #[setting(default = false)]
    verbose_output: bool,

    #[setting(default = "None")]
    qr_logo_path: String,
}

impl WifiQrConfig {
    pub fn get_config_path() -> PathBuf {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home).join(".config").join("wifi_qr").join("config.toml")
    }
    
    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: WifiQrConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = WifiQrConfig::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let toml_content = toml::to_string_pretty(self)?;
        fs::write(config_path, toml_content)?;
        Ok(())
    }
}
