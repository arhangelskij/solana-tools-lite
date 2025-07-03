use bs58;
//TODO: 🔴 remove before release
/// Encodes bytes to a Base58 string
pub fn encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

/// Decodes a Base58 string to bytes
pub fn decode(s: &str) -> anyhow::Result<Vec<u8>> {
    bs58::decode(s)
        .into_vec()
        .map_err(|e| anyhow::anyhow!("Base58 decode error: {}", e))
}

/// Educational Base58 encoding: returns both result and a vector of explanation steps.
/// This implementation is intended for explain/debug mode, not for production cryptography.
/// 
/// # Returns
/// - String: final Base58-encoded string
/// - Vec<String>: step-by-step explanation for each stage of encoding
pub fn encode_explain(data: &[u8]) -> (String, Vec<String>) {
    // Step explanations are collected here
    let mut explain = Vec::new();
    let alphabet = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let x = data.to_vec();
    explain.push(format!("Initial bytes: {:?}", x));

    // Count leading zeros (they become '1's in Base58)
    let mut zeros = 0;
    for byte in &x {
        if *byte == 0 {
            zeros += 1;
        } else {
            break;
        }
    }
    explain.push(format!("Leading zeros: {}", zeros));

    // Main loop: convert byte array to Base58 digits (division method)
    let mut num = x;
    let mut b58 = Vec::new();
    while !num.is_empty() && num.iter().any(|&b| b != 0) {
        let mut rem = 0u32;
        let mut new_num = Vec::with_capacity(num.len());

        for byte in num {
            let acc = (rem << 8) | byte as u32;
            let digit = acc / 58;
            rem = acc % 58;
            if !new_num.is_empty() || digit != 0 {
                new_num.push(digit as u8);
            }
        }
        b58.push(alphabet[rem as usize]);
        explain.push(format!(
            "Remainder: {}, Char: '{}'", rem, alphabet[rem as usize] as char
        ));
        num = new_num;
    }
    // Add leading '1's for each leading zero in the input
    for _ in 0..zeros {
        b58.push(alphabet[0]);
        explain.push("Added leading '1' for each 0-byte in input".into());
    }
    // Reverse result: Base58 digits are in little-endian order
    b58.reverse();
    let result = String::from_utf8(b58).unwrap_or_default();
    explain.push(format!("Final Base58 result: {}", result));
    (result, explain)
}