/*4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
con signo y un valor booleano, y luego imprima cada valor de la tupla */

fn main() {
    let tup:(&str, i32 , bool ) = ("fermin"  , -87, false);
    println!("1 {}. 2{}. 3{}", tup.0, tup.1, tup.2); // no uso for por que en un tupla no te permite iterar.
}

