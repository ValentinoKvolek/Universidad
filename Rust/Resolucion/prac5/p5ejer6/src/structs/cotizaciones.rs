/*
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cotizacion {
    pub compra: f64,
    pub venta: f64,
}

#[derive(Debug)]
pub enum ErrorTransaccionBlockChain{
    PrecioInvalido,

}

impl Cotizacion {
    pub(crate) fn new(precio_venta: f64, precio_compra: f64) -> Result <Cotizacion, ErrorTransaccionBlockChain> {
        if precio_compra <= 0.0 || precio_venta <= 0.0 || precio_venta.is_nan() || precio_compra.is_nan() || !precio_venta.is_finite() || !precio_compra.is_finite() {
            return Err(ErrorTransaccionBlockChain::PrecioInvalido);
        }
        Ok(Cotizacion {
            compra: precio_compra,
            venta: precio_venta,
        })
    }
}

#[cfg(test)]
mod test_cotizacion {
    use super::*;

    #[test]
    fn test_cotizacion() {

        let c = Cotizacion::new(1.0, 1.0);
        assert!(c.is_ok());

        let c2 = Cotizacion::new(-1.0, 1.0);
        assert!(c2.is_err());

    }

}


 */