
/*

5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro


*/

fn main() {

}

fn duplicar_valores(vec:Vec<f64>) -> Vec<f64> {

   let mut nuevo_vector:Vec<f64> = vec![];

    for i in 0..vec.len() {
        nuevo_vector.push(vec[i] * 2.0);
    }

    nuevo_vector

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valores_duplicados(){

        let vec= vec!(10.3,20.5,30.3,40.2,50.3);

        let nuevo_vector = duplicar_valores(vec);

        assert_eq!(nuevo_vector[0], 20.6);
        assert_eq!(nuevo_vector[4], 100.6);
        assert_eq!(nuevo_vector[3], 80.4);

    }
}
