use crate::qr::wifi::WifiQr;
use crate::types::ValidatedFilePath;
use qrcode::QrCode;
use image::Luma;


pub fn gen_qr(wifi_struct: WifiQr, output:Option<ValidatedFilePath>, terminal:Option<bool>) {
    let wifi_string = wifi_struct.to_qr_string();
    println!("{}", wifi_string);
    let _code = QrCode::new(b"todo!");
}
