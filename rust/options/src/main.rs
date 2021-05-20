use rand::Rng;

fn main() {
    let i = option_generator(rand::thread_rng()); // Genera ona opción de tipo u8

    match i {
        Some(x) => launcher(x), // Si la opción no esta vacia, ejecuta la funcion launcher.
        None => println!("Nothing to do!."), // Si la opción esta vacia, termina el programa.
    }
}

// Recibe un generador de número aleatorios y genera una opción de tipo u8 con un 50% de probabilidad o un None en su lugar.
fn option_generator(mut rng: rand::prelude::ThreadRng) -> Option<u8> {
    let flag: u8 = rng.gen();

    if flag % 2 == 0 {
        let op: Option<u8> = Some(rng.gen()); // Genera una opción en formato u8.
        return op; // Retorna la opción.
    }

    None // Retorna None como valor por defecto. Al no finalizar con ; se asume un valor de retorno.
}

// Recibe un nuemero u8, invoca una evaluación de número primo y una sumatoria de enteros.
fn launcher(i: u8) {
    let flag = is_prime_number(i); // Consulta si el número es primo.

    if flag {
        println!("{} Is a prime number!.", i); // Si es primo, imprime el número.
    } else {
        let sum = get_summation(i); // Si no es primo, invoca una sumatoria.
        println!("The summation from 1 to {} is {}.", i, sum);
    }
}

// Recibe un número u8 y verfica si es primo, devolviendo un booleano.
fn is_prime_number(i: u8) -> bool {
    if i == 2 {
        return true; // Si el número es 2, es el primo par y retorna true.
    }

    // Si el número es mayor a 2 y no es par, lo evalua. En caso contrario devuelve false.
    if i > 2 && i % 2 != 0 {
        let limit = i / 2; // Calcula el limite de búsqueda en la mitad del número a evaluar.

        // Ejecuta un ciclo desde 3 hasta el limite de búsqueda.
        for j in 3..=limit {
            if i % j == 0 {
                return false; // Si el número es divisible por j, no es primo. Devuelve false.
            }
        }

        return true; // Si pasa la prueba de los divisores, devuelve true.
    }

    false // Devuelve false por defecto.
}

fn get_summation(i: u8) -> u16 {
    let limit: u16 = u16::from(i);
    let mut sum: u16 = 0;

    for j in 1..=limit {
        sum = sum + j;
    }

    sum
}
