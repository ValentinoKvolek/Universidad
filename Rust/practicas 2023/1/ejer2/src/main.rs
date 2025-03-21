/*2- Escribir un programa que defina una variable de tipo entero sin signo, y luego imprima su
valor en hexadecimal. */

fn main() {
    let x : u32  = 13548;
    let hex = format!("{:x}", x); 
    println!("el valor {} en hexa es : {}", x, hex);
}