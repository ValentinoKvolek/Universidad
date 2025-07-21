/*
    12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
    enteros, y luego imprima la cadena y la suma de los valores en el arreglo
*/





fn main() {
    let datos: (&str, [i32; 5]) = ("Números", [1, 2, 3, 4, 5]);
    let suma: i32 = datos.1.iter().sum();
    println!("Cadena: {}", datos.0);
    println!("Suma de los valores: {}", suma);
}
