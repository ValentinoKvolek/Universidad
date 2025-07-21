/*

4- Escribir un programa que defina la estructura Triángulo que tenga campos para las longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero, isósceles o escaleno.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.

*/

#[derive(Debug)]
#[derive(PartialEq)]
enum Representacion{
    Isoseles,
    Escaleno,
    Equilatero,
}
struct Triangulo{
    lado_1:f64,
    lado_2:f64,
    lado_3:f64
}


fn main() {

}

impl Triangulo {



    fn new(lado_1:f64,lado_2:f64,lado_3:f64) -> Result<Triangulo,String> {

        if (lado_1 < 0.0|| lado_2< 0.0 || lado_3 < 0.0){
            return Err("se esperaba un lado positivo".to_string());
        }

        Ok( Triangulo{ lado_1, lado_2, lado_3 } )
    }

    fn determinar_tipo(&self)-> Representacion{

        let mut contador:u8 = 0;

        if self.lado_1 == self.lado_2 || self.lado_1 == self.lado_3 {
            contador += 1;
        }
        if self.lado_2 == self.lado_3 || self.lado_2 == self.lado_1 {
            contador += 1;
        }
        if self.lado_3 == self.lado_1 || self.lado_3 == self.lado_2 {
            contador += 1;
        }

        if(contador == 2){
            return Representacion::Isoseles
        }else if contador == 3{  return Representacion::Equilatero;
        }else { return Representacion::Escaleno }

    }

    fn calcular_area(&self)-> f64 {

        let s: f64 = (self.lado_1 + self.lado_2 + self.lado_3) / 2.0;;

        let area = (s*(s - self.lado_1)*(s - self.lado_2) * (s - self.lado_3)).sqrt();

        area
    }


    fn calc_perimetro(&self)-> f64 {

        let perimetro = self.lado_1 + self.lado_2 + self.lado_3;();

        perimetro
    }

}

#[cfg(test)]
mod tests {
    use super::*;


    fn triangulo()->Triangulo{
        Triangulo{
            lado_1:3.0,
            lado_2:4.0,
            lado_3:5.0,
        }
    }
    
    fn triangulo_isoseles()->Triangulo{
        Triangulo{
            lado_1:2.0,
            lado_2:2.0,
            lado_3:3.0,
        }
    }
    fn triangulo_equlatero()->Triangulo{
        Triangulo{
            lado_1:2.0,
            lado_2:2.0,
            lado_3:2.0,
        }
    }


    #[test]

    fn test_triangulo() {
        let tipo = triangulo().determinar_tipo();
        assert_eq!(tipo, Representacion::Escaleno);
        assert_eq!(triangulo().calcular_area(), 6.0 );
        assert_eq!(triangulo().calc_perimetro(), 12.0 );
    }

    #[test]
    fn test_determinar_tipo2() {
        let tipo =  triangulo_isoseles().determinar_tipo();
        assert_eq!(tipo, Representacion::Isoseles);
    }

    #[test]
    fn test_determinar_tipo3() {
        let tipo =  triangulo_equlatero().determinar_tipo();
        assert_eq!(tipo,  Representacion::Equilatero);

    }
    
}
