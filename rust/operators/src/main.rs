fn main() {
    // Aritmeticos

    let a: i32 = 10;
    let b: i32 = 3;
    let c: i32 = 256;
    let d: f64 = 1.0 / 4.0;

    println!("Suma = {}", a + b); // a + b
    println!("Resta = {}", a - b); // a - b
    println!("Multiplicación = {}", a * b); // a * b
    println!("Cociente = {}", a / b); // a / d
    println!("Residuo = {}", a % b); // a % d
    println!("Potencia 10^3 = {}", a.pow(b as u32)); // a ^ b. B debe ser un entero sin signo, por eso se fuerza el cambio a u32.
    println!("Raiz 4 de 256 = {}", f64::from(c).powf(d)); // Las raices se calculan como potencias de exponente fracionario. Raíz 4 de 256 es 256 ^ (1/4) o 256 ^ 0.25.

    // Bitwise

    let e: u8 = 0b1001; // Enteros de 8 bits en formato binario.
    let f: u8 = 0b1111;
    let g: u8 = 0b0110;
    let h: u8 = 0b11111111;

    println!("E      = {:08b}", e); // {:08b} se utiliza para imprimir con formato de byte, completando siempre con 8 los 8 bits.
    println!("E | F  = {:08b}", e | f); // Operación OR bit a bit.
    println!("E & F  = {:08b}", e & f); // Operación AND bit a bit.
    println!("E ^ F  = {:08b}", e ^ f); // Operación XOR bit a bit.
    println!("NOT G  = {:08b}", !g); // Operación NOT bit a bit.
    println!("H << 2 = {:08b}", h << 2); // Desplazamiento izquierdo de bits.
    println!("H >> 2 = {:08b}", h >> 2); // Desplazamiento derecho de bits.

    // Comparación

    let i = 9;
    let j = 3;
    let k: f64 = 9.0;
    let l: f32 = 3.0;

    println!("I == J {}", i == j); // Operador igualdad
    println!("I != J {}", i != j); // Operador diferencia
    println!("I >  K {}", f64::from(i) > k); // Operador >. El tipo de los datos a comparar debe ser igual
    println!("I >= K {}", f64::from(i) >= k); // Operador >=.
    println!("J <  L {}", (j as f32) < l); // Operador <. El tipd de los datos a comparar debe ser exactamente igual, tanto en tipo como en longitud.
    println!("J <= L {}", (j as f64) <= f64::from(l)); // Operador <=.
}
