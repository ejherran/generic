use crypto_hash::{Algorithm, hex_digest};

fn main() {
    let digest = hex_digest(Algorithm::SHA512, b"Hola SHA512");
    println!("Hola SHA512 < {} >", digest);
}
