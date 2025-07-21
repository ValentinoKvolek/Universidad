/*

9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive

*/

fn main() {


}

fn cantidad_en_rango(array:Vec<i32>, inf:i32, sup:i32) -> i32 {
    array.iter().filter(|&&x| x >= inf && x <= sup).count() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_en_rango_0() {
    let array = vec![1, 2, 3, 4, 5];
    let inferior = 0;
    let superior = 0;
    assert_eq!(cantidad_en_rango( array, inferior, superior ), 0, "deberia ser 0");
    }

    #[test]
    fn test_cantidad_en_rango_5() {
        let array = vec![1, 2, 3, 4, 5];
        let inferior = 1;
        let superior = 5;
        assert_eq!(cantidad_en_rango( array, inferior, superior ), 5,"deberia ser 5");
    }
}
