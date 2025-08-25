use crate::types::SecurityType;

pub struct WifiQr {
    pub ssid: String,
    pub password: Option<String>,
    pub security: SecurityType,
    pub hidden: bool,
    pub size: u32,
}

#[derive(Debug, PartialEq)]
pub enum WifiQrError {
    SsidTooLong(usize),
    SsidEmpty,
    PasswordRequired,
    PasswordTooLong(usize),
    InvalidQrSize(u32),
}

impl std::fmt::Display for WifiQrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WifiQrError::SsidTooLong(len) => write!(f, "SSID too long: {} characters (max 32)", len),
            WifiQrError::SsidEmpty => write!(f, "SSID cannot be empty"),
            WifiQrError::PasswordRequired => write!(f, "Password is required for this security type"),
            WifiQrError::PasswordTooLong(len) => write!(f, "Password too long: {} characters (max 63)", len),
            WifiQrError::InvalidQrSize(size) => write!(f, "Invalid QR code size: {} (must be between 1 and 1000)", size),
        }
    }
}

impl std::error::Error for WifiQrError {}

impl WifiQr {
    pub fn new(
        ssid: String,
        password: Option<String>,
        security: SecurityType,
        size: u32,
        hidden: bool,
    ) -> Result<Self, WifiQrError> {
        if ssid.is_empty() {
            return Err(WifiQrError::SsidEmpty);
        }
        
        if ssid.len() > 32 {
            return Err(WifiQrError::SsidTooLong(ssid.len()));
        }

        match security {
            SecurityType::Wep | SecurityType::Wpa2 | SecurityType::Wpa3 => {
                if password.is_none() {
                    return Err(WifiQrError::PasswordRequired);
                }

                #[allow(clippy::collapsible_if)]
                if let Some(ref pwd) = password {
                    if pwd.len() > 63 {
                        return Err(WifiQrError::PasswordTooLong(pwd.len()));
                    }
                }
            }
            SecurityType::None => {}
        }

        if size == 0 || size > 1000 {
            return Err(WifiQrError::InvalidQrSize(size));
        }

        Ok(Self { ssid, password, security, hidden, size })
    }

    fn escape_special_chars(input: &str) -> String {
        input
            .replace('\\', "\\\\")
            .replace(';', "\\;")
            .replace(',', "\\,")
            .replace('"', "\\\"")
    }
    

    pub fn to_qr_string(&self) -> String {
        let base_length = "WIFI:T:;S:;P:;H:;;".len(); // 18 chars for structure
        let security_length = match self.security {
            SecurityType::None => "nopass".len(),
            SecurityType::Wep => "WEP".len(),
            SecurityType::Wpa2 | SecurityType::Wpa3 => "WPA".len(),
        };
        let hidden_length = if self.hidden { 4 } else { 5 }; // "true" or "false"
        
        // Estimate escaped string lengths (add 20% buffer for escaping)
        let ssid_estimated = (self.ssid.len() as f32 * 1.2) as usize;
        let password_estimated = self.password.as_ref()
            .map(|p| (p.len() as f32 * 1.2) as usize)
            .unwrap_or(0);
        
        let estimated_capacity = base_length + security_length + hidden_length + 
                                ssid_estimated + password_estimated;

        let mut res = String::with_capacity(estimated_capacity);
        res += "WIFI:T:";

        res += match self.security {
            SecurityType::None => "nopass",
            SecurityType::Wep => "WEP",
            SecurityType::Wpa2 => "WPA",
            SecurityType::Wpa3 => "WPA",
        };

        res += ";S:";
        res += &Self::escape_special_chars(&self.ssid);
        res += ";P:";
        
        if let Some(password) = &self.password {
            res += &Self::escape_special_chars(password);
        }

        res += ";H:";
        res += if self.hidden { "true" } else { "false" };
        res += ";;";

        res
    }
}
