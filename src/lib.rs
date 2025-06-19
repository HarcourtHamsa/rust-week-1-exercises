// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err(String::from("Transaction data too short"));
    }

    let version_bytes = &raw_tx_hex[0..8];
    let version = u32::from_str_radix(version_bytes, 16);

    match version {
        Ok(val) => Ok(val.swap_bytes()),
        Err(_msg) => Err(String::from("Hex decode error")),
    }
}
