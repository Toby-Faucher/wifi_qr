use crate::qr::wifi::WifiQr;
use crate::types::ValidatedFilePath;
use qrcode::QrCode;
use image::Luma;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub enum GenerateError {
    QrCodeError(qrcode::types::QrError),
    ImageError(image::ImageError),
    IoError(io::Error),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::QrCodeError(e) => write!(f, "QR code generation error: {}", e),
            GenerateError::ImageError(e) => write!(f, "Image processing error: {}", e),
            GenerateError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for GenerateError {}

impl From<qrcode::types::QrError> for GenerateError {
    fn from(error: qrcode::types::QrError) -> Self {
        GenerateError::QrCodeError(error)
    }
}

impl From<image::ImageError> for GenerateError {
    fn from(error: image::ImageError) -> Self {
        GenerateError::ImageError(error)
    }
}

impl From<io::Error> for GenerateError {
    fn from(error: io::Error) -> Self {
        GenerateError::IoError(error)
    }
}

pub fn gen_qr(wifi_struct: WifiQr, output: Option<ValidatedFilePath>, terminal: Option<bool>) -> Result<(), GenerateError> {
    let wifi_string = wifi_struct.to_qr_string();
    let code = QrCode::new(wifi_string)?;
    
    let should_display_terminal = terminal.unwrap_or(false);
    
    if should_display_terminal {
        display_qr_terminal(&code);
    }
    
    if let Some(output_path) = output {
        save_qr_to_file(&code, &wifi_struct, &output_path)?;
    } else {
        let default_path = generate_default_filename()?;
        save_qr_to_file(&code, &wifi_struct, &default_path)?;
    }
    
    Ok(())
}

fn display_qr_terminal(code: &QrCode) {
    let string = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    
    println!("{}", string);
}

fn generate_default_filename() -> Result<ValidatedFilePath, GenerateError> {
    let mut counter = 0;
    let base_name = "qr_code";
    let extension = "png";
    let current_dir = std::env::current_dir()?;
    
    loop {
        let filename = if counter == 0 {
            format!("{}.{}", base_name, extension)
        } else {
            format!("{}{}.{}", base_name, counter, extension)
        };
        
        let path = current_dir.join(&filename);
        
        if !path.exists() {
            return ValidatedFilePath::from_str(&path.to_string_lossy()).map_err(|e| {
                GenerateError::IoError(io::Error::new(io::ErrorKind::InvalidInput, e))
            });
        }
        
        counter += 1;
        
        if counter > 1000 {
            return Err(GenerateError::IoError(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Too many existing qr_code files, cannot generate unique filename"
            )));
        }
    }
}

fn save_qr_to_file(code: &QrCode, wifi_struct: &WifiQr, output_path: &ValidatedFilePath) -> Result<(), GenerateError> {
    let image = code.render::<Luma<u8>>()
        .max_dimensions(wifi_struct.size, wifi_struct.size)
        .build();
    
    image.save(output_path)?;
    println!("QR code saved to: {}", output_path.as_ref().display());
    
    Ok(())
}
