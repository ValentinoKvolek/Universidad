/*3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
and y or. Se deben imprimir ambos resultados. */

use std::io;

fn main() {

    let mut bool_and : bool = true;
    let mut bool_or : bool = true;

    let mut aux = String :: new(); //valor aux;

    println!("Ingrese valor booleano (1 // 0)");

    io::stdin()
    .read_line(&mut aux)
    .expect("Error al leer la entrada");

    let input: u8 = match aux.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Operación inválida");
            return
        }
    };
    
    println!("el valor del booleano haciendo un AND es: {}", and_gate(&input, bool_and));
    println!("el valor del booleano haciendo un OR es : {}", or_gate(&input, bool_or));
}

fn and_gate(num:&u8, mut b:bool) -> bool {
    if (*num == 0){
        b = false;
    }
    b
}
fn or_gate(num:&u8, mut b:bool) -> bool{
    if (*num == 1){ // desreferencio la varible ya que para &u8 no esta implementada  el partialeq{integer}
        b = true;
    }
    b
}
