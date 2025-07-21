/*
    11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
    ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
    ingresada por el usuario se encuentra en el arreglo.
*/

use std::io;

fn main() {
    let array = ["messi", "dybala", "neymar", "valen", "dobby"];

    let mut input = String::new();

    let mut exite : bool = false;

    io::stdin().read_line(&mut input).expect("Error al leer");

    let input = input.trim();

    for i in array.iter(){
        if i == &input{
            exite = true;
        }
    }
    if exite{
        println!("la cadena esta en el array");

    } else { println!("la cadena NO esta en el array"); }

}
