use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use crate::suscripciones::suscripciones::Suscripcion;
pub(crate) use crate::suscripciones::suscripciones::Suscripciones;

#[derive( Serialize, Deserialize)]
pub struct Usuario {

    pub id_usuario: u8,

    pub suscripcion: Suscripciones,

    pub datos: Suscripcion,

}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorUpgrade{
    NoCuentaConSuscripcion,
    SuscripcionMasAltaPosible,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorDowngrade{
    NoCuentaConSuscripcion,
    SuscripcionMasBajaPosible,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorCancelar{
    NoCuentaConSuscripcion,
    SuscripcionYaCancelada,
}


impl Usuario {
    
    //➢ Crear un usuario con una determinada suscripción y medio de pago.
    pub fn new(nombreid:u8, suscripciones: Suscripciones, datos: Suscripcion) -> Usuario {
        Usuario {
            id_usuario:nombreid,
            suscripcion: suscripciones,
            datos,
        }
    }

    //➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.
    pub fn upgrade (& mut self) -> Result<bool, ErrorUpgrade> {
        match self.suscripcion {
            Suscripciones::Basic => {self.suscripcion = Suscripciones::Classic}
            Suscripciones::Classic => {self.suscripcion = Suscripciones::Super},
            Suscripciones::Super => {return Err(ErrorUpgrade::SuscripcionMasAltaPosible)}
            _ => { return Err(ErrorUpgrade::NoCuentaConSuscripcion); }
        };
        Ok(true)
    }

    //➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
    pub fn downgrade(&mut self) -> Result<bool, ErrorDowngrade> {
        match self.suscripcion {
            Suscripciones::Super => { self.suscripcion = Suscripciones::Classic; }
            Suscripciones::Classic => { self.suscripcion = Suscripciones::Basic; }
            Suscripciones::Basic => { return Err(ErrorDowngrade::SuscripcionMasBajaPosible); }
            _ => { return Err(ErrorDowngrade::NoCuentaConSuscripcion); }
        }
        Ok(true)
    }

   // ➢ Dado un usuario cancelar la suscripción.
    pub fn cancelar_suscription(& mut self) -> Result<bool, ErrorCancelar> {
       match self.suscripcion {
           Suscripciones::Super => {self.suscripcion = Suscripciones::Cancelada;}
           Suscripciones::Classic => {self.suscripcion = Suscripciones::Cancelada;}
           Suscripciones::Basic => {self.suscripcion = Suscripciones::Cancelada;}
           Suscripciones::Cancelada => {return Err(ErrorCancelar::SuscripcionYaCancelada); }
       }
       Ok(true)
   }
    
}