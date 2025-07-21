/*
use crate::structs::fecha::Fecha;
use crate::structs::cotizaciones::{Cotizacion, ErrorTransaccionBlockChain};
use crate::structs::usuario::Usuario;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum MedioRetiro {
    Transferencia,
    MercadoPago,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TipoTransaccion {
    DepositoFiat,
    RetiroFiat { medio: MedioRetiro },
    DepositoBlockchain,
    RetiroBlockchain,
    CompraCripto,
    VentaCripto,
}

pub struct TransaccionFiat {
    pub fecha: Fecha,
    pub monto: f64,
    pub usuario: u32,
    pub tipo_transaccion: TipoTransaccion,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TransaccionCripto {
    pub fecha: Fecha,
    pub monto: f64,
    pub usuario: u32,
    pub cripto: &str,
    pub tipo_transaccion: TipoTransaccion,
}

pub struct Hasheo(String);

impl<'a> Hasheo {
    fn nuevo(prefijo: String) -> Hasheo {
        Hasheo(format!("{}-{}", prefijo, rand::random::<u32>()))
    }
}

#[derive(Debug)]
pub enum ErrorNuevaTransaccion {
    FechaInvalida,
    MontoInvalido { monto: f64 },
    TipoTransaccionInvalido { tipo_transaccion: TipoTransaccion },
    CadenaNoDeclarada,
    CriptoNoSoportadaPorCadena { cripto: String, cadena: String },
    RetiroFiatRequiereMedio,
}

pub struct TransaccionBlockchain {
    pub fecha: Fecha,
    pub usuario: u32,
    pub monto: f64,
    pub hasheo: Hasheo,
    pub cadena: String,
    pub cripto: String,
    pub cotizacion: Cotizacion,
}
impl TransaccionBlockchain {
    pub fn new(fecha: Fecha, usuario: &Usuario, monto: f64, tipo_transaccion: TipoTransaccion, cadena: String, hasheo: Option<Hasheo>, cripto: String, cotizacion: Cotizacion, ) -> Result<Self, ErrorNuevaTransaccion> {

        if !fecha.es_fecha_valida() {
            return Err(ErrorNuevaTransaccion::FechaInvalida);
        }

        if monto < 0.0 {
            return Err(ErrorNuevaTransaccion::MontoInvalido { monto });
        }

        if tipo_transaccion != TipoTransaccion::RetiroBlockchain
            && tipo_transaccion != TipoTransaccion::DepositoBlockchain
        {
            return Err(ErrorNuevaTransaccion::TipoTransaccionInvalido {
                tipo_transaccion,
            });
        }

        let hasheo = hasheo.unwrap_or_else(|| Hasheo::nuevo(cadena.clone()));

        Ok(TransaccionBlockchain { fecha, usuario: usuario.dni, monto, cadena, hasheo, cripto, cotizacion })
    }
}

impl TransaccionCripto {
    pub fn new(fecha: Fecha, monto: f64, usuario: u32, prefijo:  &str, tipo_transaccion: TipoTransaccion, ) -> TransaccionCripto {

        TransaccionCripto {
            fecha,
            monto,
            usuario,
            cripto: prefijo,
            tipo_transaccion,
        }

    }
}

impl TransaccionFiat {
    pub fn nueva_transaccion_fiat(fecha: Fecha, monto: f64, usuario: u32, tipo_transaccion: TipoTransaccion, ) -> TransaccionFiat {
        TransaccionFiat {
            fecha,
            monto,
            usuario,
            tipo_transaccion,
        }
    }
}


#[cfg(test)]
mod tests_transacciones {
    use std::collections::HashMap;
    use crate::structs::cotizaciones::Cotizacion;
    use crate::structs::fecha::Fecha;
    use crate::structs::transacciones::{TipoTransaccion, TransaccionBlockchain};
    use crate::structs::usuario::{Balance, Usuario};

    #[test]
    fn test_nueva_transaccion_blockchain() {

        let usuario = Usuario {
            nombre: "Valentino".parse().unwrap(),
            apellido: "Franco".parse().unwrap(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(0.0),
            balance_cripto: HashMap::new(),
        };
        let fecha = Fecha::new(23, 4, 2025);

        let cotizacion = Cotizacion::new(1.0, 2.0).expect("Cotización válida");

        let tb = TransaccionBlockchain::new(fecha.clone(), &usuario, 100.0, TipoTransaccion::RetiroBlockchain, "03849329023".to_string(), None, "BTC".to_string(), cotizacion, );

        match tb {
            Ok(t) => {
                assert_eq!(t.usuario, 12345678);
                assert_eq!(t.monto, 100.0);
                assert_eq!(t.cripto, "BTC");
                assert_eq!(t.cadena, "03849329023");
                assert_eq!(t.fecha, fecha);
            }
            Err(e) => {
                panic!("No debería dar error: {:?}", e);
            }
        }


    }

}


 */
