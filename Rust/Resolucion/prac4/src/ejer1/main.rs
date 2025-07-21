
//1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de
// números primos. Cree un trait para la determinación del número primo e impleméntelo
// según corresponda. Utilice la función iter sobre el vector y aplique un closure para
// resolverlo


fn main() {
}
trait NumPrimos {
    fn es_primo(&self) -> bool;

}

impl<T> NumPrimos for T where T: Copy + Into<i128>  {   //agrego los requisitos necesarios para T

    fn es_primo(&self) -> bool {

        let num : i128 =  (*self).into();

        if num <= 1 {
            return false;
        }
        for i in 2 ..=(num.isqrt()) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

}
fn contar_primos<T: NumPrimos>(numeros:Vec<T>) -> usize{

    numeros.iter().filter(|&x| x.es_primo()).count()

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_es_primo() {

        let mi_vect = vec![2, 3, 4, 5, 6, 7, 11, 13, 15];;

        let cantidad = contar_primos(mi_vect);
        assert_eq!(cantidad, 6);

    }

    #[test]
    fn test_contar_primos_0() {


        let mi_vect = vec! [0, 1, 4, 6, 8, 9, 10];
        let cantidad = contar_primos(mi_vect);
        assert_eq!(cantidad, 0);
    }
}
