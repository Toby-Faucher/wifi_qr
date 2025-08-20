use clap::{ValueEnum};
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
