use std::f64::consts; // Acceso a las constantes en formato flotante de 64 bits.

fn main() {
    let a: i64 = -888; // Definir variables con tipo conocido.
    let b = 64; // Definir variables con tipo inferido.

    let mut c = 0; // En rust las variables son inmutables. Si se desea asignar diferentes valores a ua variable se debe marcar con mut.
    c = c + 1;

    let d = 9; // Rust permite definir varias veces la misma variable, a estos se le llama Shadowing.
    let d = d / 2; // La división de enteros en rust es siempre entera.

    const SIZE: u8 = 64; // Las constantes deben escribirse en mayusculas y deben definir el tipo.

    let radio: f64 = 5.0;
    let area = consts::PI * radio.powf(2.0);
    println!("A = {} | B = {}", a, b);
    println!("C = {}", c);
    println!("D = {}", d);
    println!("SIZE = {}", SIZE);
    println!("Área = {}", area);
}
