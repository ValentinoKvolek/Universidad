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
use std::collections::LinkedList;
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
enum Color {
    Blanco, Negro, Azul, Amarillo,Rojo, Verde
}

#[derive(Serialize, PartialEq, Clone)]
struct Auto{
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f64,
    color: Color

}


#[derive(Debug)]
pub enum ErrorAgregarAuto{
    AlmacenLLeno,
    ArchivoNoGuardado,
}

pub enum ErrorEliminarAuto{
    AutoNoExiste,
    ArchivoNoGuardado,
}
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
            if !self.reescribir_json_autos() { return Err(ErrorAgregarAuto::ArchivoNoGuardado) }
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
        if !self.reescribir_json_autos() { return Err(ErrorEliminarAuto::ArchivoNoGuardado) }
        Ok(auto)
    }

    pub fn buscar_auto(&mut self, marca:String, modelo:String, color: Color, ano:u32) -> Result<&Auto, ErrorBuscarAuto>{

        if let Some(i) = self.autos.iter().find(|auto| auto.marca == marca && auto.modelo == modelo && auto.anio == ano && auto.color == color) {
            Ok(i)
        }else{
            Err(ErrorBuscarAuto::AutoNoExiste)
        }
    }


    pub fn reescribir_json_autos(&self) -> bool {
        // c. reescribir el archivo con la información del vector de autos
        match serde_json::to_string_pretty(&self.autos) {
            Ok(res) => {
                if fs::write("autos.json", res).is_err() {
                    return false
                }
                true
            }
            Err(_) => { false }
        }
    }
}


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

    use super::*;

    fn concesionario(capacity: usize) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre: "valenAutos".to_string(),
            direccion: "Calle falsa 123".to_string(),
            autos: Vec::with_capacity(capacity)
        }
    }

    #[test]
    fn test_agregar() {
        
        
    }

    #[test]
    fn test_eliminar() {

    }
}

fn main() {}