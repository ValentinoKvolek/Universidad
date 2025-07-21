/*

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::structs::block_chain::BlockChain;
use crate::structs::fecha::Fecha;
use crate::structs::cotizaciones::Cotizacion;
use crate::structs::transacciones::{
    TransaccionBlockchain, TransaccionCripto, TransaccionFiat, TipoTransaccion,
};
use crate::structs::usuario::{ComoBalance, Balance, Usuario};


const BASE_FOLDER: &str ="D:/Universidad/Rust/Resolucion/prac5/p5ejer6/";


#[derive(Debug, Serialize, Deserialize)]
pub struct Plataforma {
    path : String,
    usuarios: HashMap<u32, Usuario>,
    cotizaciones: HashMap<String, Cotizacion>,
    block_chains: HashMap<String, BlockChain>,
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorTransaccionGeneral {
    UsuarioNoEncontrado { usuario: u32 },
    MontoInvalido { monto: f64 },
    MontoInsuficiente { monto: f64, necesario: f64 },
}

pub enum ErrorDepositoFiat {
    ErrorTransaccion(ErrorTransaccionGeneral),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorCompraCripto {
    ErrorGeneral(ErrorTransaccionGeneral),
    CriptoNoCotizada { prefijo_cripto: String },
}

#[derive(Debug)]
pub enum ErrorTransaccionBlockChain {
    ErrorGeneral(ErrorTransaccionGeneral),
    BlockchainNoEncontrada { prefix: String },
    BalanceInsuficiente { balance: Balance, necesario: Balance },
    CriptoNoCotizada { prefijo_cripto: String },
}

impl From<ErrorTransaccionGeneral> for ErrorCompraCripto {
    fn from(value: ErrorTransaccionGeneral) -> Self {
        ErrorCompraCripto::ErrorGeneral(value)
    }
}

impl From<ErrorTransaccionGeneral> for ErrorTransaccionBlockChain {
    fn from(value: ErrorTransaccionGeneral) -> Self {
        ErrorTransaccionBlockChain::ErrorGeneral(value)
    }
}


impl Plataforma {

    fn reescribir_archivo_json(&self) -> bool {

        let mut file = match File::create(format!("{}biblioteca.json", self.path)) {
            Ok(res) => res,
            Err(_) => return false,
        };

        let json_data = match serde_json::to_string_pretty(&self) {
            Ok(res) => res,
            Err(_) => return false,
        };

        file.write_all(json_data.as_bytes()).is_ok()
    }
    pub fn new() -> Self {
        Plataforma {
            path:BASE_FOLDER.to_string(),
            usuarios: HashMap::new(),
            cotizaciones: HashMap::new(),
            block_chains: HashMap::new(),
        }
    }
    pub fn deposito_fiat(&mut self, dni_usuario: u32, monto: f64, fecha: Fecha, ) -> Result<TransaccionFiat, ErrorTransaccionGeneral> {

        if monto <= 0.0 || monto.is_nan() || monto.is_infinite() {

            return Err(ErrorTransaccionGeneral::MontoInvalido { monto });

        }

        match self.usuarios.get_mut(&dni_usuario) {
            Some(user) => {
                user.balance += monto.como_balance();
                Ok(TransaccionFiat::nueva_transaccion_fiat(
                    fecha,
                    monto,
                    user.dni,
                    TipoTransaccion::DepositoFiat,
                ))
            }
            None => Err(ErrorTransaccionGeneral::UsuarioNoEncontrado { usuario: dni_usuario }),
        }
    }
    pub fn comprar_cripto(&mut self, monto_fiat: f64, dni_usuario: u32, prefijo_cripto: &str, fecha: Fecha, ) -> Result<TransaccionCripto, ErrorCompraCripto> {

        if monto_fiat <= 0.0 || !monto_fiat.is_finite() || monto_fiat.is_nan() {

            return Err(ErrorTransaccionGeneral::MontoInvalido { monto: monto_fiat }.into());

        }

        match self.usuarios.get_mut(&dni_usuario) {

            Some(u) => {

                if u.balance < monto_fiat.como_balance() {
                    return Err(ErrorTransaccionGeneral::MontoInsuficiente { monto: u.balance.f64(), necesario: monto_fiat }.into());
                }
                let precio_compra = if let Some(cot) = self.cotizaciones.get(prefijo_cripto) {
                    cot.compra
                } else {
                    return Err(ErrorCompraCripto::CriptoNoCotizada { prefijo_cripto: prefijo_cripto.to_string() }.into());
                };
                u.balance -= monto_fiat.como_balance();
                let cantidad_cripto = monto_fiat / precio_compra;
                u.balance_cripto.entry(prefijo_cripto.to_string()).and_modify(|b| *b += cantidad_cripto.como_balance()).or_insert(cantidad_cripto.como_balance());

                Ok(TransaccionCripto::new(fecha, monto_fiat, u.dni, prefijo_cripto, TipoTransaccion::CompraCripto,))

            }
            None => Err(ErrorTransaccionGeneral::UsuarioNoEncontrado { usuario: dni_usuario }.into())
        }
    }
    pub fn vender_cripto(&mut self, monto_cripto: f64, prefijo_cripto: String, usuario: u32, fecha: Fecha, ) -> Result<TransaccionCripto, ErrorCompraCripto> {

        if monto_cripto <= 0.0 || !monto_cripto.is_finite() || monto_cripto.is_nan() {
            return Err(ErrorTransaccionGeneral::MontoInvalido { monto: monto_cripto }.into());
        }

        match self.usuarios.get_mut(&usuario) {

            Some(u) => match u.balance_cripto.get_mut(&prefijo_cripto.clone()) {
                Some(balance) => {
                    if balance.f64() < monto_cripto {
                        return Err(ErrorTransaccionGeneral::MontoInsuficiente { monto: balance.f64(), necesario: monto_cripto }.into());
                    }
                    let precio_venta = if let Some(cot) = self.cotizaciones.get(&prefijo_cripto.clone()) {
                        cot.venta
                    } else {
                        return Err(ErrorCompraCripto::CriptoNoCotizada { prefijo_cripto: prefijo_cripto.to_string() });
                    };

                    *balance -= monto_cripto.como_balance();
                    let fiat_recibido = monto_cripto * precio_venta;
                    u.balance += fiat_recibido.como_balance();

                    Ok(TransaccionCripto::new(fecha, monto_cripto, u.dni, &prefijo_cripto, TipoTransaccion::VentaCripto))
                }
                None => Err(ErrorTransaccionGeneral::MontoInsuficiente { monto: 0.0, necesario: monto_cripto }.into())
            }
            None => Err(ErrorTransaccionGeneral::UsuarioNoEncontrado { usuario }.into()),
        }
    }
    pub fn retirar_cripto(&mut self, monto: f64, blockchain: String, prefijo_cripto:String, usuario: Usuario, fecha: Fecha, ) -> Result<TransaccionBlockchain, ErrorTransaccionBlockChain> {

        if monto <= 0.0 || !monto.is_finite() || monto.is_nan() {
            return Err(ErrorTransaccionGeneral::MontoInvalido { monto }.into());
        }

        if !self.block_chains.contains_key(&blockchain) {
            return Err(ErrorTransaccionBlockChain::BlockchainNoEncontrada { prefix: blockchain });
        }

        let cotizacion = match self.cotizaciones.get(prefijo_cripto) {
            Some(cot) => (*cot).clone(),
            None => return Err(ErrorTransaccionBlockChain::CriptoNoCotizada { prefijo_cripto }),
        };

        match TransaccionBlockchain::new(fecha, &usuario, monto, TipoTransaccion::RetiroBlockchain, blockchain, None, prefijo_cripto, cotizacion.clone(), ) {

            Ok(transaccion) => {
                if let Some(u) = self.usuarios.get_mut(&usuario.dni) {
                    if let Some(saldo) = u.balance_cripto.get_mut(prefijo_cripto) {
                        if saldo.f64() < monto {
                            return Err(ErrorTransaccionBlockChain::BalanceInsuficiente { balance: *saldo, necesario: monto.como_balance() });
                        }
                        *saldo -= Balance::from(monto);
                    } else {
                        return Err(ErrorTransaccionBlockChain::BalanceInsuficiente { balance: 0.0.como_balance(), necesario: monto.como_balance() });
                    }
                } else {
                    return Err(ErrorTransaccionGeneral::UsuarioNoEncontrado { usuario: usuario.dni }.into());
                }
                Ok(transaccion)
            }
            Err(_) => Err(ErrorTransaccionBlockChain::BlockchainNoEncontrada { prefix: blockchain }),

        }
    }
}
#[cfg(test)]
mod tests_plataforma {
    use super::*;
    use crate::structs::fecha::Fecha;
    use crate::structs::cotizaciones::Cotizacion;
    use crate::structs::block_chain::BlockChain;
    use crate::structs::usuario::{Usuario, Balance};
    use std::collections::HashMap;
    use serde_json::to_string;

    #[test]
    fn test_deposito_fiat_exitoso() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(0.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(23, 4, 2025);
        let result = plataforma.deposito_fiat(12345678, 100.0, fecha);

        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, 100.0);
        assert_eq!(transaccion.tipo_transaccion, TipoTransaccion::DepositoFiat);
        assert_eq!(plataforma.usuarios.get(&12345678).unwrap().balance.f64(), 100.0);

    }
    #[test]
    fn test_deposito_fiat_usuario_no_encontrado() {
        let mut plataforma = Plataforma::new();
        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.deposito_fiat(12345678, 100.0, fecha);

        assert!(matches!(result, Err(ErrorTransaccionGeneral::UsuarioNoEncontrado { usuario: 12345678 })));
    }
    #[test]
    fn test_deposito_fiat_monto_invalido() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(0.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.deposito_fiat(12345678, -100.0, fecha);

        assert!(matches!(result, Err(ErrorTransaccionGeneral::MontoInvalido { monto: -100.0 })));

    }
    #[test]
    fn test_comprar_cripto_exitoso() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario);
        plataforma.cotizaciones.insert("BTC".to_string(), Cotizacion { compra: 50000.0, venta: 51000.0 });

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.comprar_cripto(500.0, 12345678, "BTC", fecha);

        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, 500.0);
        assert_eq!(transaccion.tipo_transaccion, TipoTransaccion::CompraCripto);
        assert_eq!(plataforma.usuarios.get(&12345678).unwrap().balance.f64(), 500.0);
        assert_eq!(plataforma.usuarios.get(&12345678).unwrap().balance_cripto.get("BTC").unwrap().f64(), 0.01);

    }
    #[test]
    fn test_comprar_cripto_saldo_insuficiente_usuario_no_encontrado() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario);
        plataforma.cotizaciones.insert("BTC", Cotizacion { compra: 50000.0, venta: 51000.0 });

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.comprar_cripto(500.0, 12345678, "BTC", fecha.clone());

        assert!(matches!(result, Err(ErrorCompraCripto::ErrorGeneral(ErrorTransaccionGeneral::MontoInsuficiente { monto: 100.0, necesario: 500.0 }))));

        let result = plataforma.comprar_cripto(500.0, 0, "BTC", fecha);
        assert_eq!(result, Err(ErrorCompraCripto::ErrorGeneral(ErrorTransaccionGeneral::UsuarioNoEncontrado { usuario: 0 })));

    }
    #[test]
    fn test_comprar_cripto_no_cotizada() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.comprar_cripto(500.0, 12345678, "BTC", fecha);

        assert!(matches!(result, Err(ErrorCompraCripto::CriptoNoCotizada { prefijo_cripto: String::from("BTC") })));
    }
    #[test]
    fn test_vender_cripto_exitoso() {

        let mut plataforma = Plataforma::new();

        let mut balance_cripto = HashMap::new();

        balance_cripto.insert("BTC", Balance::nuevo(0.01));

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto,
        };

        plataforma.usuarios.insert(usuario.dni, usuario);
        plataforma.cotizaciones.insert("BTC", Cotizacion { compra: 50000.0, venta: 51000.0 });

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.vender_cripto(0.01, "BTC", 12345678, fecha);

        assert!(result.is_ok());
        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, 0.01);
        assert_eq!(transaccion.tipo_transaccion, TipoTransaccion::VentaCripto);
        assert_eq!(plataforma.usuarios.get(&12345678).unwrap().balance.f64(), 610.0);
        assert_eq!(plataforma.usuarios.get(&12345678).unwrap().balance_cripto.get("BTC").unwrap().f64(), 0.0);

    }
    #[test]
    fn test_vender_cripto_saldo_insuficiente() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario);
        plataforma.cotizaciones.insert("BTC", Cotizacion { compra: 50000.0, venta: 51000.0 });

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.vender_cripto(0.01, "BTC", 12345678, fecha);
        assert!(matches!(result, Err(ErrorCompraCripto::ErrorGeneral(ErrorTransaccionGeneral::MontoInsuficiente { monto: 0.0, necesario: 0.01 }))));
    }
    #[test]
    fn test_retirar_cripto_exitoso() {

        let mut plataforma = Plataforma::new();

        let mut balance_cripto = HashMap::new();

        balance_cripto.insert("BTC".to_string(), Balance::nuevo(0.01));

        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto,
        };

        plataforma.usuarios.insert(usuario.dni, usuario.clone());
        plataforma.cotizaciones.insert("BTC".to_string(), Cotizacion { compra: 50000.0, venta: 51000.0 });
        plataforma.block_chains.insert("ETH".to_string(), BlockChain::new("Ethereum".to_string(), "ETH".to_string()));

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.retirar_cripto(0.01, "ETH".to_string(), "BTC".to_string(), usuario, fecha);

        assert!(result.is_ok());
        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, 0.01);
        assert_eq!(transaccion.cripto, "BTC");
        assert_eq!(plataforma.usuarios.get(&12345678).unwrap().balance_cripto.get("BTC").unwrap().f64(), 0.0);

    }
    #[test]
    fn test_retirar_cripto_blockchain_no_encontrada() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino",
            apellido: "Franco",
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario.clone());
        plataforma.cotizaciones.insert("BTC", Cotizacion { compra: 50000.0, venta: 51000.0 });

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.retirar_cripto(0.01, "ETH", "BTC", usuario, fecha);

        assert!(matches!(result, Err(ErrorTransaccionBlockChain::BlockchainNoEncontrada { prefix: "ETH" })));

    }
    #[test]
    fn test_retirar_cripto_saldo_insuficiente() {

        let mut plataforma = Plataforma::new();

        let usuario = Usuario {
            nombre: "Valentino",
            apellido: "Franco",
            dni: 12345678,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto: HashMap::new(),
        };

        plataforma.usuarios.insert(usuario.dni, usuario.clone());
        plataforma.cotizaciones.insert("BTC", Cotizacion { compra: 50000.0, venta: 51000.0 });
        plataforma.block_chains.insert("ETH", BlockChain::new("Ethereum", "ETH"));

        let fecha = Fecha::new(1, 6, 2025);
        let result = plataforma.retirar_cripto(0.01, "ETH", "BTC", usuario, fecha);

        assert!(matches!(result, Err(ErrorTransaccionBlockChain::BalanceInsuficiente { balance, necesario }) if balance.f64() == 0.0 && necesario.f64() == 0.01));

    }
}



*/
