/*6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
usuario ingresar un número entero por teclado para sumarse con la variable definida. El
programa debe imprimir el valor del número elevado al cuadrado. */
use std::io;

fn main() {

    let num = 10; // Ejemplo de número base
    let mut input = String::new();

    println!("ïngrese un numero");

    io::stdin() //leer
    .read_line(&mut input)
    .expect("Error al leer la entrada");

    let input = input.trim(); 
    let input =  input.parse::<u32>();

    println!("El resultado es: {:?}", resultado(num, input.unwrap()));


    
}

fn resultado(num1:u32, num2:u32)-> u32{
    let  aux : u32;
    aux = num1 + num2;
    
    aux * aux
}