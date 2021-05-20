use crypto_hash::{hex_digest, Algorithm};

fn main() {
    let digest = hex_digest(Algorithm::SHA512, b"Hola SHA512");
    println!("Hola SHA512 < {} >", digest);
}
