use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let i: (i8, i8) = (rng.gen(), rng.gen());
    println!("Seed: {:?}", i);

    match i {
        (-128, -128) | (-128, 127) | (127, 127) | (127, -128) => println!("Is the limit"), // Se pueden probar varios casos con |.
        (0, 0) => println!("Is the origin"), // Es posible crear un patron con valores fijos.
        (_x, 0) => print!("It is on the horizontal axis."), // Se puede dejar un valor fijo y otro irrelevante que se marca con _ al inicio.
        (0, _y) => println!("It is on the vertical axis."),
        (x, y) if (x.abs() <= 25) && (y.abs() <= 25) => println!("Is in the zone"), // Es posible hacer condiciones usando las variables del patron.
        (x, y) if x > 0 && y > 0 => println!("t is in the first quadrant."),
        (x, y) if x > 0 && y < 0 => println!("t is in the second quadrant."),
        (x, y) if x < 0 && y < 0 => println!("t is in the third quadrant."),
        (x, y) if x < 0 && y > 0 => println!("t is in the fourth quadrant."),
        _ => println!("Nothing special."), // En este ejemplo los patrones cubren todos los casos, así que el patron general nunca se alcanza.
    }
}
