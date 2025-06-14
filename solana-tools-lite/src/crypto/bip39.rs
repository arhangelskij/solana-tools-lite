use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;

pub fn generate_mnemonic() -> String {
    // TODO: tmp
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
        .into()
}

pub fn derive_seed(mnemonic: &str, passphrase: &str) -> [u8; 64] {
    let salt = format!("mnemonic{}", passphrase);
    let mut out = [0u8; 64];
    
    pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut out)
        .expect("PBKDF2 failed");

    out
}
