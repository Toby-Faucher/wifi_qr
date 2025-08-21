use schematic::Config;

#[derive(Config)]
struct WifiQrConfig {
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
