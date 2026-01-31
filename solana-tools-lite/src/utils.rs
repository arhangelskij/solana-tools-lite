/// HEX encode
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

/// Format lamports to SOL string with dynamic precision, trimming trailing zeros.
/// Shows up to 9 decimals for very small amounts, otherwise 3 decimals.
/// Removes trailing zeros after decimal point.
pub fn format_sol(lamports: u128) -> String {
    if lamports == 0 {
        return "0 SOL".to_string();
    }
    
    let whole = lamports / 1_000_000_000;
    let frac = lamports % 1_000_000_000;
    
    if frac == 0 {
        format!("{} SOL", whole)
    } else {
        let frac_str = format!("{:09}", frac);
        let trimmed_frac = frac_str.trim_end_matches('0');
        format!("{}.{} SOL", whole, trimmed_frac)
    }
}
