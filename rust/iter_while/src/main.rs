use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut i: i8 = 0;

    while i < 100 {
        i = rng.gen(); // Genera un nuero aleatorio en el rango de 8 bits.

        if i < 0 {
            continue;
        } // Si el número generado es negativo, se omite y pasa al siguiente ciclo.

        if (i % 9 == 0) && (i != 0) {
            break;
        } // Si el número generado es multiplo de 9 y diferente de 0, termina el ciclo forzadamente.

        println!("i = {}", i);
    }
}
