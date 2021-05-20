fn main() {
    let a = "Hola"; // La variable a esta en ele alcance de la función main.

    {
        let a = "Mundo"; // Esta variable a es local al alcance de este bloque y tiene prioridad sobre la variable a de la función main.
        println!("A dentro = {}", a);
    }

    println!("A fuera = {}", a); // El valor de la variable a exterior no se ve modificada por la asignación interna al bloque.
}
