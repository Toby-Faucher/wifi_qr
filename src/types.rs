use clap::{ValueEnum};

#[derive(Clone, Debug, ValueEnum)]    
pub enum SecurityType {
    #[value(name = "none")]
    None,
    #[value(name = "wep")]
    Wep,
    #[value(name = "wpa2")]
    Wpa2,
    #[value(name = "wpa3")]
    Wpa3,
}
