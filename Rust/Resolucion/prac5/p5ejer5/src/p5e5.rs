//
// fecha.rs
//

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

const NOMBRE_MESES: [&str; 12]=["Enero", "Febrero", "Marzo",
    "Abril", "Mayo", "Junio","Julio","Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Clone, Debug, Default, PartialOrd ,PartialEq,Serialize,Deserialize)]
pub struct Fecha{
    pub dia:u8, pub mes:u8, pub anio:u16
}

fn main() {

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

        if!(1..=12).contains(&self.mes) {return false;}

        if self.dia < 1 || self.dia > self.dias_mes_actual(){return false};

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

    pub fn es_mayor(&self, otra: Fecha) -> bool {
        if self.anio > otra.anio ||
            (self.anio == otra.anio && self.mes > otra.mes) ||
            (self.anio == otra.anio && self.mes == otra.mes && self.dia > otra.dia) {
            return true;
        }
        false
    }

    pub fn dias_mes_actual(&self) -> u8 {
        match self.mes  {
            4| 6 | 9| 11 => 30, // si es alguno de estos meses tiene 30 dias
            2=> if self.es_bisiesto() {29} else {28},
            _ => 31,
        }
    }

    pub fn es_bisiesto(&self)-> bool{
        self.anio % 4 == 0}
}

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
    fn test_es_mayor_true() {
        let f1 = Fecha::new(10, 5, 2024);
        let f2 = Fecha::new(5, 5, 2024);
        assert!(f1.es_mayor(f2));
    }

    #[test]
    fn test_es_mayor_false() {
        let f1 = Fecha::new(5, 5, 2024);
        let f2 = Fecha::new(10, 5, 2024);
        assert!(!f1.es_mayor(f2));
    }
}

//
// usuario.rs
//


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

//
// suscripciones.rs
//


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
//
// straming_rust.rs
//

const BASE_FOLDER: &str ="D:/Universidad/Rust/Resolucion/prac5/p5ejer5/";



struct StreamingRust {
    path: String,
    usuarios: BTreeMap<u64, Usuario>
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorMayorMedioPago{
    SinSuscripciones
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorSuscripciones{
    SinSuscripciones
}



//
//Errores
//
impl StreamingRust {

    fn reescribir_archivo_json(&self) -> bool {

        let mut file = match File::create(format!("{}biblioteca.json", self.path)) {
            Ok(res) => res,
            Err(_) => return false,
        };

        let json_data = match serde_json::to_string_pretty(&self.usuarios) {
            Ok(res) => res,
            Err(_) => return false,
        };

        file.write_all(json_data.as_bytes()).is_ok()
    }

    fn new(usuarios: Option<BTreeMap<u64, Usuario>>) -> StreamingRust {

        let plataforma = StreamingRust {
            path: BASE_FOLDER.to_string(),
            usuarios: usuarios.unwrap_or_default(),
        };

        plataforma.reescribir_archivo_json();

        plataforma
    }

    //➢ Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas
    pub fn mayor_medio_pago(&self) -> Result<MediosPago, ErrorMayorMedioPago> {
        let mut contador: HashMap<MediosPago, usize> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {
            if usuario.suscripcion != Suscripciones::Cancelada {
                *contador.entry(usuario.datos.medio_pago.clone()).or_insert(0) += 1;
            }
        });
        contador.into_iter().max_by_key(|(_, count)| *count).map(|(medio, _)| medio).ok_or(ErrorMayorMedioPago::SinSuscripciones)
    }

    //➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones activas.
    pub fn suscripcion_mas_contratada(&self) -> Result<Suscripciones, ErrorSuscripciones> {
        let mut contador: HashMap<Suscripciones, usize> = HashMap::new();
        for (_, usuario) in self.usuarios.iter().filter(|(_, u)| u.suscripcion != Suscripciones::Cancelada) {
            *contador.entry(usuario.suscripcion).or_insert(0) += 1;
        }
        if contador.is_empty() {
            return Err(ErrorSuscripciones::SinSuscripciones);
        }
        let (mas_usada, _) = contador.into_iter().max_by_key(|(_, count)| *count).unwrap();
        Ok(mas_usada)
    }

    //➢ Saber cuál fue el medio de pago más utilizado.
    pub fn medio_pago_mas_usado_con_cancelados(&self) -> Result<MediosPago, ErrorMayorMedioPago> {
        let mut contador: HashMap<MediosPago, usize> = HashMap::new();

        for usuario in self.usuarios.values() {
            let tipo = usuario.datos.medio_pago.clone(); // 👈 CLONÁS el valor
            *contador.entry(tipo).or_insert(0) += 1;
        }

        if contador.is_empty() {
            return Err(ErrorMayorMedioPago::SinSuscripciones);
        }

        let (mas_usado, _) = contador
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap();

        Ok(mas_usado)
    }
    fn suscripcion_mas_contratada_con_cancelados(&self) -> Result<Suscripciones, ErrorSuscripciones> {
        let mut contador: HashMap<Suscripciones, usize> = HashMap::new();
        for (_, usuario) in self.usuarios.iter() {
            let tipo = usuario.suscripcion  ;
            *contador.entry(tipo).or_insert(0) += 1;
        }
        if contador.is_empty() {
            return Err(ErrorSuscripciones::SinSuscripciones);
        }
        let (mas_usado, _) = contador.into_iter().max_by_key(|&(_, count)| count).unwrap();
        Ok(mas_usado)
    }

    //➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.

    pub fn upgrade(&mut self, id: u64) -> Result<bool, ErrorUpgrade> {

        if let Some(usuario) = self.usuarios.get_mut(&id) {
            match usuario.suscripcion {
                Suscripciones::Basic => {
                    usuario.suscripcion = Suscripciones::Classic;
                    self.reescribir_archivo_json();
                    return Ok(true)
                },
                Suscripciones::Classic => {
                    usuario.suscripcion = Suscripciones::Super;
                    self.reescribir_archivo_json();
                    return Ok(true)
                },
                Suscripciones::Super => { return Err(ErrorUpgrade::SuscripcionMasAltaPosible); },

                _ => { return Err(ErrorUpgrade::NoCuentaConSuscripcion); }
            }
            Ok(true)
        } else {
            Err(ErrorUpgrade::NoCuentaConSuscripcion)
        }
    }

    pub fn downgrade(&mut self, id: u64) -> Result<bool, ErrorDowngrade> {
        if let Some(usuario) = self.usuarios.get_mut(&id) {
            match usuario.suscripcion {
                Suscripciones::Super => {
                    usuario.suscripcion = Suscripciones::Classic;
                }
                Suscripciones::Classic => {
                    usuario.suscripcion = Suscripciones::Basic;
                }
                Suscripciones::Basic => {
                    usuario.suscripcion = Suscripciones::Cancelada;
                    self.reescribir_archivo_json();
                    return Ok(true);
                }
                Suscripciones::Cancelada => {
                    return Err(ErrorDowngrade::NoCuentaConSuscripcion);
                }
            }
            self.reescribir_archivo_json();
            Ok(true)
        } else {
            Err(ErrorDowngrade::NoCuentaConSuscripcion)
        }
    }

    pub fn cancelar_suscripcion(&mut self, id: u64) -> Result<bool, ErrorCancelar> {
        if let Some(usuario) = self.usuarios.get_mut(&id) {
            match usuario.suscripcion {
                Suscripciones::Cancelada => {
                    Err(ErrorCancelar::SuscripcionYaCancelada)
                }
                _ => {
                    usuario.suscripcion = Suscripciones::Cancelada;
                    self.reescribir_archivo_json();
                    Ok(true)
                }
            }
        } else {
            Err(ErrorCancelar::NoCuentaConSuscripcion)
        }
    }



}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;
    fn crear_plataforma()-> StreamingRust{
        StreamingRust{
            path : BASE_FOLDER.to_string(),
            usuarios: BTreeMap::new(),
        }
    }
    fn crear_usuario(id_usuario: u8, suscripcion: Suscripciones, datos:Suscripcion ) -> Usuario {
        Usuario {
            id_usuario,
            suscripcion,
            datos,
        }
    }
    #[test]
    fn test_upgrade_downgrade_cancelar_usuario() {

        let mut nuevo_user = Usuario::new(123, Suscripciones::Classic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user.downgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user.suscripcion, Suscripciones::Basic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }

        let mut nuevo_user6 = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user6.downgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user6.suscripcion, Suscripciones::Classic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }
        //upgrade:
        let mut nuevo_user2 = Usuario::new(123, Suscripciones::Basic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user2.upgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user2.suscripcion, Suscripciones::Classic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }

        //upgrade:
        let mut nuevo_user2 = Usuario::new(123, Suscripciones::Classic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user2.upgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user2.suscripcion, Suscripciones::Super);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }

        let mut nuevo_user7  = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user7.downgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user7.suscripcion, Suscripciones::Classic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }
        //cancelar:
        let mut nuevo_user3 = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user3.cancelar_suscription();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user3.suscripcion, Suscripciones::Cancelada);
            }
            Err(e)=>{
                panic!("no deberia dar el error  {:?}", e);
            }
        }
        //cancelar:
        let mut nuevo_user3 = Usuario::new(123, Suscripciones::Classic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user3.cancelar_suscription();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user3.suscripcion, Suscripciones::Cancelada);
            }
            Err(e)=>{
                panic!("no deberia dar el error  {:?}", e);
            }
        }
        //cancelar:
        let mut nuevo_user3 = Usuario::new(123, Suscripciones::Basic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user3.cancelar_suscription();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user3.suscripcion, Suscripciones::Cancelada);
            }
            Err(e)=>{
                panic!("no deberia dar el error  {:?}", e);
            }
        }

        //Test Errores:

        let mut nuevo_user_error1 = Usuario::new(123, Suscripciones::Basic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error1.downgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorDowngrade::SuscripcionMasBajaPosible)
            }
        }
        let mut nuevo_user_error2 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error2.downgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorDowngrade::NoCuentaConSuscripcion)
            }
        }
        let mut nuevo_user_error1 = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error1.upgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorUpgrade::SuscripcionMasAltaPosible)
            }
        }
        let mut nuevo_user_error2 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error2.upgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorUpgrade::NoCuentaConSuscripcion)
            }
        }
        let mut nuevo_user_error3 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error3.cancelar_suscription();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorCancelar::SuscripcionYaCancelada)
            }
        }
        let mut nuevo_user_error3 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error3.cancelar_suscription();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorCancelar::SuscripcionYaCancelada)
            }
        }
    }
    #[test]
    fn test_mayor_medio_pago() {
        let mut usuarios = BTreeMap::new();

        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Classic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Basic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Tarjeta(5585),
                ),
            ),
        );

        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.mayor_medio_pago();
        assert_eq!(resultado, Ok(MediosPago::Efectivo));

        let plataforma = StreamingRust::new(None);
        assert!(
            matches!(plataforma.mayor_medio_pago(),
                     Err(ErrorMayorMedioPago::SinSuscripciones))
        );

        let mut usuarios = BTreeMap::new();
        usuarios.insert(1, Usuario::new(1, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(1, 1, 2025), Fecha::new(1, 2, 2025), MediosPago::Efectivo)));
        usuarios.insert(2, Usuario::new(2, Suscripciones::Cancelada, Suscripcion::new(10.0,Fecha::new(1, 1, 2025), Fecha::new(1, 2, 2025), MediosPago::Tarjeta(595959))));

        let plataforma = StreamingRust::new(Some(usuarios));
        assert!(
            matches!(plataforma.mayor_medio_pago(),
                     Err(ErrorMayorMedioPago::SinSuscripciones))
        );


    }
    #[test]
    fn test_suscripcion_mas_contratada() {

        let mut usuarios = BTreeMap::new();
        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Basic,
                Suscripcion::new(
                    5.0,
                    Fecha::new(1, 4, 2025),
                    Fecha::new(1, 5, 2025),
                    MediosPago::Tarjeta(23232),
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Classic,
                Suscripcion::new(
                    8.0,
                    Fecha::new(2, 4, 2025),
                    Fecha::new(2, 5, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Basic,
                Suscripcion::new(
                    5.0,
                    Fecha::new(3, 4, 2025),
                    Fecha::new(3, 5, 2025),
                    MediosPago::Tarjeta(343242),
                ),
            ),
        );
        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.suscripcion_mas_contratada();
        match resultado {
            Ok(r)=>{
                assert_eq!(r, Suscripciones::Basic);
            }
            Err(e)=>{
                panic!("no deberia dar error {:?}", e);
            }
        }


        //error

        let mut usuarios = BTreeMap::new();
        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    0.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(2, 1, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        let plataforma = StreamingRust::new(Some(usuarios));
        let result  = plataforma.suscripcion_mas_contratada();
        match  result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorSuscripciones::SinSuscripciones);
            }
        }
    }
    #[test]
    fn test_mayor_medio_pago_con_cancelados(){

        let mut usuarios = BTreeMap::new();

        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Basic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Tarjeta(5585),
                ),
            ),
        );

        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.medio_pago_mas_usado_con_cancelados();
        assert_eq!(resultado, Ok(MediosPago::Efectivo));

        //error

        let plataforma = StreamingRust::new(None);
        assert!(
            matches!(plataforma.medio_pago_mas_usado_con_cancelados(),
                     Err(ErrorMayorMedioPago::SinSuscripciones))
        );

    }
    #[test]
    fn test_suscripcion_mas_contratada_con_cancelados() {
        let mut usuarios = BTreeMap::new();

        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Classic,
                Suscripcion::new(
                    12.0,
                    Fecha::new(10, 3, 2025),
                    Fecha::new(10, 4, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Basic,
                Suscripcion::new(
                    8.0,
                    Fecha::new(12, 3, 2025),
                    Fecha::new(12, 4, 2025),
                    MediosPago::Tarjeta(12345),
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Classic,
                Suscripcion::new(
                    12.0,
                    Fecha::new(15, 3, 2025),
                    Fecha::new(15, 4, 2025),
                    MediosPago::Tarjeta(67890),
                ),
            ),
        );

        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.suscripcion_mas_contratada_con_cancelados();
        assert_eq!(resultado, Ok(Suscripciones::Classic));

        let plataforma = StreamingRust::new(None);
        let result = plataforma.suscripcion_mas_contratada_con_cancelados();
        assert_eq!(result, Err(ErrorSuscripciones::SinSuscripciones))

    }
    #[test]
    fn test_error_reescribir_arch(){
        let p = StreamingRust{
            path : "///ruta/invalida/".to_string(),
            usuarios: BTreeMap::new(),
        };
        let result =  p.reescribir_archivo_json();
        assert!(!result)
    }
    #[test]
    fn test_upgrade_downgrade_desde_plataforma() {
        let mut usuarios = BTreeMap::new();

        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Basic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );

        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Super,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Tarjeta(123456),
                ),
            ),
        );

        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Classic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Transferencia(789456),
                ),
            ),
        );

        let mut plataforma = StreamingRust::new(Some(usuarios));

        // Upgrade usuario 1 (de Basic a Classic)
        let result = plataforma.upgrade(1);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&1).unwrap().suscripcion, Suscripciones::Classic);

        // Upgrade usuario 3 (de Classic a Super)
        let result = plataforma.upgrade(3);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&3).unwrap().suscripcion, Suscripciones::Super);

        // Upgrade usuario 2 (ya está en Super, debe fallar)
        let result = plataforma.upgrade(2);
        assert_eq!(result, Err(ErrorUpgrade::SuscripcionMasAltaPosible));

        // Downgrade usuario 3 (de Super a Classic)
        let result = plataforma.downgrade(3);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&3).unwrap().suscripcion, Suscripciones::Classic);

        // Downgrade usuario 1 (de Classic a Basic)
        let result = plataforma.downgrade(1);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&1).unwrap().suscripcion, Suscripciones::Basic);

        // Downgrade usuario 1 nuevamente (de Basic a Cancelada)
        let result = plataforma.downgrade(1);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&1).unwrap().suscripcion, Suscripciones::Cancelada);

        // Downgrade usuario 1 otra vez (ya Cancelada, debe fallar)
        let result = plataforma.downgrade(1);
        assert_eq!(result, Err(ErrorDowngrade::NoCuentaConSuscripcion));

        // Cancelar usuario 2 (Super → Cancelada)
        let result = plataforma.cancelar_suscripcion(2);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&2).unwrap().suscripcion, Suscripciones::Cancelada);

        // Cancelar usuario 2 otra vez (ya Cancelada, debe fallar)
        let result = plataforma.cancelar_suscripcion(2);
        assert_eq!(result, Err(ErrorCancelar::SuscripcionYaCancelada));

        // Usuario inexistente
        let result = plataforma.upgrade(999);
        assert_eq!(result, Err(ErrorUpgrade::NoCuentaConSuscripcion));
    }


}




