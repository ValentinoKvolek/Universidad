use serde::{Deserialize, Serialize};
use crate::suscripciones::fecha::Fecha;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Suscripciones{
    Basic, Classic, Super, Cancelada
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum MediosPago {
    Efectivo,
    MercadoPago(u128),
    Tarjeta(u128),
    Transferencia(u128),
    Cripto(Vec<u8>),
}


#[derive( Serialize, Deserialize)]
pub struct Suscripcion {
    //Cada suscripción tiene un costo mensual y una duración de meses y una fecha de inicio,
    pub costo_mensual: f64,
    pub duracion: Fecha,
    pub fecha_inicio: Fecha,
    pub medio_pago: MediosPago,
}

impl Suscripcion {

   pub fn new (costo_mensual: f64, duracion: Fecha,fecha_inicio: Fecha,medio_pago: MediosPago,) -> Suscripcion {
        Suscripcion{
            costo_mensual,
            duracion,
            fecha_inicio,
            medio_pago
        }
    }
}
