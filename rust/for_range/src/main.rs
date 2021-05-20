fn main() {
    // Ciclo for basico, con i desde 1 hasta i < 10, es decir 1-9.
    for i in 1..10 {
        println!("{}", i);
    }

    println!("--------------------------------------------------------------");

    // Se incluye el límie superiror -5 <= i <= 5
    for i in -5..=5 {
        println!("{}", i);
    }

    println!("--------------------------------------------------------------");

    // Se modifica el paso del ciclo a 2, genera todos los números pares de 0 a 10.
    for i in (0..=10).step_by(2) {
        println!("{}", i);
    }

    println!("--------------------------------------------------------------");

    // Para hacer un ciclo con paso inverso, se debe invertir el rango con .rev() en lugar de usar un paso negativo.
    for i in (0..=10).rev().step_by(2) {
        println!("{}", i);
    }
}
