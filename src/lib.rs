pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let bytes = match hex::decode(&raw_tx_hex[..8]) {
        Ok(b) => b,
        Err(_) => return Err("Hex decode error".to_string()),
    };

    if bytes.len() != 4 {
        return Err("Invalid version byte length".to_string());
    }

    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(version)
}

