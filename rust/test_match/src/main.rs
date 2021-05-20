use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let i: i8 = rng.gen();
    println!("SEED = {}", i);

    // Match compara directamente un valor con un patron.
    // Los patrones se evaluan en orden de escritura, del primero al último.
    match i {
        -128 => println!("IT'S MIN!."), // Compara i con -128.
        127 => println!("IT'S MAX!."),
        0 => print!("IT'S THE CENTER!."),
        -50..=50 => println!("IS IN 'THE ZONE'!"), // Compara i con un rango entre -50 y 50 inclusive.
        -100 | 100 => println!("IT'S A HUNDRED!."), // Compara i con -100 o 100.
        _ => println!("NOTHING SPECIAL!."), // _ Indica un patron comodin que sirve para todos los casos que no se hallan definido.
    }
}
