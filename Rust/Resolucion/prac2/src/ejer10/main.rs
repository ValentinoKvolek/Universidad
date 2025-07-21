/*

10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite


*/


fn main() {}

fn cantidad_de_cadenas_mayor_a(array :Vec<String>, x : i32) -> u32 {
    let mut suma = 0;
    for i in array.iter() {
        if i.len() > x as usize {
            suma+=1;

        }
    }
    suma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_0() {
        let array_strig  = vec!["fermin".to_string(),"valen".to_string(),"yas".to_string(),"yoni".to_string(),"jp".to_string()];
        assert_eq!(cantidad_de_cadenas_mayor_a(array_strig.clone(), 6), 0);
        assert_eq!(cantidad_de_cadenas_mayor_a(array_strig, 1), 5);
    }
}