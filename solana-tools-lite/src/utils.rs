/// HEX encode
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

/// Format lamports to SOL string with dynamic precision.
/// Shows up to 9 decimals for very small amounts, otherwise 3 decimals.
pub fn format_sol(lamports: u128) -> String {
    let sol = lamports as f64 / crate::constants::LAMPORTS_PER_SOL;
    if lamports > 0 && sol < 0.001 {
        format!("{:.9} SOL", sol)
    } else {
        format!("{:.3} SOL", sol)
    }
}
