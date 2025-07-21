use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use crate::structs::atencion::{Atencion, Mascota};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Veterinaria {
    pub path:String,
    pub nombre:String,
    pub direccion: String,
    pub id: u64,
    pub cola: VecDeque<Mascota>,
    pub atenciones: Vec<Atencion>
}

const DEFAULT_FILE_PATH : &str = "C:/Users/valen/OneDrive/veterinaria_test.json";

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorAgregarMascota{
    NoHayMasTurnos,
    ErrorArchivo,
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorAtenderMascota{
    ColaVacia,
    ErrorArchivo,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorEliminarMascota{
    MascotaNoExiste,
    ErrorArchivo,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorRegistrarAtencion{
    VectorLLeno,
    ErrorArchivo,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorBuscarAtencion{
    AtencionNoExiste,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorEliminarAtencion{
    AtencionNoExiste,
    VectorVacio,
    ErrorArchivo,
}
impl Veterinaria {

    fn reescribir_archivo_json(&self, path: String)-> bool {
        let mut file = match File::create(format!("{}veterinaria.json", path)) {
            Ok(res) => { res }
            Err(_) => { return false }
        };

        let json_data = match serde_json::to_string_pretty(&self.atenciones) {
            Ok(res) => { res }
            Err(_) => { return false }
        };

        match file.write(json_data.as_bytes()) {
            Ok(res) => { true },
            Err(_) => { false}
        }
    }
    pub fn agregar_mascota(&mut self, mascota: Mascota) -> Result<bool, ErrorAgregarMascota> {
        if self.cola.len() == self.cola.capacity() {
            return Err(ErrorAgregarMascota::NoHayMasTurnos)
        }
        self.cola.push_back(mascota);
        let result = self.reescribir_archivo_json(self.path.parse().unwrap());
        if result {
            Ok(true)
        }else {
            Err(ErrorAgregarMascota::ErrorArchivo)
        }
    }
    pub fn agregar_mascota_prioridad(&mut self, mascota: Mascota) -> Result<bool, ErrorAgregarMascota> {
        if self.cola.len() == self.cola.capacity() {
            return Err(ErrorAgregarMascota::NoHayMasTurnos)
        }
        self.cola.push_front(mascota);

        let result = self.reescribir_archivo_json(self.path.parse().unwrap());
        if result {
            Ok(true)
        }else {
            Err(ErrorAgregarMascota::ErrorArchivo)
        }
    }

    pub fn atender_proxima_mascota(&mut self) -> Result<Mascota, ErrorAtenderMascota> {
        if let Some(mascota) = self.cola.pop_front() {
            Ok(mascota)
        }
        else { Err(ErrorAtenderMascota::ColaVacia)}
    }

    pub fn eliminar_mascota(&mut self, nombre_mascota:String, nombre_duenio: String) -> Result<Mascota, ErrorEliminarMascota> {
        let i = if let Some(i) = self.cola.iter().position(|m| m.nombre == nombre_mascota && m.dueno.nombre == nombre_duenio ) {
            i
        }
        else { return Err(ErrorEliminarMascota::MascotaNoExiste) };

        if let Some(mascota) = self.cola.remove(i) {
            Ok(mascota)
        } else {
            Err(ErrorEliminarMascota::MascotaNoExiste)
        }
    }

    pub fn registrar_atencion(&mut self, atencion: Atencion) -> Result<(), ErrorRegistrarAtencion> {

        if self.atenciones.len() == self.atenciones.capacity() { return Err(ErrorRegistrarAtencion::VectorLLeno) }
        self.atenciones.push(atencion);

        let result = self.reescribir_archivo_json(self.path.parse().unwrap());
        if result {
            Ok(())
        }else {
            Err(ErrorRegistrarAtencion::ErrorArchivo)
        }
    }

    pub fn buscar_atencion(&mut self, mascota_nombre: String, dueno_nombre: String, telefono: u64 ) -> Result<bool, ErrorBuscarAtencion> {
        match self.atenciones.iter_mut().find(|atencion|
            atencion.mascota.nombre == mascota_nombre &&
                atencion.mascota.dueno.nombre == dueno_nombre &&
                atencion.mascota.dueno.telefono == telefono
        ) {
            Some(_) => Ok(true),
            None => Err(ErrorBuscarAtencion::AtencionNoExiste),
        }
    }

    pub fn eliminar_atencion (&mut self, nombre_mascota:String, nombre_dueno: String, diagnostico:String) -> Result<Atencion, ErrorEliminarAtencion> {

        if self.atenciones.is_empty() {
           return  Err(ErrorEliminarAtencion::VectorVacio)
        }
        if let Some(i) = self.atenciones.iter().position( |a| a.mascota.nombre == nombre_mascota && a.mascota.dueno.nombre == nombre_dueno && a.diagnostico == diagnostico) {
            let result = self.reescribir_archivo_json(self.path.parse().unwrap());
            if result {
                Ok(self.atenciones.remove(i))
            }else {
                Err(ErrorEliminarAtencion::ErrorArchivo)
            }

        } else {
           Err(ErrorEliminarAtencion::AtencionNoExiste)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::structs::atencion::{Animal, Dueno, Fecha};
    use super::*;
    fn veterinaria_de_valen(capacidad: usize) -> Veterinaria {

        let dueno1 = Dueno {
            nombre: "Valentino Franco".to_string(),
            direccion: "Calle 123".to_string(),
            telefono: 1234567890,
        };
        let dueno2 = Dueno {
            nombre: "Nahuel Pardo".to_string(),
            direccion: "Avenida 456".to_string(),
            telefono: 9876543210,
        };
        let dueno3 = Dueno {
            nombre: "Jonathan Hiriart".to_string(),
            direccion: "Carrera 789".to_string(),
            telefono: 5555555555,
        };

        let mascota1 = Mascota {
            nombre: "Dobby".to_string(),
            edad: 3,
            animal: Animal::Perro,
            dueno: dueno1.clone(),
        };
        let mascota2 = Mascota {
            nombre: "Saul".to_string(),
            edad: 2,
            animal: Animal::Gato,
            dueno: dueno2.clone(),
        };
        let mascota3 = Mascota {
            nombre: "Aslan".to_string(),
            edad: 5,
            animal: Animal::Caballo,
            dueno: dueno3.clone(),
        };

        let fecha1 = Fecha { dia: 15, mes: 10, anio: 2023 };
        let fecha2 = Fecha { dia: 20, mes: 10, anio: 2023 };
        let fecha3 = Fecha { dia: 25, mes: 10, anio: 2023 };

        let atencion1 = Atencion {
            mascota: mascota1.clone(),
            diagnostico: "Resfriado leve".to_string(),
            tratamiento: "Antibióticos".to_string(),
            proxima_visita: fecha1,
        };
        let atencion2 = Atencion {
            mascota: mascota2.clone(),
            diagnostico: "Infección ocular".to_string(),
            tratamiento: "Gotas".to_string(),
            proxima_visita: fecha2,
        };
        let atencion3 = Atencion {
            mascota: mascota3.clone(),
            diagnostico: "Dolor de Pansa".to_string(),
            tratamiento: "Reposo".to_string(),
            proxima_visita: fecha3,
        };

        let mut cola: VecDeque<Mascota> = VecDeque::with_capacity(capacidad);
        cola.push_back(mascota1);
        cola.push_back(mascota2);
        cola.push_back(mascota3);

        let mut atenciones: Vec<Atencion> = Vec::with_capacity(capacidad);
        atenciones.push(atencion1);
        atenciones.push(atencion2);
        atenciones.push(atencion3);

        Veterinaria {
            path: DEFAULT_FILE_PATH.to_string(),
            nombre: "Lo que come tu Mascota".to_string(),
            direccion: "Calle Principal 100".to_string(),
            id: 1,
            cola,
            atenciones,
        }
    }

    fn veterinaria_de_valen_error_path(capacidad: usize) -> Veterinaria {

        let dueno1 = Dueno {
            nombre: "Valentino Franco".to_string(),
            direccion: "Calle 123".to_string(),
            telefono: 1234567890,
        };
        let dueno2 = Dueno {
            nombre: "Nahuel Pardo".to_string(),
            direccion: "Avenida 456".to_string(),
            telefono: 9876543210,
        };
        let dueno3 = Dueno {
            nombre: "Jonathan Hiriart".to_string(),
            direccion: "Carrera 789".to_string(),
            telefono: 5555555555,
        };

        let mascota1 = Mascota {
            nombre: "Dobby".to_string(),
            edad: 3,
            animal: Animal::Perro,
            dueno: dueno1.clone(),
        };
        let mascota2 = Mascota {
            nombre: "Saul".to_string(),
            edad: 2,
            animal: Animal::Gato,
            dueno: dueno2.clone(),
        };
        let mascota3 = Mascota {
            nombre: "Aslan".to_string(),
            edad: 5,
            animal: Animal::Caballo,
            dueno: dueno3.clone(),
        };

        let fecha1 = Fecha { dia: 15, mes: 10, anio: 2023 };
        let fecha2 = Fecha { dia: 20, mes: 10, anio: 2023 };
        let fecha3 = Fecha { dia: 25, mes: 10, anio: 2023 };

        let atencion1 = Atencion {
            mascota: mascota1.clone(),
            diagnostico: "Resfriado leve".to_string(),
            tratamiento: "Antibióticos".to_string(),
            proxima_visita: fecha1,
        };
        let atencion2 = Atencion {
            mascota: mascota2.clone(),
            diagnostico: "Infección ocular".to_string(),
            tratamiento: "Gotas".to_string(),
            proxima_visita: fecha2,
        };
        let atencion3 = Atencion {
            mascota: mascota3.clone(),
            diagnostico: "Dolor de Pansa".to_string(),
            tratamiento: "Reposo".to_string(),
            proxima_visita: fecha3,
        };

        let mut cola: VecDeque<Mascota> = VecDeque::with_capacity(capacidad);
        cola.push_back(mascota1);
        cola.push_back(mascota2);
        cola.push_back(mascota3);

        let mut atenciones: Vec<Atencion> = Vec::with_capacity(capacidad);
        atenciones.push(atencion1);
        atenciones.push(atencion2);
        atenciones.push(atencion3);

        Veterinaria {
            path: "nada/c/ss".to_string(),
            nombre: "Lo que come tu Mascota".to_string(),
            direccion: "Calle Principal 100".to_string(),
            id: 1,
            cola,
            atenciones,
        }
    }
    #[test]
    fn test_registrar_atencion() {
        let mut veterinaria = veterinaria_de_valen(4);
        let nueva_mascota = Mascota {
            nombre: "Marrillo".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Fermin".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233745784,
            }
        };
        let nueva_atencion = Atencion {
            mascota: nueva_mascota,
            diagnostico: "Fiebre".to_string(),
            tratamiento: "Ibuprofeno".to_string(),
            proxima_visita: Fecha {
                dia: 15,
                mes: 4,
                anio: 2025,
            }
        };
        let result = veterinaria.registrar_atencion(nueva_atencion);
        assert!(result.is_ok());
        let nueva_mascota2 = Mascota {
            nombre: "Haru".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Lucia".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233456,
            }
        };
        let nueva_atencion2 = Atencion {
            mascota: nueva_mascota2,
            diagnostico: "Pulgas".to_string(),
            tratamiento: "Vacuna".to_string(),
            proxima_visita: Fecha {
                dia: 10,
                mes: 2,
                anio: 2025,
            }
        };
        let result = veterinaria.registrar_atencion(nueva_atencion2);
        match result {
            Ok(result) => {
                panic!("Deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorRegistrarAtencion::VectorLLeno)
            }
        }
        let mut veterinaria2 = veterinaria_de_valen_error_path(10);

        let nueva_mascota = Mascota {
            nombre: "Marrillo".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Fermin".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233745784,
            }
        };
        let nueva_atencion = Atencion {
            mascota: nueva_mascota,
            diagnostico: "Fiebre".to_string(),
            tratamiento: "Ibuprofeno".to_string(),
            proxima_visita: Fecha {
                dia: 15,
                mes: 4,
                anio: 2025,
            }
        };
        let result = veterinaria2.registrar_atencion(nueva_atencion);
        match result {
            Ok(result) => {
                panic!("Deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorRegistrarAtencion::ErrorArchivo)
            }
        }

    }
    #[test]
    fn test_agregar_mascota() {

        let mut veterinaria = veterinaria_de_valen(4);
        let nueva_mascota = Mascota {
            nombre: "Marrillo".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Fermin".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233745784,
            }
        };
        let result  = veterinaria.agregar_mascota(nueva_mascota);
        match result {
            Ok(result) => {
                assert!(result)
            }
            Err(e) => {
                panic!("no deberia dar el error {:?}", e)
            }
        }

        let nueva_mascota2 = Mascota {
            nombre: "Haru".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Lucia".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233456,
            }
        };

        let result  = veterinaria.agregar_mascota(nueva_mascota2);
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorAgregarMascota::NoHayMasTurnos)
            }
        }

        let mut veterinaria2 = veterinaria_de_valen_error_path(10);
        let nueva_mascota = Mascota {
            nombre: "Marrillo".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Fermin".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233745784,
            }
        };

        let result  = veterinaria2.agregar_mascota(nueva_mascota);
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorAgregarMascota::ErrorArchivo)
            }
        }
    }
    #[test]
    fn test_agregar_mascota_con_prioridad(){

        let mut veterinaria = veterinaria_de_valen(4);
        let nueva_mascota = Mascota {
            nombre: "Marrillo".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Fermin".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233745784,
            }
        };
        let result  = veterinaria.agregar_mascota_prioridad(nueva_mascota.clone());
        match result {
            Ok(result) => {
                assert!(result);
                assert_eq!(*veterinaria.cola.get(0).unwrap(), nueva_mascota);
            }
            Err(e) => {
                panic!("no deberia dar el error {:?}", e)
            }
        }
        let nueva_mascota2 = Mascota {
            nombre: "Haru".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Lucia".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233456,
            }
        };
        let result  = veterinaria.agregar_mascota_prioridad(nueva_mascota2);
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorAgregarMascota::NoHayMasTurnos)
            }
        }

        let mut veterinaria2 = veterinaria_de_valen_error_path(10);
        let nueva_mascota = Mascota {
            nombre: "Marrillo".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Fermin".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233745784,
            }
        };

        let result  = veterinaria2.agregar_mascota_prioridad(nueva_mascota);
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorAgregarMascota::ErrorArchivo)
            }
        }

    }
    #[test]
    fn test_atender_proxima_mascota(){
        let mut veterinaria = veterinaria_de_valen(3);
        let result = veterinaria.atender_proxima_mascota();
        match result{
            Ok(result) => {
                assert_eq!(result.nombre, "Dobby".to_string());
            }
            Err(e) => {
                panic!{"no deberia dar el error {:?}", e}
            }
        }
        let result = veterinaria.atender_proxima_mascota();
        let result = veterinaria.atender_proxima_mascota();

        let result = veterinaria.atender_proxima_mascota();
        match result{
            Ok(result) => {
                panic!{"deberia dar el error"}
            }
            Err(e) => {
                assert_eq!(e,ErrorAtenderMascota::ColaVacia)
            }
        }

    }
    #[test]
    fn test_eliminar_mascota() {
        let mut veterinaria = veterinaria_de_valen(3);
        let result = veterinaria.eliminar_mascota("Dobby".to_string(), "Valentino Franco".to_string());
        match result {
            Ok(result) => {
                assert_eq!(result.nombre, "Dobby".to_string());
            }
            Err(e) => {
                panic!("no deberia dar el error {:?}", e)
            }
        }
        let result = veterinaria.eliminar_mascota("nada".to_string(), "Valentino Franco".to_string());
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e,ErrorEliminarMascota::MascotaNoExiste)
            }
        }
    }
    #[test]
    fn test_buscar_atencion(){
        let mut veterinaria = veterinaria_de_valen(3);
        let result = veterinaria.buscar_atencion("Dobby".to_string(),"Valentino Franco".to_string(),1234567890);
        match result {
            Ok(result) => {
                assert!(result)
            }
            Err(e) => {
                panic!("no deberia dar el error {:?}", e)
            }
        }
        let result = veterinaria.buscar_atencion("Doby".to_string(),"nada".to_string(),12345670);
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorBuscarAtencion::AtencionNoExiste)
            }
        }
    }
    #[test]
    fn test_eliminar_atencion(){
        let mut veterinaria = veterinaria_de_valen(3);

        let result  = veterinaria.eliminar_atencion("Dobby".to_string(), "Valentino Franco".to_string(),"Resfriado leve".to_string());
        match result {
            Ok(result) => {
                assert_eq!(result.mascota.nombre, "Dobby".to_string());
            }
            Err(e) => {
                panic!("no deberia dar el error {:?}", e)
            }
        }
        let result  = veterinaria.eliminar_atencion("nada".to_string(), "Nahuel Pardo".to_string(),"Infección ocular".to_string());
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorEliminarAtencion::AtencionNoExiste)
            }
        }
        let result  = veterinaria.eliminar_atencion("Saul".to_string(), "Nahuel Pardo".to_string(),"Infección ocular".to_string());
        let result  = veterinaria.eliminar_atencion("Aslan".to_string(), "Jonathan Hiriart".to_string(),"Dolor de Pansa".to_string());
        let result  = veterinaria.eliminar_atencion("Aslan".to_string(), "Jonathan Hiriart".to_string(),"Dolor de Pansa".to_string());
        match result {
            Ok(result) => {
                panic!("deberia dar error")
            }
            Err(e) => {
                assert_eq!(e, ErrorEliminarAtencion::VectorVacio)
            }
        }


    }
    #[test]
    fn test_rescribir_arch(){
        let veterinaria = veterinaria_de_valen(3);
        let result = veterinaria.reescribir_archivo_json(DEFAULT_FILE_PATH.parse().unwrap());
        assert!(result);
        let result = veterinaria.reescribir_archivo_json("Z:/NADA".to_string());
        assert!(!result);
    }
    #[test]
    fn test_modificar_atencion(){

        let nueva_mascota = Mascota {
            nombre: "Marrillo".to_string(),
            edad: 4,
            animal: Animal::Gato,
            dueno: Dueno {
                nombre: "Fermin".to_string(),
                direccion: "Diagonal 73".to_string(),
                telefono: 233745784,
            }
        };
        let mut nueva_atencion = Atencion {
            mascota: nueva_mascota,
            diagnostico: "Fiebre".to_string(),
            tratamiento: "Ibuprofeno".to_string(),
            proxima_visita: Fecha {
                dia: 15,
                mes: 4,
                anio: 2025,
            }
        };

        nueva_atencion.modificar_diagnostico_atencion("pulgas".to_string());

        assert_eq!(nueva_atencion.diagnostico, "pulgas".to_string());

        nueva_atencion.modificar_fecha_atencion(Fecha{ dia: 4,mes: 3, anio:2025});
        assert_eq!(nueva_atencion.proxima_visita.dia, 4);


    }

}

