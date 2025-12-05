//! QR code generation module

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use qrcode::{QrCode, render::svg};

/// Generate QR code as SVG string
///
/// # Arguments
/// * `data` - Data to encode in QR code
///
/// # Returns
/// SVG string of QR code
pub fn generate_qr_svg(data: &str) -> Result<String> {
    let code = QrCode::new(data.as_bytes())?;
    let svg = code
        .render()
        .min_dimensions(300, 300)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    Ok(svg)
}

/// Generate QR code as base64-encoded PNG data URL
///
/// # Arguments
/// * `data` - Data to encode in QR code
///
/// # Returns
/// Data URL (data:image/png;base64,...)
pub fn generate_qr_data_url(data: &str) -> Result<String> {
    let code = QrCode::new(data.as_bytes())?;
    
    // Render as PNG bytes
    let png_data = code
        .render::<image::Luma<u8>>()
        .min_dimensions(300, 300)
        .build();
    
    // Convert to PNG buffer
    let mut png_buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_buffer);
    
    image::DynamicImage::ImageLuma8(png_data)
        .write_to(&mut cursor, image::ImageFormat::Png)?;
    
    // Encode as base64
    let base64_data = general_purpose::STANDARD.encode(&png_buffer);
    Ok(format!("data:image/png;base64,{}", base64_data))
}

/// Generate pairing QR code data URI
///
/// Format: bridgex://pair?id={device_id}&key={public_key_base64}
///
/// # Arguments
/// * `device_id` - Unique device identifier
/// * `public_key` - Public key bytes
///
/// # Returns
/// QR code data URI and the encoded string
pub fn generate_pairing_qr(
    device_id: &str,
    public_key: &[u8],
) -> Result<(String, String)> {
    let key_b64 = general_purpose::STANDARD.encode(public_key);
    let pairing_uri = format!("bridgex://pair?id={}&key={}", device_id, key_b64);
    let qr_data_url = generate_qr_data_url(&pairing_uri)?;
    Ok((qr_data_url, pairing_uri))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_qr_svg() {
        let svg = generate_qr_svg("test data").unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_generate_qr_data_url() {
        let data_url = generate_qr_data_url("test data").unwrap();
        assert!(data_url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_generate_pairing_qr() {
        let device_id = "test-device-123";
        let public_key = vec![1, 2, 3, 4, 5];
        let (qr_url, pairing_uri) = generate_pairing_qr(device_id, &public_key).unwrap();
        
        assert!(qr_url.starts_with("data:image/png;base64,"));
        assert!(pairing_uri.contains("bridgex://pair"));
        assert!(pairing_uri.contains(device_id));
    }
}
