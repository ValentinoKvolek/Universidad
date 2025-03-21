use std::io;
fn main() {
    let mut input = String::new();
    let mut operacion = String::new();
    let y: f32 = 3.0; // variable flotante

    println!("Ingrese un valor: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");

    // entrada a tipo : f32
    let input: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Número inválido");
            return;
        }
    };

    println!("Ingrese una operación: 1 (multiplicar), 2 (dividir), 3 (sumar), 4 (restar): ");
    io::stdin()
        .read_line(&mut operacion)
        .expect("Error al leer la entrada");

    // operación a u8
    let operacion: u8 = match operacion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Operación inválida");
            return;
        }
    };

    //elegir con match
    let resultado = match operacion {
        1 => multiplicar(input, y),
        2 => dividir(input, y),
        3 => sumar(input, y),
        4 => restar(input, y),
        _ => {
            println!("Operación no válida");
            return;
        }
    };

    println!("El resultado es: {}", resultado);
}



fn multiplicar(num: f32, x: f32) -> f32 {
    num * x
}

fn dividir(num: f32, x: f32) -> f32 {
    if x == 0.0 {
        println!("No se puede dividir por 0");
        return 0.0;
    }
    num / x
}

fn sumar(num: f32, x: f32) -> f32 {
    num + x
}

fn restar(num: f32, x: f32) -> f32 {
    num - x
}
