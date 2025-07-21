use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::structs::fecha::Fecha;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub nombre: String,
    pub prefijo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cripto {
    pub nombre: String,
    pub prefijo: String,
    pub blockchains: Vec<Blockchain>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub dni: String,
    pub validado: bool,
    pub balance_fiat: f64,
    pub balance_cripto: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TipoTransaccion {
    IngresoDinero,
    CompraCripto,
    VentaCripto,
    RetiroCripto,
    RecepcionCripto,
    RetiroFiat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MedioPago {
    MercadoPago,
    TransferenciaBancaria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaccion {
    pub fecha: Fecha,
    pub usuario_email: String,
    pub tipo: TipoTransaccion,
    pub monto_fiat: Option<f64>,
    pub monto_cripto: Option<f64>,
    pub cotizacion: Option<f64>,
    pub cripto_prefijo: Option<String>,
    pub blockchain: Option<Blockchain>,
    pub hash_blockchain: Option<String>,
    pub medio_pago: Option<MedioPago>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plataforma {
    pub usuarios: HashMap<String, Usuario>,
    pub criptos: HashMap<String, Cripto>,
    pub transacciones: Vec<Transaccion>,
    pub hash_counter: usize,
}

impl Plataforma {
    pub fn new() -> Self {
        Plataforma {
            usuarios: HashMap::new(),
            criptos: HashMap::new(),
            transacciones: Vec::new(),
            hash_counter: 0,
        }
    }

    pub fn obtener_cotizacion(&self, _cripto_prefijo: &str) -> f64 {
        50.0
    }

    pub fn registrar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.insert(usuario.email.clone(), usuario);
    }

    pub fn agregar_cripto(&mut self, cripto: Cripto) {
        self.criptos.insert(cripto.prefijo.clone(), cripto);
    }

    pub fn guardar_usuarios_json<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("Error abriendo archivo usuarios: {}", e))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.usuarios)
            .map_err(|e| format!("Error guardando usuarios en JSON: {}", e))?;
        Ok(())
    }

    pub fn cargar_usuarios_json<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let file = File::open(path).map_err(|e| format!("Error abriendo archivo usuarios: {}", e))?;
        let reader = BufReader::new(file);
        let usuarios: HashMap<String, Usuario> =
            serde_json::from_reader(reader).map_err(|e| format!("Error leyendo JSON usuarios: {}", e))?;
        self.usuarios = usuarios;
        Ok(())
    }

    pub fn guardar_transacciones_json<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("Error abriendo archivo transacciones: {}", e))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.transacciones)
            .map_err(|e| format!("Error guardando transacciones en JSON: {}", e))?;
        Ok(())
    }

    pub fn cargar_transacciones_json<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let file = File::open(path).map_err(|e| format!("Error abriendo archivo transacciones: {}", e))?;
        let reader = BufReader::new(file);
        let transacciones: Vec<Transaccion> =
            serde_json::from_reader(reader).map_err(|e| format!("Error leyendo JSON transacciones: {}", e))?;
        self.transacciones = transacciones;
        Ok(())
    }

    pub fn cripto_mas_cantidad_ventas(&self) -> Option<(String, usize)> {
        let mut ventas: HashMap<String, usize> = HashMap::new();

        for t in &self.transacciones {
            if let TipoTransaccion::VentaCripto = t.tipo {
                if let Some(prefijo) = &t.cripto_prefijo {
                    *ventas.entry(prefijo.clone()).or_insert(0) += 1;
                }
            }
        }
        ventas.into_iter().max_by_key(|(_, cant)| *cant)
    }

    pub fn cripto_mas_cantidad_compras(&self) -> Option<(String, usize)> {
        let mut compras: HashMap<String, usize> = HashMap::new();

        for t in &self.transacciones {
            if let TipoTransaccion::CompraCripto = t.tipo {
                if let Some(prefijo) = &t.cripto_prefijo {
                    *compras.entry(prefijo.clone()).or_insert(0) += 1;
                }
            }
        }
        compras.into_iter().max_by_key(|(_, cant)| *cant)
    }

    pub fn cripto_mas_volumen_ventas(&self) -> Option<(String, f64)> {
        let mut ventas: HashMap<String, f64> = HashMap::new();

        for t in &self.transacciones {
            if let TipoTransaccion::VentaCripto = t.tipo {
                if let Some(prefijo) = &t.cripto_prefijo {
                    if let Some(monto) = t.monto_cripto {
                        *ventas.entry(prefijo.clone()).or_insert(0.0) += monto;
                    }
                }
            }
        }
        ventas.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    pub fn cripto_mas_volumen_compras(&self) -> Option<(String, f64)> {
        let mut compras: HashMap<String, f64> = HashMap::new();

        for t in &self.transacciones {
            if let TipoTransaccion::CompraCripto = t.tipo {
                if let Some(prefijo) = &t.cripto_prefijo {
                    if let Some(monto) = t.monto_cripto {
                        *compras.entry(prefijo.clone()).or_insert(0.0) += monto;
                    }
                }
            }
        }
        compras.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}


pub trait OperacionesUsuario {
    fn ingresar_dinero(&mut self, email: &str, monto: f64) -> Result<(), String>;
    fn comprar_cripto(&mut self, email: &str, cripto_prefijo: &str, monto_fiat: f64) -> Result<(), String>;
    fn vender_cripto(&mut self, email: &str, cripto_prefijo: &str, monto_cripto: f64) -> Result<(), String>;
    fn retirar_cripto(&mut self, email: &str, cripto_prefijo: &str, blockchain_nombre: &str, monto_cripto: f64) -> Result<String, String>;
    fn recibir_cripto(&mut self, email: &str, cripto_prefijo: &str, blockchain_nombre: &str, monto_cripto: f64) -> Result<(), String>;
    fn retirar_fiat(&mut self, email: &str, monto: f64, medio: MedioPago) -> Result<(), String>;
}

impl OperacionesUsuario for Plataforma {
    fn ingresar_dinero(&mut self, email: &str, monto: f64) -> Result<(), String> {
        if monto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }
        let usuario = self.usuarios.get_mut(email).ok_or("Usuario no encontrado")?;
        usuario.balance_fiat += monto;

        self.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: email.to_string(),
            tipo: TipoTransaccion::IngresoDinero,
            monto_fiat: Some(monto),
            monto_cripto: None,
            cotizacion: None,
            cripto_prefijo: None,
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        Ok(())
    }

    fn comprar_cripto(&mut self, email: &str, cripto_prefijo: &str, monto_fiat: f64) -> Result<(), String> {
        if monto_fiat <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }

        let cotizacion = self.obtener_cotizacion(cripto_prefijo);

        let usuario = self.usuarios.get_mut(email).ok_or("Usuario no encontrado")?;

        if !usuario.validado {
            return Err("Usuario no validado".to_string());
        }

        let _cripto = self.criptos.get(cripto_prefijo).ok_or("Cripto no encontrada")?;

        if usuario.balance_fiat < monto_fiat {
            return Err("Saldo fiat insuficiente".to_string());
        }

        let cantidad_cripto = monto_fiat / cotizacion;

        usuario.balance_fiat -= monto_fiat;
        *usuario.balance_cripto.entry(cripto_prefijo.to_string()).or_insert(0.0) += cantidad_cripto;

        self.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: email.to_string(),
            tipo: TipoTransaccion::CompraCripto,
            monto_fiat: Some(monto_fiat),
            monto_cripto: Some(cantidad_cripto),
            cotizacion: Some(cotizacion),
            cripto_prefijo: Some(cripto_prefijo.to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        Ok(())
    }

    fn vender_cripto(&mut self, email: &str, cripto_prefijo: &str, monto_cripto: f64) -> Result<(), String> {
        if monto_cripto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }

        let cotizacion = self.obtener_cotizacion(cripto_prefijo);

        let usuario = self.usuarios.get_mut(email).ok_or("Usuario no encontrado")?;

        if !usuario.validado {
            return Err("Usuario no validado".to_string());
        }

        let _cripto = self.criptos.get(cripto_prefijo).ok_or("Cripto no encontrada")?;

        let saldo_cripto = usuario.balance_cripto.get(cripto_prefijo).copied().unwrap_or(0.0);

        if saldo_cripto < monto_cripto {
            return Err("Saldo cripto insuficiente".to_string());
        }

        let monto_fiat = monto_cripto * cotizacion;

        usuario.balance_cripto.insert(cripto_prefijo.to_string(), saldo_cripto - monto_cripto);
        usuario.balance_fiat += monto_fiat;

        self.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: email.to_string(),
            tipo: TipoTransaccion::VentaCripto,
            monto_fiat: Some(monto_fiat),
            monto_cripto: Some(monto_cripto),
            cotizacion: Some(cotizacion),
            cripto_prefijo: Some(cripto_prefijo.to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        Ok(())
    }

    fn retirar_cripto(&mut self, email: &str, cripto_prefijo: &str, blockchain_nombre: &str, monto_cripto: f64) -> Result<String, String> {
        if monto_cripto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }
        let usuario = self.usuarios.get_mut(email).ok_or("Usuario no encontrado")?;

        if !usuario.validado {
            return Err("Usuario no validado".to_string());
        }

        let cripto = self.criptos.get(cripto_prefijo).ok_or("Cripto no encontrada")?;

        let blockchain = cripto.blockchains.iter()
            .find(|b| b.nombre == blockchain_nombre)
            .ok_or("Blockchain no encontrada")?;

        let saldo_cripto = usuario.balance_cripto.get(cripto_prefijo).copied().unwrap_or(0.0);
        if saldo_cripto < monto_cripto {
            return Err("Saldo cripto insuficiente".to_string());
        }

        usuario.balance_cripto.insert(cripto_prefijo.to_string(), saldo_cripto - monto_cripto);

        self.hash_counter += 1;
        let hash = format!("{}-{}", blockchain_nombre, self.hash_counter);

        self.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: email.to_string(),
            tipo: TipoTransaccion::RetiroCripto,
            monto_fiat: None,
            monto_cripto: Some(monto_cripto),
            cotizacion: Some(self.obtener_cotizacion(cripto_prefijo)),
            cripto_prefijo: Some(cripto_prefijo.to_string()),
            blockchain: Some(blockchain.clone()),
            hash_blockchain: Some(hash.clone()),
            medio_pago: None,
        });

        Ok(hash)
    }

    fn recibir_cripto(&mut self, email: &str, cripto_prefijo: &str, blockchain_nombre: &str, monto_cripto: f64) -> Result<(), String> {
        if monto_cripto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }
        let usuario = self.usuarios.get_mut(email).ok_or("Usuario no encontrado")?;

        let cripto = self.criptos.get(cripto_prefijo).ok_or("Cripto no encontrada")?;

        let blockchain = cripto.blockchains.iter()
            .find(|b| b.nombre == blockchain_nombre)
            .ok_or("Blockchain no encontrada")?;

        *usuario.balance_cripto.entry(cripto_prefijo.to_string()).or_insert(0.0) += monto_cripto;

        self.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: email.to_string(),
            tipo: TipoTransaccion::RecepcionCripto,
            monto_fiat: None,
            monto_cripto: Some(monto_cripto),
            cotizacion: Some(self.obtener_cotizacion(cripto_prefijo)),
            cripto_prefijo: Some(cripto_prefijo.to_string()),
            blockchain: Some(blockchain.clone()),
            hash_blockchain: None,
            medio_pago: None,
        });

        Ok(())
    }

    fn retirar_fiat(&mut self, email: &str, monto: f64, medio: MedioPago) -> Result<(), String> {
        if monto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }
        let usuario = self.usuarios.get_mut(email).ok_or("Usuario no encontrado")?;

        if !usuario.validado {
            return Err("Usuario no validado".to_string());
        }

        if usuario.balance_fiat < monto {
            return Err("Saldo fiat insuficiente".to_string());
        }

        usuario.balance_fiat -= monto;

        self.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: email.to_string(),
            tipo: TipoTransaccion::RetiroFiat,
            monto_fiat: Some(monto),
            monto_cripto: None,
            cotizacion: None,
            cripto_prefijo: None,
            blockchain: None,
            hash_blockchain: None,
            medio_pago: Some(medio),
        });

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::structs::fecha::Fecha;

    fn crear_usuario_valido(email: &str) -> Usuario {
        Usuario {
            nombre: "Juan".into(),
            apellido: "Pérez".into(),
            email: email.into(),
            dni: "12345678".into(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        }
    }

    fn crear_usuario_no_valido(email: &str) -> Usuario {
        Usuario {
            nombre: "Ana".into(),
            apellido: "Gomez".into(),
            email: email.into(),
            dni: "87654321".into(),
            validado: false,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        }
    }

    fn crear_plataforma_con_cripto() -> Plataforma {
        let mut plataforma = Plataforma::new();
        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };
        let cripto = Cripto {
            nombre: "Ether".to_string(),
            prefijo: "ETH".to_string(),
            blockchains: vec![blockchain],
        };
        plataforma.agregar_cripto(cripto);
        plataforma
    }


    #[test]
    fn test_ingresar_dinero_monto_negativo() {
        let mut plataforma = Plataforma::new();
        let usuario = crear_usuario_valido("a@a.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.ingresar_dinero("a@a.com", -100.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_ingresar_dinero_usuario_no_existente() {
        let mut plataforma = Plataforma::new();
        let res = plataforma.ingresar_dinero("noexiste@a.com", 100.0);
        assert!(res.is_err());
    }

    // ---------- Tests para comprar cripto ----------

    #[test]
    fn test_comprar_cripto_usuario_no_valido() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_no_valido("b@b.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.comprar_cripto("b@b.com", "ETH", 100.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Usuario no validado");
    }

    #[test]
    fn test_comprar_cripto_saldo_insuficiente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let mut usuario = crear_usuario_valido("c@c.com");
        usuario.balance_fiat = 50.0; // poco saldo
        plataforma.registrar_usuario(usuario);
        let res = plataforma.comprar_cripto("c@c.com", "ETH", 100.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Saldo fiat insuficiente");
    }

    #[test]
    fn test_comprar_cripto_cripto_inexistente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("d@d.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.comprar_cripto("d@d.com", "BTC", 100.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Cripto no encontrada");
    }

    #[test]
    fn test_comprar_cripto_monto_negativo() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("e@e.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.comprar_cripto("e@e.com", "ETH", -10.0);
        assert!(res.is_err());
    }


    #[test]
    fn test_vender_cripto_monto_negativo() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("f@f.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.vender_cripto("f@f.com", "ETH", -5.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_vender_cripto_usuario_no_valido() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_no_valido("g@g.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.vender_cripto("g@g.com", "ETH", 1.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Usuario no validado");
    }

    #[test]
    fn test_vender_cripto_saldo_insuficiente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("h@h.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.vender_cripto("h@h.com", "ETH", 5.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Saldo cripto insuficiente");
    }

    #[test]
    fn test_vender_cripto_cripto_inexistente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let mut usuario = crear_usuario_valido("i@i.com");
        usuario.balance_cripto.insert("ETH".to_string(), 10.0);
        plataforma.registrar_usuario(usuario);
        let res = plataforma.vender_cripto("i@i.com", "BTC", 5.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Cripto no encontrada");
    }

    #[test]
    fn test_vender_cripto_correcto() {
        let mut plataforma = crear_plataforma_con_cripto();
        let mut usuario = crear_usuario_valido("j@j.com");
        usuario.balance_cripto.insert("ETH".to_string(), 10.0);
        plataforma.registrar_usuario(usuario);
        let res = plataforma.vender_cripto("j@j.com", "ETH", 5.0);
        assert!(res.is_ok());
        let usuario_post = plataforma.usuarios.get("j@j.com").unwrap();
        assert_eq!(usuario_post.balance_cripto["ETH"], 5.0);
        assert!(usuario_post.balance_fiat > 1000.0);
    }


    #[test]
    fn test_retirar_cripto_monto_negativo() {
        let mut plataforma = crear_plataforma_con_cripto();
        let res = plataforma.retirar_cripto("k@k.com", "ETH", "Ethereum", -3.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_retirar_cripto_usuario_no_valido() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_no_valido("l@l.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.retirar_cripto("l@l.com", "ETH", "Ethereum", 1.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Usuario no validado");
    }

    #[test]
    fn test_retirar_cripto_saldo_insuficiente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("m@m.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.retirar_cripto("m@m.com", "ETH", "Ethereum", 1.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Saldo cripto insuficiente");
    }

    #[test]
    fn test_retirar_cripto_blockchain_inexistente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let mut usuario = crear_usuario_valido("n@n.com");
        usuario.balance_cripto.insert("ETH".to_string(), 10.0);
        plataforma.registrar_usuario(usuario);
        let res = plataforma.retirar_cripto("n@n.com", "ETH", "NoExist", 1.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Blockchain no encontrada");
    }

    #[test]
    fn test_retirar_cripto_correcto() {
        let mut plataforma = crear_plataforma_con_cripto();
        let mut usuario = crear_usuario_valido("o@o.com");
        usuario.balance_cripto.insert("ETH".to_string(), 5.0);
        plataforma.registrar_usuario(usuario);
        let res = plataforma.retirar_cripto("o@o.com", "ETH", "Ethereum", 3.0);
        assert!(res.is_ok());
        let hash = res.unwrap();
        assert!(hash.contains("Ethereum-"));
        let usuario_post = plataforma.usuarios.get("o@o.com").unwrap();
        assert_eq!(usuario_post.balance_cripto["ETH"], 2.0);
    }


    #[test]
    fn test_recibir_cripto_monto_negativo() {
        let mut plataforma = crear_plataforma_con_cripto();
        let res = plataforma.recibir_cripto("p@p.com", "ETH", "Ethereum", -1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_recibir_cripto_cripto_inexistente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("q@q.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.recibir_cripto("q@q.com", "BTC", "Ethereum", 1.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Cripto no encontrada");
    }

    #[test]
    fn test_recibir_cripto_blockchain_inexistente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("r@r.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.recibir_cripto("r@r.com", "ETH", "NoExist", 1.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Blockchain no encontrada");
    }

    #[test]
    fn test_recibir_cripto_correcto() {
        let mut plataforma = crear_plataforma_con_cripto();
        let mut usuario = crear_usuario_valido("s@s.com");
        usuario.balance_cripto.insert("ETH".to_string(), 2.0);
        plataforma.registrar_usuario(usuario);
        let res = plataforma.recibir_cripto("s@s.com", "ETH", "Ethereum", 3.0);
        assert!(res.is_ok());
        let usuario_post = plataforma.usuarios.get("s@s.com").unwrap();
        assert_eq!(usuario_post.balance_cripto["ETH"], 5.0);
    }

    #[test]
    fn test_retirar_fiat_monto_negativo() {
        let mut plataforma = crear_plataforma_con_cripto();
        let res = plataforma.retirar_fiat("t@t.com", -100.0, MedioPago::MercadoPago);
        assert!(res.is_err());
    }

    #[test]
    fn test_retirar_fiat_usuario_no_valido() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_no_valido("u@u.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.retirar_fiat("u@u.com", 100.0, MedioPago::MercadoPago);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Usuario no validado");
    }

    #[test]
    fn test_retirar_fiat_saldo_insuficiente() {
        let mut plataforma = crear_plataforma_con_cripto();
        let mut usuario = crear_usuario_valido("v@v.com");
        usuario.balance_fiat = 50.0;
        plataforma.registrar_usuario(usuario);
        let res = plataforma.retirar_fiat("v@v.com", 100.0, MedioPago::MercadoPago);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Saldo fiat insuficiente");
    }

    #[test]
    fn test_retirar_fiat_correcto() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("w@w.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.retirar_fiat("w@w.com", 500.0, MedioPago::MercadoPago);
        assert!(res.is_ok());
        let usuario_post = plataforma.usuarios.get("w@w.com").unwrap();
        assert_eq!(usuario_post.balance_fiat, 500.0);
    }

    #[test]
    fn test_cripto_mas_cantidad_ventas_y_compras() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("x@x.com");
        plataforma.registrar_usuario(usuario);

        // Simular ventas y compras en transacciones
        plataforma.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: "x@x.com".to_string(),
            tipo: TipoTransaccion::VentaCripto,
            monto_fiat: Some(100.0),
            monto_cripto: Some(2.0),
            cotizacion: Some(50.0),
            cripto_prefijo: Some("ETH".to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        plataforma.transacciones.push(Transaccion {
            fecha:Fecha::new(1,1,2025),
            usuario_email: "x@x.com".to_string(),
            tipo: TipoTransaccion::VentaCripto,
            monto_fiat: Some(50.0),
            monto_cripto: Some(1.0),
            cotizacion: Some(50.0),
            cripto_prefijo: Some("ETH".to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        plataforma.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: "x@x.com".to_string(),
            tipo: TipoTransaccion::CompraCripto,
            monto_fiat: Some(150.0),
            monto_cripto: Some(3.0),
            cotizacion: Some(50.0),
            cripto_prefijo: Some("ETH".to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        assert_eq!(plataforma.cripto_mas_cantidad_ventas(), Some(("ETH".to_string(), 2)));
        assert_eq!(plataforma.cripto_mas_cantidad_compras(), Some(("ETH".to_string(), 1)));
    }

    #[test]
    fn test_cripto_mas_volumen_ventas_y_compras() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("y@y.com");
        plataforma.registrar_usuario(usuario);

        plataforma.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: "y@y.com".to_string(),
            tipo: TipoTransaccion::VentaCripto,
            monto_fiat: Some(100.0),
            monto_cripto: Some(2.0),
            cotizacion: Some(50.0),
            cripto_prefijo: Some("ETH".to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        plataforma.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: "y@y.com".to_string(),
            tipo: TipoTransaccion::VentaCripto,
            monto_fiat: Some(50.0),
            monto_cripto: Some(1.5),
            cotizacion: Some(50.0),
            cripto_prefijo: Some("ETH".to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        plataforma.transacciones.push(Transaccion {
            fecha: Fecha::new(1,1,2025),
            usuario_email: "y@y.com".to_string(),
            tipo: TipoTransaccion::CompraCripto,
            monto_fiat: Some(150.0),
            monto_cripto: Some(3.5),
            cotizacion: Some(50.0),
            cripto_prefijo: Some("ETH".to_string()),
            blockchain: None,
            hash_blockchain: None,
            medio_pago: None,
        });

        assert_eq!(plataforma.cripto_mas_volumen_ventas(), Some(("ETH".to_string(), 3.5)));
        assert_eq!(plataforma.cripto_mas_volumen_compras(), Some(("ETH".to_string(), 3.5)));
    }

    #[test]
    fn test_obtener_cotizacion_devuelve_valor_fijo() {
        let plataforma = Plataforma::new();
        let cotizacion = plataforma.obtener_cotizacion("ETH");
        assert_eq!(cotizacion, 50.0);
    }

    #[test]
    fn test_registrar_usuario_y_agregar_cripto() {
        let mut plataforma = Plataforma::new();
        let usuario = crear_usuario_valido("z@z.com");
        plataforma.registrar_usuario(usuario.clone());
        assert_eq!(plataforma.usuarios.get("z@z.com").unwrap().dni, usuario.dni);

        let blockchain = Blockchain {
            nombre: "Bitcoin".into(),
            prefijo: "BTC".into(),
        };
        let cripto = Cripto {
            nombre: "Bitcoin".into(),
            prefijo: "BTC".into(),
            blockchains: vec![blockchain.clone()],
        };
        plataforma.agregar_cripto(cripto.clone());
        assert_eq!(plataforma.criptos.get("BTC").unwrap().nombre, "Bitcoin");
    }

    #[test]
    fn test_guardar_y_cargar_transacciones_json() {
        let path = "test_transacciones.json";
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("trans@ok.com");
        plataforma.registrar_usuario(usuario.clone());
        plataforma.comprar_cripto("trans@ok.com", "ETH", 100.0).unwrap();
        plataforma.guardar_transacciones_json(path).unwrap();

        let mut nueva_plataforma = crear_plataforma_con_cripto();
        nueva_plataforma.registrar_usuario(usuario);
        nueva_plataforma.cargar_transacciones_json(path).unwrap();

        assert_eq!(nueva_plataforma.transacciones.len(), 1);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_guardar_usuarios_json_error_path_invalido() {
        let plataforma = Plataforma::new();
        // Ruta inválida para generar error de apertura
        let res = plataforma.guardar_usuarios_json("/ruta/invalida/usuarios.json");
        assert!(res.is_err());
    }

    #[test]
    fn test_cargar_usuarios_json_error_json_malformado() {
        let path = "test_usuarios_mal.json";
        // Creo archivo con contenido inválido JSON
        std::fs::write(path, "contenido_no_json").unwrap();

        let mut plataforma = Plataforma::new();
        let res = plataforma.cargar_usuarios_json(path);
        assert!(res.is_err());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_comprar_cripto_monto_cero_error() {
        let mut plataforma = crear_plataforma_con_cripto();
        let usuario = crear_usuario_valido("zero@cero.com");
        plataforma.registrar_usuario(usuario);
        let res = plataforma.comprar_cripto("zero@cero.com", "ETH", 0.0);
        assert!(res.is_err());
    }

}