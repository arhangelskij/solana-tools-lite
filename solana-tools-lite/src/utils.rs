/// HEX encode
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

/// Format lamports to SOL string with dynamic precision, trimming trailing zeros.
/// Shows up to 9 decimals for very small amounts, otherwise 3 decimals.
/// Removes trailing zeros after decimal point.
pub fn format_sol(lamports: u128) -> String {
    let sol = lamports as f64 / crate::constants::LAMPORTS_PER_SOL;
    
    let formatted = if lamports > 0 && sol < 0.001 {
        format!("{:.9}", sol)
    } else {
        format!("{:.3}", sol)
    };
    
    // Trim trailing zeros and decimal point if needed
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    format!("{} SOL", trimmed)
}
