use crate::qr::wifi::WifiQr;
use crate::types::{ErrorCorrectionLevel, ValidatedFilePath};
use qrcode::QrCode;
use image::Luma;
use std::io;
use std::str::FromStr;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Png,
    Svg,
}

impl OutputFormat {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        match path.as_ref().extension().and_then(|ext| ext.to_str()) {
            Some(ext) => match ext.to_lowercase().as_str() {
                "svg" => OutputFormat::Svg,
                _ => OutputFormat::Png,
            },
            None => OutputFormat::Png,
        }
    }
    
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Svg => "svg",
        }
    }
}

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

pub fn gen_qr(wifi_struct: WifiQr, output: Option<ValidatedFilePath>, terminal: Option<bool>, ec_level: Option<ErrorCorrectionLevel>) -> Result<(), GenerateError> {
    let wifi_string = wifi_struct.to_qr_string();
    let error_correction = ec_level.unwrap_or_default();
    let code = QrCode::with_error_correction_level(wifi_string, error_correction.to_ec_level())?;
    
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
    generate_default_filename_with_format(OutputFormat::Png)
}

fn generate_default_filename_with_format(format: OutputFormat) -> Result<ValidatedFilePath, GenerateError> {
    let mut counter = 0;
    let base_name = "qr_code";
    let extension = format.extension();
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
    let format = OutputFormat::from_path(output_path.as_ref());
    
    match format {
        OutputFormat::Png => {
            let image = code.render::<Luma<u8>>()
                .max_dimensions(wifi_struct.size, wifi_struct.size)
                .build();
            
            image.save(output_path)?;
        },
        OutputFormat::Svg => {
            let svg_string = code.render::<qrcode::render::svg::Color>()
                .max_dimensions(wifi_struct.size, wifi_struct.size)
                .build();
            
            fs::write(output_path.as_ref(), svg_string)?;
        }
    }
    
    println!("QR code saved to: {}", output_path.as_ref().display());
    
    Ok(())
}
