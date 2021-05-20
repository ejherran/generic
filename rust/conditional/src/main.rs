use rand::Rng; // Carga la libreria rand.

fn main() {
    let mut rng = rand::thread_rng(); // Crea un generador basico de números aleatorios.
    let x: i8 = rng.gen(); // Genera un número aleatorio que corresponda con el formato de número de la variable destino.

    if x > 0 {
        println!("Positivo. {}", x);
    } else if x < 0 {
        println!("Negativo. {}", x);
    } else {
        println!("Es 0.");
    }

    let y: i8 = rng.gen();
    let z = if y < 0 { y.abs() } else { y }; // Asigna a una variable el resultado la condición.

    println!("ABS({}) = {}", y, z);
}
