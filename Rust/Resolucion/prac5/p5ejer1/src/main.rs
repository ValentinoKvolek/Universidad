/*

7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo, verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:
❖ ConcesionarioAuto:
    ➢ new: que pasando los parámetros correspondientes, crea un ConcesionarioAuto y lo retorna.
    ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar la máxima cantidad para albergarlos y retorna true, en caso de que lo supere no lo agrega y retorna false.
    ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
    ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
❖ Auto:
    ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo retorna.
    ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios: ■ si es de color primario le aplica un recargo del 25%, sino le aplica un descuento del 10%.
                                                                                      ■ si la marca es BMW le aplica un recargo del 15%■ si el año es menor a 2000 le aplica un descuento del 5%.

*/
use std::fs;
use serde::Serialize;

#[derive(Serialize, PartialEq, Clone)]

struct ConcesionarioAuto{
    nombre: String,
    direccion: String,
    autos: Vec<Auto>
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Serialize)]
#[derive(Debug)]
enum Color {
    Blanco, Negro, Azul, Amarillo,Rojo, Verde
}

#[derive(Serialize, PartialEq, Clone, Debug)]
struct Auto{
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f64,
    color: Color
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorAgregarAuto{
    AlmacenLLeno,
    ArchivoNoGuardado,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorEliminarAuto{
    AutoNoExiste,
    ArchivoNoGuardado,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum  ErrorBuscarAuto{
    AutoNoExiste,
}



impl ConcesionarioAuto{

    pub fn new(nombre: String, direccion: String, cant_max_almacen: usize, autos: Option<Vec<Auto>> ) -> ConcesionarioAuto{
        let autos = match autos {
            Some(autos) => autos,
            None => Vec::with_capacity(cant_max_almacen),
        };
        // En mi opinion una consecionaria ya tieene autos. en el caso de que no tenga(rarisimo) creo un vector nuevo con dimF de cant max capacidad.
        ConcesionarioAuto{
            nombre,
            direccion,
            autos
        }
    }
    pub fn agregar_auto(&mut self, auto_nuevo: Auto) -> Result<bool, ErrorAgregarAuto> {
        if self.autos.len() < self.autos.capacity(){
            self.autos.push(auto_nuevo);
            let path = "C:/Users/valen/OneDrive/autos_test.json";
            if !self.reescribir_json_autos(path) { return Err(ErrorAgregarAuto::ArchivoNoGuardado) }
            return Ok(true)
        }
        Err(ErrorAgregarAuto::AlmacenLLeno)
    }

    pub fn eliminar_auto(&mut self, marca:String, modelo:String, color: Color, ano:u32) -> Result<Auto, ErrorEliminarAuto>{
        let index = if let Some(i) = self.autos.iter().position(|auto| auto.marca == marca && auto.modelo == modelo && auto.anio == ano && auto.color == color) {
            i
        }else{
            return Err(ErrorEliminarAuto::AutoNoExiste)
        };
        let auto =self.autos.remove(index);
        let path = "C:/Users/valen/OneDrive/autos_test.json";
        if !self.reescribir_json_autos(path) { return Err(ErrorEliminarAuto::ArchivoNoGuardado) }
        Ok(auto)
    }

    pub fn buscar_auto(&mut self, marca:String, modelo:String, color: Color, ano:u32) -> Result<&Auto, ErrorBuscarAuto>{

        if let Some(i) = self.autos.iter().find(|auto| auto.marca == marca && auto.modelo == modelo && auto.anio == ano && auto.color == color) {
            Ok(i)
        }else{
            Err(ErrorBuscarAuto::AutoNoExiste)
        }
    }


    pub fn reescribir_json_autos(&self, path:&str) -> bool {
        // c. reescribir el archivo con la información del vector de autos
        match serde_json::to_string_pretty(&self.autos) {
            Ok(res) => {
                if fs::write(path, res).is_err() {
                    return false
                }
                true
            }
            Err(_) => { false }
        }
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorCrearAuto{
    AnoInvalido,
    PrecioInvalido,
}

impl Auto{

    pub fn new(marca:String, modelo: String, anio:u32, precio_bruto:f64, color:Color) -> Result<Auto, ErrorCrearAuto>{
        if anio < 1885{
            return Err(ErrorCrearAuto::AnoInvalido)
        }
        if precio_bruto == 0.0{
            return Err(ErrorCrearAuto::PrecioInvalido)
        }
        Ok(Auto{
            marca,
            modelo,
            anio,
            precio_bruto,
            color
        })
    }

    pub fn calcular_precio(&self) -> f64 {

        let mut precio = self.precio_bruto;

        // ■ si es de color primario le aplica un recargo del 25%, sino le aplica un descuento del 10%.
        match self.color {
            Color::Rojo | Color::Azul | Color::Amarillo => precio*= 1.25,
            _ => precio/= 0.9
        }

        // ■ si la marca es BMW le aplica un recargo del 15%
        if self.marca == "BMW" { precio*= 1.15 }

        // ■ si el año es menor a 2000 le aplica un descuento del 5%.
        if self.anio  < 2000 { precio*= 0.95 }

        precio
    }
}




#[cfg(test)]
mod tests {
    use std::panic::AssertUnwindSafe;
    use super::*;

    fn concesionario(capacity: usize) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre: "valenAutos".to_string(),
            direccion: "Calle falsa 123".to_string(),
            autos: Vec::with_capacity(capacity)
        }
    }
    
    #[test]
    fn crear_concecionario(){
        let c = ConcesionarioAuto::new("valen".to_string(), "calle falsa12".to_string(), 3, None);
        assert_eq!(c.nombre, "valen".to_string());

        let auto1 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo).unwrap();
        let auto2 = Auto::new("Ford".to_string(), "Fiesta".to_string(), 2010, 20000.0, Color::Negro).unwrap();
        let auto3 = Auto::new("Chevrolet".to_string(), "Onix".to_string(), 2015, 25000.0, Color::Blanco).unwrap();
        let auto4 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 1999, 30000.0, Color::Verde).unwrap();

        let autos = vec![auto1, auto2, auto3, auto4];

        let c2 = ConcesionarioAuto::new("valen".to_string(), "calle falsa12".to_string(), 4, Some(autos.clone()));

        assert_eq!(c2.autos.capacity(), 4);

    }
    #[test]
    fn test_agregar() {
        let mut c = concesionario(1);

        let auto1 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo).unwrap();
        c.agregar_auto(auto1);
        assert_eq!(c.autos.len(), 1);

        let auto2 = Auto::new("Ford".to_string(), "Fiesta".to_string(), 2010, 20000.0, Color::Negro).unwrap();
        let result = c.agregar_auto(auto2);

        match result {
            Ok(r) => {
                panic!("deberia dar error");
            }
            Err(e) =>{
                assert_eq!(e, ErrorAgregarAuto::AlmacenLLeno);
            }
        }
    }
    #[test]
    fn test_eliminar() {
        let mut c = ConcesionarioAuto::new("valen".to_string(), "calle falsa12".to_string(), 3, None);
        let auto1 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo).unwrap();
        c.agregar_auto(auto1.clone());
        let result = c.eliminar_auto("BMW".to_string(), "X5".to_string(), Color::Rojo, 2020);
        match result {
            Ok(r) => {
                assert_eq!(r, auto1);
            }
            Err(e) => {
                panic!("no deberia dar error {:?}", e);
            }
        }
        let result = c.eliminar_auto("BMW".to_string(), "X5".to_string(), Color::Rojo, 2020);
        match result {
            Ok(r) => {
                panic!(" deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorEliminarAuto::AutoNoExiste);
            }
        }
    }
    #[test]
    fn test_buscar_auto(){
        let mut c = ConcesionarioAuto::new("valen".to_string(), "calle falsa12".to_string(), 3, None);
        let auto1 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo).unwrap();
        c.agregar_auto(auto1.clone());
        let result = c.buscar_auto("BMW".to_string(), "X5".to_string(), Color::Rojo, 2020);
        match result {
            Ok(r) => {
                assert_eq!(*r, auto1);
            }
            Err(e) => {
                panic!("no deberia dar el error {:?}", e)
            }
        }
        let result = c.buscar_auto("Audi".to_string(), "X5".to_string(), Color::Rojo, 2020);
        match result {
            Ok(r) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorBuscarAuto::AutoNoExiste)
            }
        }
    }
    #[test]
    fn escritura_de_arch(){
        const TEST_FAKE_PATH: &str = "C:/Users/valen/OneDrive/autos_test.json";
        let c = ConcesionarioAuto::new("valen".to_string(), "calle falsa12".to_string(), 3, None);
        let result: bool = c.reescribir_json_autos(TEST_FAKE_PATH);
        assert!(result);
        const TEST_FAKE_PATH_ERROR: &str = "C:/nadaquever/hola";
        let result: bool = c.reescribir_json_autos(TEST_FAKE_PATH_ERROR);
        assert!(!result);
    }
    #[test]
    fn  test_crear_auto(){
        let auto = Auto::new("BMW".to_string(), "X5".to_string(), 1884, 50000.0, Color::Rojo);
        match auto {
            Ok(a) => {
                panic!("deberia ser error!");
            }
            Err(e) => {
                assert_eq!(e, ErrorCrearAuto::AnoInvalido)
            }

        }
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2000, 0.0, Color::Rojo);
        match auto2 {
            Ok(a) => {
                panic!("deberia ser error!");
            }
            Err(e) => {
                assert_eq!(e, ErrorCrearAuto::PrecioInvalido)
            }

        }
        let auto3 = Auto::new("BMW".to_string(), "X5".to_string(), 2000, 1000.0, Color::Azul);
        match auto3 {
            Ok(a) => {
                assert_eq!(a.marca, "BMW".to_string() );
            }
            Err(e) => {
                panic!("deberia ser error {:?}",e); 
            }

        }
    }

    #[test]
    fn test_calcular_precio() {
        
        let auto1 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 100_000.0, Color::Rojo).unwrap();
        let precio1 = auto1.calcular_precio();
        // 100_000 * 1.25 * 1.15 = 143_750
        assert!((precio1 - 143_750.0).abs() < f64::EPSILON);
        
        let auto2 = Auto::new("Ford".to_string(), "Focus".to_string(), 2010, 50_000.0, Color::Negro).unwrap();
        let precio2 = auto2.calcular_precio();
        // 50_000 / 0.9 = 55_555.55
        assert!((precio2 - 55_555.555555).abs() < 1.0);
        
        let auto3 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 1999, 40_000.0, Color::Azul).unwrap();
        let precio3 = auto3.calcular_precio();
        // 40_000 * 1.25 * 0.95 = 47_500
        assert!((precio3 - 47_500.0).abs() < f64::EPSILON);
        
        let auto4 = Auto::new("BMW".to_string(), "Serie 3".to_string(), 1995, 80_000.0, Color::Blanco).unwrap();
        let precio4 = auto4.calcular_precio();
        // 80_000 / 0.9 * 1.15 * 0.95 = ≈ 96_888.888
        let esperado = 80_000.0 / 0.9 * 1.15 * 0.95;
        assert!((precio4 - esperado).abs() < 1.0);
    }

}
fn main() {}