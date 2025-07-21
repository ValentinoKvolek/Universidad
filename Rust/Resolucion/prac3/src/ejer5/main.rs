/*

5- Escribir un programa que defina una estructura Producto que tenga campos para el nombre, el precio bruto y un número identificatorio.
Para dicha estructura implemente los siguientes métodos:

➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre el precio bruto
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de descuento sobre el precio bruto
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los parámetros son opcionales.

*/

#[derive(Clone)]
struct Producto{
    nombre:String, precio_bruto:f64,  num_id:u8
}

impl Producto {


    fn new( nombre: String,  precio_bruto: f64,  num_id:u8) -> Result<Producto,String> {
        
        if (precio_bruto < 0.0){
            return Err(String::from("precio_bruto Negativo"));
        }
        
        Ok(Producto{
            nombre,
            precio_bruto,
            num_id
        })
    }

    fn calcular_impuesto(&self, porcentaje:f64)-> f64 {

        self.precio_bruto * ( porcentaje / 100.0)

    }

    fn aplicar_descuento(&self, descuento:f64)-> f64 {

        self.precio_bruto * (1.0  - (descuento / 100.0))

    }

    fn calcular_precio_total(&self, porcentaje:Option<f64>, descuento:Option<f64>) -> f64 {

        let mut precio = self.precio_bruto;

       precio+= match porcentaje {
            Some(porcentaje)=> self.calcular_impuesto(porcentaje),
            None => 0.0,
        };


        precio-= match descuento {
            Some(des)=> self.aplicar_descuento(des),
            None => 0.0,
        };
        
        precio

    }

}

#[cfg(test)]
mod tests {
    use super::*;
    fn nuevo_producto() -> Producto {
        Producto {
            num_id: 60,
            nombre: "notebook".to_string(),
            precio_bruto: 2000.0
        }
    }
    #[test]
    fn test_producto() {
        assert_eq!(nuevo_producto().calcular_impuesto(500.0), 10000.0);
        assert_eq!(nuevo_producto().calcular_impuesto(30.0), 600.0);
        assert_eq!(nuevo_producto().calcular_precio_total(Some(50.0), Some(50.0)), 2000.0);
    }
}

fn main() {

}




