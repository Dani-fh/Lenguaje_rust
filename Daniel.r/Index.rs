

fn main() {
    // Obtener la hora actual
    let ahora = Local::now();
    
    // Imprimir "Hola, mundo!" junto con la hora actual
    println!("Hola, mundo! La hora actual es: {}", ahora.format("%H:%M:%S"));

    // Crear una variable para almacenar el nombre
    let mut nombre = String::new();

    // Pedir al usuario que ingrese su nombre
    println!("Por favor, ingresa tu nombre:");

    // Leer la entrada del usuario
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la línea");

    // Eliminar el salto de línea al final
    let nombre = nombre.trim();

    // Imprimir el saludo personalizado
    println!("Hola, {}!", nombre);
}
