use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let i: u16 = rng.gen(); // Genera un número aleatorio entero sin signo de 16 bits.

    println!("Seed i = {}", i);

    let mut i: u32 = u32::from(i); // Se expande el número inicial a un formato de 32 bits, para evitar desbordamientos al multiplicar por 3.
    let mut c: i32 = 0; // Inicia un contador.

    // El ciclo loop es el equivalente a un ciclo while(true) {}.
    loop {
        if i == 1 {
            break;
        } // El ciclo termina cuando el número se reduce a 1.

        if i % 2 == 0 {
            // Si el número es par se divide por 2.
            i = i / 2;
        } else {
            i = i * 3 + 1; // Si el número es impar se multiplica por 3 y se suma 1.
        }

        c += 1;
        println!("Step {}: {}", c, i);
    }
}
