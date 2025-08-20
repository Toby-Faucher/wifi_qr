use crate::types::SecurityType;

pub struct WifiQr {
    pub ssid: String,
    pub password: Option<String>,
    pub security: SecurityType,
    pub hidden: bool,
    pub size: u32,
}

impl WifiQr {
    pub fn new(
        ssid: String,
        password: Option<String>,
        security: SecurityType,
        size: u32,
        hidden: bool
        ) -> Self{
       Self {ssid, password, security, hidden, size} 
    }
    
    pub fn to_qr_string(&self) -> String {
        todo!()
    }
}

