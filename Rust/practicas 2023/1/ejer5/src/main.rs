/*5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
cadena en mayúsculas. */
use std::io;

fn main() {

    let mut cadena =  String :: from("pelado");

    let mut input = String::new();

    println!("Ingrese una cadena de texto: ");
    io::stdin() //leer
        .read_line(&mut input)
        .expect("Error al leer la entrada");

    let input = input.trim(); 


    cadena.push_str(" ");
    cadena.push_str(input);
    println!("{}", cadena);

}
