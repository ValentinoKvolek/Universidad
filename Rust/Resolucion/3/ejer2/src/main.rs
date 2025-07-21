/*

2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo retorna.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario

*/

struct Rectangulo{

    longitud: f32, ancho: f32

}

fn main() {
    
}

impl Rectangulo {
    fn new(longitud: f32, ancho: f32) -> Result<Rectangulo, String> {

        if longitud < 0 as f32 { return Err("Se ingreso una longitud menor a 0".to_string())}
        if ancho < 0 as f32 { return Err("Se ingreso una anchor menor a 0".to_string())}
        Ok(Rectangulo { longitud, ancho })

    }

    fn calcular_area(&self) -> f32 {
        self.longitud * self.ancho
    }

    fn calcular_perimetro(&self) -> f32 {
        2.0 * (self.longitud + self.ancho)
    }

    fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    fn rectangulo_nuevo()-> Rectangulo{
        Rectangulo{
            longitud: 10.0,
            ancho: 20.0,
        }
    }

    #[test]
    fn test_crear_rectangulo() {
        assert_eq!(rectangulo_nuevo().longitud, 10.0);
        assert_eq!(rectangulo_nuevo().ancho, 20.0);
        assert_eq!(rectangulo_nuevo().calcular_perimetro(), 60.0);
        assert_eq!(rectangulo_nuevo().es_cuadrado(), false);
        assert_eq!(rectangulo_nuevo().calcular_area(), 200.0);
    }
    #[test]
    fn test_error_al_crear(){
        let rectangulo = Rectangulo::new(-10.0, 20.0);
        assert!(rectangulo.is_err());
    }
    #[test]
    fn test_es_cuadro() {
        let rectangulo = Rectangulo::new(10.0, 10.0).unwrap();
        assert_eq!(rectangulo.es_cuadrado(), true);
    }
}
