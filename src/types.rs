use clap::{ValueEnum};
use qrcode::EcLevel;
use std::path::PathBuf;
use std::str::FromStr;

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

#[derive(Clone, Debug, Default, ValueEnum)]    
pub enum ErrorCorrectionLevel {
    #[value(name = "low", alias = "l")]
    Low,
    #[value(name = "medium", alias = "m")]
    #[default]
    Medium,
    #[value(name = "quartile", alias = "q")]
    Quartile,
    #[value(name = "high", alias = "h")]
    High,
}

#[derive(Clone,Debug)]
pub struct ValidatedFilePath(PathBuf);

impl FromStr for ValidatedFilePath {
   type Err = String; 

   fn from_str(s: &str) -> Result<Self, Self::Err> {
       let path = PathBuf::from(s);

       if path.as_os_str().is_empty() {
           return Err("Path Cannot be empty".to_string());
       }

       // The code is much more idomatic this way
       #[allow(clippy::collapsible_if)]
       if let Some(parent) = path.parent() {
           if !parent.exists() {
                return Err(format!("Parent does not exist: {}", parent.display()));
           }
       }

       Ok(ValidatedFilePath(path))
   }
}

impl AsRef<std::path::Path> for ValidatedFilePath {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}

impl ErrorCorrectionLevel {
    pub fn to_ec_level(self) -> EcLevel {
        match self {
            ErrorCorrectionLevel::Low => EcLevel::L,
            ErrorCorrectionLevel::Medium => EcLevel::M,
            ErrorCorrectionLevel::Quartile => EcLevel::Q,
            ErrorCorrectionLevel::High => EcLevel::H,
        }
    }
}

impl FromStr for ErrorCorrectionLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "l" | "low" => Ok(ErrorCorrectionLevel::Low),
            "m" | "medium" => Ok(ErrorCorrectionLevel::Medium),
            "q" | "quartile" => Ok(ErrorCorrectionLevel::Quartile),
            "h" | "high" => Ok(ErrorCorrectionLevel::High),
            _ => Err(format!("Invalid Error Correction Level: {}", s))
        }
    }
}


impl std::fmt::Display for ErrorCorrectionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            ErrorCorrectionLevel::Low => "L",
            ErrorCorrectionLevel::Medium => "M",
            ErrorCorrectionLevel::Quartile => "Q",
            ErrorCorrectionLevel::High => "H",
        };
        write!(f, "{}", letter)
    }
}
