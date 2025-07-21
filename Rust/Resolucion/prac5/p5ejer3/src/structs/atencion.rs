use serde::{Deserialize, Serialize};
pub(crate) use crate::structs::fecha::Fecha;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum Animal {
    Perro, Gato, Caballo,
    #[default] Otros
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u16,
    pub animal: Animal,
    pub dueno: Dueno,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico: String,
    pub tratamiento: String,
    pub proxima_visita: Fecha,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Dueno {
    pub nombre: String,
    pub direccion: String,
    pub telefono: u64,
}
impl Atencion {
    pub fn modificar_diagnostico_atencion(&mut self, nuevo_diagnostico: String) {
        self.diagnostico = nuevo_diagnostico;
    }
    
    pub fn modificar_fecha_atencion(&mut self, nueva_fecha: Fecha) {
        self.proxima_visita = nueva_fecha;
    }
}


