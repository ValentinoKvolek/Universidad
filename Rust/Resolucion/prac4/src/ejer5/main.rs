/*
5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat. De los usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma. De las criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir. De cada blockchain se conoce el nombre, prefijo.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las siguientes acciones relacionadas al usuario:
➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho donde los datos que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.
➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad de determinada criptomoneda, tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance de la cripto y desacreditar en el balance de fiat. Luego de ello se registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo: venta de cripto,monto de cripto y cotización.
➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve un hash que representa una transacción en ella (esto hágalo retornando el nombre de la blockchain + un número random). Luego se genera una transacción con los siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto, cotización.
➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una
transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto, blockchain, cripto, monto, cotización.
➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho monto del balance al usuario y se genera una transacción con la siguiente información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago o Transferencia Bancaria)
Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado. Se debe validar siempre que haya balance suficiente para realizar la operación en los casos de compra, venta, retiro.
Además la empresa desea saber lo siguiente en base a sus operaciones:
➢ Saber cual es la criptomoneda que más cantidad de ventas tiene ➢ Saber cual es la criptomoneda que más cantidad de compras tiene ➢ Saber cual es la criptomoneda que más volumen de ventas tiene ➢ Saber cual es la criptomoneda que más volumen de compras tiene
*/


//
// Blockchain.rs
//

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, SubAssign};

pub struct BlockChain {
    pub nombre: String,
    pub prefijo: String,
}

impl BlockChain<> {
    fn new(nombre: String, prefijo: String) -> BlockChain {
        BlockChain { nombre, prefijo }
    }
}

//
// Blockchain.rs
//


//
// Cotizacion.rs
//

#[derive(Debug, Clone, PartialEq)]
pub struct Cotizacion {
    pub compra: f64,
    pub venta: f64,
}

impl Cotizacion {
    fn new(precio_venta: f64, precio_compra: f64) -> Option<Cotizacion> {

        if precio_venta.is_nan() || precio_compra.is_nan() || precio_venta.is_infinite() || precio_compra.is_infinite() || precio_venta < 0.0 || precio_compra < 0.0{
            return None;
        }
        Some(Cotizacion {
            compra: precio_compra,
            venta: precio_venta,
        })
    }
}


//
// Cotizacion.rs
//

//
//Crypto.rs
//

pub struct Cripto {
    pub nombre: String,
    pub prefijo: String,
    pub cadenas: Vec<String>,
    pub balance: Balance,
}
//
// Crypto.rs
//


//
// fecha.rs
//
const NOMBRE_MESES: [&str; 12]=["Enero", "Febrero", "Marzo",
    "Abril", "Mayo", "Junio","Julio","Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fecha{
    pub dia:u8, pub mes:u8, pub anio:u16
}
impl Fecha {
    pub fn new(dia: u8, mes: u8, anio: u16) -> Fecha {

        Fecha {
            dia,
            mes,
            anio
        }

    }

    pub fn es_fecha_valida(&self) -> bool {

        //check mes
        if!(1..=12).contains(&self.mes) {return false;}

        if self.dia < 1 || self.dia > self.dias_mes_actual(){return false};

        //como el año es un u16 asumo que ya tiene que ser valido.

        true
    }

    pub fn sumar_dias(&mut self, mut dias: u8) {

        while dias > 0 {
            let dias_restantes_mes = self.dias_mes_actual() - self.dia;

            if dias > dias_restantes_mes {
                dias -= dias_restantes_mes + 1;
                self.dia = 1;
                self.mes += 1;

                if self.mes > 12 {
                    self.mes = 1;
                    self.anio += 1;
                }

            } else {
                self.dia += dias;
                dias = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u8) {
        while dias > 0 {

            if dias >= self.dia {
                dias -= self.dia;

                if self.mes == 1 {
                    self.mes = 12;
                    self.anio -= 1;
                } else {
                    self.mes -= 1;
                }
                self.dia = self.dias_mes_actual();

            } else {
                self.dia -= dias;
                dias = 0;
            }
        }
    }


    pub fn es_menor(&self, nueva_fecha: Fecha) -> bool {
        if self.anio < nueva_fecha.anio ||
            (self.anio == nueva_fecha.anio && self.mes < nueva_fecha.mes) ||
            (self.anio == nueva_fecha.anio && self.mes == nueva_fecha.mes && self.dia < nueva_fecha.dia) { return true }

        false
    }

    pub fn dias_mes_actual(&self) -> u8 {
        match self.mes  {
            4| 6 | 9| 11 => 30, // si es alguno de estos mes
            // es tiene 30 dias
            2=> if self.es_bisiesto() {29} else {28},
            _ => 31,
        }
    }

    pub fn es_bisiesto(&self)-> bool{
        self.anio % 4 == 0}
}
//
// fecha.rs
//

//
// Transacciones.rs
//

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

// Transacción en dinero fiat
pub struct TransaccionFiat {
    pub fecha: Fecha,
    pub monto: f64,
    pub usuario: u32,
    pub tipo_transaccion: TipoTransaccion,
}

// Transacción con criptomonedas
#[derive(Debug, Clone, PartialEq )]
pub struct TransaccionCripto {
    pub fecha: Fecha,
    pub monto: f64,
    pub usuario: u32,
    pub cripto: String,
    pub tipo_transaccion: TipoTransaccion,
}

// Hash simulado de blockchain
pub struct Hasheo(String);

impl Hasheo {
    fn nuevo(prefijo: &str) -> Hasheo {
        Hasheo(format!("{}-{}", prefijo, rand::random::<u32>()))
    }
}

// Errores al crear una nueva transacción
pub enum ErrorNuevaTransaccion{
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
    pub cotizacion:Cotizacion,
}
impl TransaccionBlockchain {
    pub fn nueva(
        fecha: Fecha,
        usuario: &Usuario,
        monto: f64,
        tipo_transaccion: TipoTransaccion,
        cadena: String,
        hasheo: Option<Hasheo>,
        cripto: String,
        cotizacion: Cotizacion,
    ) -> Result<Self, ErrorNuevaTransaccion> {
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

        let hasheo = hasheo.unwrap_or_else(|| Hasheo::nuevo(cadena.as_str()));

        Ok(TransaccionBlockchain {
            fecha,
            usuario: usuario.dni,
            monto,
            cadena,
            hasheo,
            cripto,
            cotizacion,
        })
    }
}

impl TransaccionCripto {
    pub fn nueva(
        fecha: Fecha,
        monto: f64,
        usuario: u32,
        prefijo:String,
        tipo_transaccion: TipoTransaccion,
    ) -> TransaccionCripto {
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

//
// Transacciones.rs
//

//
// Usuario.rs
//
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Balance(pub f64);

impl Balance {

    pub fn nuevo(balance: f64) -> Self {
        Balance(balance)
    }


    pub fn sumar_f64(&mut self, val: f64) {
        self.0 += val;
    }


    pub fn restar_f64(&mut self, val: f64) {
        self.0 -= val;
    }


    pub fn f64(&self) -> f64 {
        self.0
    }
}


impl Hash for Balance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}


impl AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Balance {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}


impl From<f64> for Balance {
    fn from(valor: f64) -> Self {
        Balance(valor)
    }
}

pub trait ComoBalance {
    fn como_balance(&self) -> Balance;
}

// Implementación para `f64`
impl ComoBalance for f64 {
    fn como_balance(&self) -> Balance {
        Balance(*self)
    }
}

pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub dni: u32,
    pub verificado: bool,
    pub balance: Balance,
    pub balance_cripto: HashMap<String, Balance>,
}

//
// Usuario.rs
//

//
// Plataforma.rs
//

pub struct XYZ {
    pub usuarios: HashMap<u32, Usuario>,
    pub cotizaciones: HashMap<String, Cotizacion>,
    pub blockchains: HashMap<String, BlockChain>,
    pub transacciones_cripto: Vec<TransaccionCripto>,
}


impl XYZ {
    pub fn new() -> XYZ {
        XYZ {
            usuarios: HashMap::new(),
            cotizaciones: HashMap::new(),
            blockchains: HashMap::new(),
            transacciones_cripto: Vec::new(),
        }
    }

    pub fn ingresar_dinero(&mut self, dni: u32, monto: f64, fecha: Fecha, ) -> Result<TransaccionFiat, String> {

        let usuario = self.usuarios.get_mut(&dni)
            .ok_or("Usuario no encontrado")?;


        if monto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }


        usuario.balance.sumar_f64(monto);


        let transaccion = TransaccionFiat::nueva_transaccion_fiat(
            fecha,
            monto,
            dni,
            TipoTransaccion::DepositoFiat,
        );

        Ok(transaccion)
    }

    pub fn comprar_cripto(&mut self, dni: u32, monto_fiat: f64, cripto_prefijo: &str, fecha: Fecha, ) -> Result<TransaccionCripto, String> {

        let usuario = self.usuarios.get_mut(&dni)
            .ok_or("Usuario no encontrado")?;
        if !usuario.verificado {
            return Err("Usuario no verificado".to_string());
        }


        let cotizacion = self.cotizaciones.get(cripto_prefijo)
            .ok_or("Cotización no encontrada para la cripto")?;
        if monto_fiat <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }


        if usuario.balance.f64() < monto_fiat {
            return Err("Balance fiat insuficiente".to_string());
        }


        let monto_cripto = monto_fiat / cotizacion.compra;


        usuario.balance.restar_f64(monto_fiat);
        let entry = usuario.balance_cripto.entry(cripto_prefijo.to_string()).or_insert(Balance::nuevo(0.0));
        entry.sumar_f64(monto_cripto);


        let transaccion = TransaccionCripto {
            fecha,
            monto: monto_cripto,
            usuario: dni,
            cripto: cripto_prefijo.to_string(),
            tipo_transaccion: TipoTransaccion::CompraCripto,
        };

        self.transacciones_cripto.push(transaccion.clone());

        Ok(transaccion)
    }

    pub fn vender_cripto(&mut self, dni: u32, monto_cripto: f64, cripto_prefijo: &str, fecha: Fecha, ) -> Result<TransaccionCripto, String> {

        let usuario = self.usuarios.get_mut(&dni)
            .ok_or("Usuario no encontrado")?;
        if !usuario.verificado {
            return Err("Usuario no verificado".to_string());
        }


        let cotizacion = self.cotizaciones.get(cripto_prefijo)
            .ok_or("Cotización no encontrada para la cripto")?;
        if monto_cripto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }


        let balance_cripto = usuario.balance_cripto.get_mut(cripto_prefijo)
            .ok_or("Balance de cripto insuficiente")?;
        if balance_cripto.f64() < monto_cripto {
            return Err("Balance de cripto insuficiente".to_string());
        }


        let monto_fiat = monto_cripto * cotizacion.venta;


        balance_cripto.restar_f64(monto_cripto);
        usuario.balance.sumar_f64(monto_fiat);


        let transaccion = TransaccionCripto {
            fecha,
            monto: monto_cripto,
            usuario: dni,
            cripto: cripto_prefijo.to_string(),
            tipo_transaccion: TipoTransaccion::VentaCripto,
        };

        self.transacciones_cripto.push(transaccion.clone());
        Ok(transaccion)
    }

    pub fn retirar_cripto_a_blockchain(&mut self, dni: u32, monto_cripto: f64, cripto_prefijo: &str, blockchain_prefijo: &str, fecha: Fecha, ) -> Result<TransaccionBlockchain, String> {

        let usuario = self.usuarios.get_mut(&dni)
            .ok_or("Usuario no encontrado")?;
        if !usuario.verificado {
            return Err("Usuario no verificado".to_string());
        }


        let blockchain = self.blockchains.get(blockchain_prefijo)
            .ok_or("Blockchain no encontrada")?;


        let cotizacion = self.cotizaciones.get(cripto_prefijo)
            .ok_or("Cotización no encontrada para la cripto")?;
        if monto_cripto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }


        let balance_cripto = usuario.balance_cripto.get_mut(cripto_prefijo)
            .ok_or("Balance de cripto insuficiente")?;
        if balance_cripto.f64() < monto_cripto {
            return Err("Balance de cripto insuficiente".to_string());
        }


        balance_cripto.restar_f64(monto_cripto);


        let hasheo = Hasheo::nuevo(&blockchain.prefijo);


        let transaccion = TransaccionBlockchain {
            fecha,
            usuario: dni,
            monto: monto_cripto,
            hasheo,
            cadena: blockchain_prefijo.to_string(),
            cripto: cripto_prefijo.to_string(),
            cotizacion: cotizacion.clone(),
        };

        Ok(transaccion)
    }

    pub fn recibir_cripto_de_blockchain(&mut self, dni: u32, monto_cripto: f64, cripto_prefijo: &str, blockchain_prefijo: &str, fecha: Fecha, ) -> Result<TransaccionBlockchain, String> {

        let usuario = self.usuarios.get_mut(&dni)
            .ok_or("Usuario no encontrado")?;
        if !usuario.verificado {
            return Err("Usuario no verificado".to_string());
        }


        let blockchain = self.blockchains.get(blockchain_prefijo)
            .ok_or("Blockchain no encontrada")?;


        let cotizacion = self.cotizaciones.get(cripto_prefijo)
            .ok_or("Cotización no encontrada para la cripto")?;
        if monto_cripto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }


        let entry = usuario.balance_cripto.entry(cripto_prefijo.to_string()).or_insert(Balance::nuevo(0.0));
        entry.sumar_f64(monto_cripto);


        let transaccion = TransaccionBlockchain {
            fecha,
            usuario: dni,
            monto: monto_cripto,
            hasheo: Hasheo::nuevo(&blockchain.prefijo),
            cadena: blockchain_prefijo.to_string(),
            cripto: cripto_prefijo.to_string(),
            cotizacion: cotizacion.clone(),
        };

        Ok(transaccion)
    }

    pub fn retirar_fiat(&mut self, dni: u32, monto: f64, medio: MedioRetiro, fecha: Fecha, ) -> Result<TransaccionFiat, String> {
        let usuario = self.usuarios.get_mut(&dni)
            .ok_or("Usuario no encontrado")?;
        if !usuario.verificado {
            return Err("Usuario no verificado".to_string());
        }
        if monto <= 0.0 {
            return Err("Monto debe ser positivo".to_string());
        }
        if usuario.balance.f64() < monto {
            return Err("Balance fiat insuficiente".to_string());
        }
        usuario.balance.restar_f64(monto);

        let transaccion = TransaccionFiat {
            fecha,
            monto,
            usuario: dni,
            tipo_transaccion: TipoTransaccion::RetiroFiat { medio },
        };
        Ok(transaccion)
    }
    pub fn cripto_mas_cantidad_ventas(&self) -> Option<String> {
        let mut counts = HashMap::new();
        for t in &self.transacciones_cripto {
            if t.tipo_transaccion == TipoTransaccion::VentaCripto {
                *counts.entry(&t.cripto).or_insert(0) += 1;
            }
        }
        counts.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k.clone())
    }

    pub fn cripto_mas_cantidad_compras(&self) -> Option<String> {
        let mut counts = HashMap::new();
        for t in &self.transacciones_cripto {
            if t.tipo_transaccion == TipoTransaccion::CompraCripto {
                *counts.entry(&t.cripto).or_insert(0) += 1;
            }
        }
        counts.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k.clone())
    }

    pub fn cripto_mas_volumen_ventas(&self) -> Option<String> {
        let mut volumes = HashMap::new();
        for t in &self.transacciones_cripto {
            if t.tipo_transaccion == TipoTransaccion::VentaCripto {
                *volumes.entry(&t.cripto).or_insert(0.0) += t.monto;
            }
        }
        volumes.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(k, _)| k.clone())
    }

    pub fn cripto_mas_volumen_compras(&self) -> Option<String> {
        let mut volumes = HashMap::new();
        for t in &self.transacciones_cripto {
            if t.tipo_transaccion == TipoTransaccion::CompraCripto {
                *volumes.entry(&t.cripto).or_insert(0.0) += t.monto;
            }
        }
        volumes.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(k, _)| k.clone())
    }




}


//
// Plataforma.rs
//




#[cfg(test)]
mod tests_fecha {
    use super::*;

    #[test]
    fn test_fecha_valida() {
        let f = Fecha::new(29, 2, 2024); // bisiesto
        assert!(f.es_fecha_valida());
    }

    #[test]
    fn test_fecha_invalida_mes() {
        let f = Fecha::new(10, 13, 2024);
        assert!(!f.es_fecha_valida());
    }

    #[test]
    fn test_fecha_invalida_dia() {
        let f = Fecha::new(31, 4, 2024); // abril tiene 30 días
        assert!(!f.es_fecha_valida());
    }

    #[test]
    fn test_sumar_dias_con_salto_de_mes() {
        let mut f = Fecha::new(28, 2, 2023); // no bisiesto
        f.sumar_dias(3);
        assert_eq!(f.dia, 3);
        assert_eq!(f.mes, 3);
    }

    #[test]
    fn test_sumar_dias_con_salto_de_anio() {
        let mut f = Fecha::new(31, 12, 2023);
        f.sumar_dias(1);
        assert_eq!(f.dia, 1);
        assert_eq!(f.mes, 1);
        assert_eq!(f.anio, 2024);
    }

    #[test]
    fn test_restar_dias_simple() {
        let mut f = Fecha::new(10, 3, 2024);
        f.restar_dias(5);
        assert_eq!(f.dia, 5);
        assert_eq!(f.mes, 3);
    }

    #[test]
    fn test_restar_dias_con_salto_de_mes() {
        let mut f = Fecha::new(1, 3, 2024); // bisiesto
        f.restar_dias(1);
        assert_eq!(f.dia, 29);
        assert_eq!(f.mes, 2);
    }

    #[test]
    fn test_es_menor_true() {
        let f1 = Fecha::new(10, 5, 2024);
        let f2 = Fecha::new(5, 5, 2024);
        assert!(f2.es_menor(f1));
    }

    #[test]
    fn test_es_menor_false() {
        let f1 = Fecha::new(5, 5, 2024);
        let f2 = Fecha::new(10, 5, 2024);
        assert!(f1.es_menor(f2));
    }
}

#[cfg(test)]
mod tests_XYZ {
    use super::*;
    #[test]
    fn test_ingresar_dinero_ok() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 123,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let monto = 50.0;

        let result = plataforma.ingresar_dinero(123, monto, fecha.clone());
        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, monto);
        assert_eq!(transaccion.usuario, 123);
        assert_eq!(transaccion.fecha, fecha);
        assert_eq!(transaccion.tipo_transaccion, TipoTransaccion::DepositoFiat);

        let usuario_actualizado = plataforma.usuarios.get(&123).unwrap();
        assert_eq!(usuario_actualizado.balance.f64(), 150.0);
    }

    #[test]
    fn test_ingresar_dinero_usuario_no_encontrado() {
        let mut plataforma = XYZ::new();
        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.ingresar_dinero(999, 50.0, fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_ingresar_dinero_monto_invalido() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 456,
            verificado: true,
            balance: Balance::nuevo(200.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.ingresar_dinero(456, -10.0, fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_comprar_cripto_ok() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 123,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        let fecha = Fecha::new(1, 1, 2024);
        let monto_fiat = 500.0;

        let result = plataforma.comprar_cripto(123, monto_fiat, "BTC", fecha.clone());
        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, 10.0); // 500 / 50 = 10 BTC
        assert_eq!(transaccion.usuario, 123);
        assert_eq!(transaccion.fecha, fecha);
        assert_eq!(transaccion.cripto, "BTC".to_string());
        assert_eq!(transaccion.tipo_transaccion, TipoTransaccion::CompraCripto);

        let usuario_actualizado = plataforma.usuarios.get(&123).unwrap();
        assert_eq!(usuario_actualizado.balance.f64(), 500.0); // 1000 - 500
        assert_eq!(usuario_actualizado.balance_cripto.get("BTC").unwrap().f64(), 10.0);
    }

    #[test]
    fn test_comprar_cripto_usuario_no_encontrado() {
        let mut plataforma = XYZ::new();
        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.comprar_cripto(999, 500.0, "BTC", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_comprar_cripto_usuario_no_verificado() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 456,
            verificado: false,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.comprar_cripto(456, 500.0, "BTC", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_comprar_cripto_balance_insuficiente() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 789,
            verificado: true,
            balance: Balance::nuevo(100.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.comprar_cripto(789, 500.0, "BTC", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_comprar_cripto_cotizacion_no_encontrada() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 321,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.comprar_cripto(321, 500.0, "ETH", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_vender_cripto_ok() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 123,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        plataforma.usuarios.get_mut(&123).unwrap().balance_cripto.insert("BTC".to_string(), Balance::nuevo(10.0));

        let fecha = Fecha::new(1, 1, 2024);
        let monto_cripto = 5.0;

        let result = plataforma.vender_cripto(123, monto_cripto, "BTC", fecha.clone());
        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, monto_cripto);
        assert_eq!(transaccion.usuario, 123);
        assert_eq!(transaccion.fecha, fecha);
        assert_eq!(transaccion.cripto, "BTC".to_string());
        assert_eq!(transaccion.tipo_transaccion, TipoTransaccion::VentaCripto);

        let usuario_actualizado = plataforma.usuarios.get(&123).unwrap();
        assert_eq!(usuario_actualizado.balance.f64(), 1275.0); // 1000 + (5 * 55)
        assert_eq!(usuario_actualizado.balance_cripto.get("BTC").unwrap().f64(), 5.0); // 10 - 5
    }

    #[test]
    fn test_vender_cripto_usuario_no_encontrado() {
        let mut plataforma = XYZ::new();
        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.vender_cripto(999, 5.0, "BTC", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_vender_cripto_usuario_no_verificado() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 456,
            verificado: false,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.vender_cripto(456, 5.0, "BTC", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_vender_cripto_balance_insuficiente() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 789,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        plataforma.usuarios.get_mut(&789).unwrap().balance_cripto.insert("BTC".to_string(), Balance::nuevo(2.0));

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.vender_cripto(789, 5.0, "BTC", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_vender_cripto_cotizacion_no_encontrada() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 321,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        plataforma.usuarios.get_mut(&321).unwrap().balance_cripto.insert("BTC".to_string(), Balance::nuevo(10.0));

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.vender_cripto(321, 5.0, "ETH", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_retirar_cripto_a_blockchain_ok() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 123,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        let blockchain = BlockChain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };
        plataforma.blockchains.insert("ETH".to_string(), blockchain);

        plataforma.usuarios.get_mut(&123).unwrap().balance_cripto.insert("BTC".to_string(), Balance::nuevo(10.0));

        let fecha = Fecha::new(1, 1, 2024);
        let monto_cripto = 5.0;

        let result = plataforma.retirar_cripto_a_blockchain(123, monto_cripto, "BTC", "ETH", fecha.clone());
        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, monto_cripto);
        assert_eq!(transaccion.usuario, 123);
        assert_eq!(transaccion.fecha, fecha);
        assert_eq!(transaccion.cadena, "ETH".to_string());
        assert_eq!(transaccion.cripto, "BTC".to_string());
        assert!(transaccion.hasheo.0.starts_with("ETH-"));

        let usuario_actualizado = plataforma.usuarios.get(&123).unwrap();
        assert_eq!(usuario_actualizado.balance_cripto.get("BTC").unwrap().f64(), 5.0); // 10 - 5
    }

    #[test]
    fn test_retirar_cripto_a_blockchain_usuario_no_encontrado() {
        let mut plataforma = XYZ::new();
        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_cripto_a_blockchain(999, 5.0, "BTC", "ETH", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_retirar_cripto_a_blockchain_usuario_no_verificado() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 456,
            verificado: false,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_cripto_a_blockchain(456, 5.0, "BTC", "ETH", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_retirar_cripto_a_blockchain_balance_insuficiente() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 789,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        let blockchain = BlockChain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };
        plataforma.blockchains.insert("ETH".to_string(), blockchain);

        plataforma.usuarios.get_mut(&789).unwrap().balance_cripto.insert("BTC".to_string(), Balance::nuevo(2.0));

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_cripto_a_blockchain(789, 5.0, "BTC", "ETH", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_retirar_cripto_a_blockchain_blockchain_no_encontrada() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 321,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        plataforma.usuarios.get_mut(&321).unwrap().balance_cripto.insert("BTC".to_string(), Balance::nuevo(10.0));

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_cripto_a_blockchain(321, 5.0, "BTC", "NON_EXISTENT_CHAIN", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_recibir_cripto_de_blockchain_ok() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 123,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        let blockchain = BlockChain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };
        plataforma.blockchains.insert("ETH".to_string(), blockchain);

        let fecha = Fecha::new(1, 1, 2024);
        let monto_cripto = 5.0;

        let result = plataforma.recibir_cripto_de_blockchain(123, monto_cripto, "BTC", "ETH", fecha.clone());
        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, monto_cripto);
        assert_eq!(transaccion.usuario, 123);
        assert_eq!(transaccion.fecha, fecha);
        assert_eq!(transaccion.cadena, "ETH".to_string());
        assert_eq!(transaccion.cripto, "BTC".to_string());
        assert!(transaccion.hasheo.0.starts_with("ETH-"));

        let usuario_actualizado = plataforma.usuarios.get(&123).unwrap();
        assert_eq!(usuario_actualizado.balance_cripto.get("BTC").unwrap().f64(), 5.0);
    }

    #[test]
    fn test_recibir_cripto_de_blockchain_usuario_no_encontrado() {
        let mut plataforma = XYZ::new();
        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.recibir_cripto_de_blockchain(999, 5.0, "BTC", "ETH", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_recibir_cripto_de_blockchain_usuario_no_verificado() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 456,
            verificado: false,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.recibir_cripto_de_blockchain(456, 5.0, "BTC", "ETH", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_recibir_cripto_de_blockchain_blockchain_no_encontrada() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 321,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let cotizacion = Cotizacion {
            compra: 50.0,
            venta: 55.0,
        };
        plataforma.cotizaciones.insert("BTC".to_string(), cotizacion);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.recibir_cripto_de_blockchain(321, 5.0, "BTC", "NON_EXISTENT_CHAIN", fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_recibir_cripto_de_blockchain_cotizacion_no_encontrada() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 789,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let blockchain = BlockChain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };
        plataforma.blockchains.insert("ETH".to_string(), blockchain);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.recibir_cripto_de_blockchain(789, 5.0, "ETH", "ETH", fecha);
        assert!(result.is_err());
    }
    #[test]
    fn test_retirar_fiat_ok() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 123,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let monto = 500.0;
        let medio = MedioRetiro::Transferencia;

        let result = plataforma.retirar_fiat(123, monto, medio, fecha.clone());
        assert!(result.is_ok());

        let transaccion = result.unwrap();
        assert_eq!(transaccion.monto, monto);
        assert_eq!(transaccion.usuario, 123);
        assert_eq!(transaccion.fecha, fecha);
        assert_eq!(transaccion.tipo_transaccion, TipoTransaccion::RetiroFiat { medio });

        let usuario_actualizado = plataforma.usuarios.get(&123).unwrap();
        assert_eq!(usuario_actualizado.balance.f64(), 500.0); // 1000 - 500
    }

    #[test]
    fn test_retirar_fiat_usuario_no_encontrado() {
        let mut plataforma = XYZ::new();
        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_fiat(999, 500.0, MedioRetiro::Transferencia, fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_retirar_fiat_usuario_no_verificado() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 456,
            verificado: false,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_fiat(456, 500.0, MedioRetiro::MercadoPago, fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_retirar_fiat_balance_insuficiente() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 789,
            verificado: true,
            balance: Balance::nuevo(200.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_fiat(789, 500.0, MedioRetiro::Transferencia, fecha);
        assert!(result.is_err());
    }

    #[test]
    fn test_retirar_fiat_monto_invalido() {
        let mut plataforma = XYZ::new();
        let usuario = Usuario {
            nombre: "Valentino".to_string(),
            apellido: "Franco".to_string(),
            dni: 321,
            verificado: true,
            balance: Balance::nuevo(1000.0),
            balance_cripto: HashMap::new(),
        };
        plataforma.usuarios.insert(usuario.dni, usuario);

        let fecha = Fecha::new(1, 1, 2024);
        let result = plataforma.retirar_fiat(321, -100.0, MedioRetiro::MercadoPago, fecha);
        assert!(result.is_err());
    }

    fn setup_plataforma_con_transacciones() -> XYZ {
        let mut plataforma = XYZ::new();
        // Add some transactions
        plataforma.transacciones_cripto.push(TransaccionCripto {
            fecha: Fecha::new(1, 1, 2024),
            monto: 10.0,
            usuario: 1,
            cripto: "BTC".to_string(),
            tipo_transaccion: TipoTransaccion::CompraCripto,
        });
        plataforma.transacciones_cripto.push(TransaccionCripto {
            fecha: Fecha::new(2, 1, 2024),
            monto: 5.0,
            usuario: 2,
            cripto: "ETH".to_string(),
            tipo_transaccion: TipoTransaccion::CompraCripto,
        });
        plataforma.transacciones_cripto.push(TransaccionCripto {
            fecha: Fecha::new(3, 1, 2024),
            monto: 7.0,
            usuario: 1,
            cripto: "BTC".to_string(),
            tipo_transaccion: TipoTransaccion::VentaCripto,
        });
        plataforma.transacciones_cripto.push(TransaccionCripto {
            fecha: Fecha::new(4, 1, 2024),
            monto: 3.0,
            usuario: 2,
            cripto: "ETH".to_string(),
            tipo_transaccion: TipoTransaccion::VentaCripto,
        });
        plataforma.transacciones_cripto.push(TransaccionCripto {
            fecha: Fecha::new(5, 1, 2024),
            monto: 2.0,
            usuario: 1,
            cripto: "BTC".to_string(),
            tipo_transaccion: TipoTransaccion::CompraCripto,
        });
        plataforma
    }
    

    #[test]
    fn test_cripto_mas_cantidad_compras() {
        let plataforma = setup_plataforma_con_transacciones();
        let result = plataforma.cripto_mas_cantidad_compras();
        assert_eq!(result, Some("BTC".to_string())); // BTC: 2 compras, ETH: 1 compra
    }

    #[test]
    fn test_cripto_mas_volumen_ventas() {
        let plataforma = setup_plataforma_con_transacciones();
        let result = plataforma.cripto_mas_volumen_ventas();
        assert_eq!(result, Some("BTC".to_string())); // BTC: 7.0, ETH: 3.0
    }

    #[test]
    fn test_cripto_mas_volumen_compras() {
        let plataforma = setup_plataforma_con_transacciones();
        let result = plataforma.cripto_mas_volumen_compras();
        assert_eq!(result, Some("BTC".to_string())); // BTC: 12.0, ETH: 5.0
    }

    #[test]
    fn test_estadisticas_sin_transacciones() {
        let plataforma = XYZ::new();
        assert_eq!(plataforma.cripto_mas_cantidad_ventas(), None);
        assert_eq!(plataforma.cripto_mas_cantidad_compras(), None);
        assert_eq!(plataforma.cripto_mas_volumen_ventas(), None);
        assert_eq!(plataforma.cripto_mas_volumen_compras(), None);
    }

}


fn main() {}