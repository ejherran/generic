use std::mem;

fn main() {
    let x: i8 = -128; // Entero con signo de 8 bits.
    let y: u8 = 255; // Entero sin signo de 8 bits.

    let zx: isize = -981; // Entero con signo de tamaño automatico.
    let zy: usize = 819; // Entero sin signo de tamaño automatico.

    let mi: usize = 1000000000000;
    let size = mem::size_of_val(&mi); // Calcula el tamaño en memoria de un valor. &mi es una referencia a la variable mi.

    let fx: f32 = -56.89; // Flotante de 32 bits.
    let fy: f64 = 0.33333333333333333; // Flotante de 64 bits.

    let hexadecimal = 0x2b; // Entero en formato hexadecimal.
    let octal = 0o14; // Entero en formato octal.
    let binario = 0b10101010; // Entero en formato binario.

    let c: char = 'Ñ'; // Variable de tipo caracter.
    let zc = mem::size_of_val(&c); // Tamaño de una variable de tipo caracter.

    let b: bool = true; // Variable de tipo booleano.
    let zb = mem::size_of_val(&b); // Tamaño de una varible de tipo booleano.

    let largo: u64 = 999_999_999_999_999_999; // Para números largo rust permite usar el caracter _ como delimitador de lectura.

    println!("x = {}", x);
    println!("y = {}", y);
    println!("zx = {}", zx);
    println!("zy = {}", zy);
    println!(
        "Tamaño de memoria (bits) para mi = {} -> {} Bits",
        mi,
        size * 8
    );
    println!("Flotante de 32 < {} > - Flotante de 64 < {} >", fx, fy);
    println!(
        "Hexadecimal 2B = {} - Octal 14 = {} - Binario 10101010 = {}",
        hexadecimal, octal, binario
    );
    println!("Caracter '{}' de tamaño {} bits.", c, zc * 8);
    println!("Buleano '{}' de tamaño {} bits.", b, zb * 8);
    println!("Largo = {}", largo);
}
