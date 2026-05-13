/**
 * retorna un vector de números del 1 al 5. Este es un ejemplo simple para demostrar cómo se pueden retornar datos desde una función asincrónica en Rust. En un caso real, podrías estar obteniendo estos números de una base de datos o realizando algún tipo de cálculo antes de retornarlos.
 */
pub async fn numero() -> Vec<u8> {
    let mut numeros = Vec::new();
    for i in 1..=100 {
        //nuemro aleatorio entre 1 y 100
        numeros.push(rand::random::<u8>() % 100 + 1);
    }
    numeros
}


//guardar nombre en txt
/**
 * para que es el ansync en este metodo? 
 * el async es necesario para que la función pueda ser llamada desde 
 * un contexto asincrónico, 
 * como un controlador de ruta en Axum. 
 * Aunque las operaciones de escritura en archivos 
 * son generalmente bloqueantes, al marcar la función como async, 
 * permite que el servidor maneje otras solicitudes 
 * mientras se realiza la operación de escritura,
 *  mejorando así la eficiencia y capacidad de respuesta del servidor.
 */
pub async fn guardar_nombre(nombre: &str) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("nombres.txt")?;

    writeln!(file, "{}", nombre)?;
    Ok(())
}

//leer nombres guardados en txt
pub async fn leer_nombres() -> std::io::Result<Vec<String>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("nombres.txt")?;
    let reader = BufReader::new(file);
    let mut nombres = Vec::new();
    for line in reader.lines() {
        nombres.push(line?);
    }
    Ok(nombres)
}